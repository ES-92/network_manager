import { create } from "zustand";
import type { Service } from "../lib/tauri/types";
import * as api from "../lib/tauri/commands";

interface ServiceState {
  services: Service[];
  selectedService: Service | null;
  isLoading: boolean;
  error: string | null;
  autoRefreshInterval: number | null;
  refreshIntervalMs: number;

  // Actions
  fetchServices: () => Promise<void>;
  selectService: (service: Service | null) => void;
  startService: (serviceId: string) => Promise<void>;
  stopService: (serviceId: string) => Promise<void>;
  restartService: (serviceId: string) => Promise<void>;
  killService: (serviceId: string) => Promise<void>;
  toggleAutostart: (serviceId: string, enable: boolean) => Promise<void>;
  startAutoRefresh: (intervalMs?: number) => void;
  stopAutoRefresh: () => void;
  setRefreshInterval: (intervalMs: number) => void;
}

export const useServiceStore = create<ServiceState>((set, get) => ({
  services: [],
  selectedService: null,
  isLoading: false,
  error: null,
  autoRefreshInterval: null,
  refreshIntervalMs: 3000, // Default 3 seconds

  fetchServices: async () => {
    // Don't set loading state during auto-refresh to avoid flickering
    const isAutoRefresh = get().autoRefreshInterval !== null;
    if (!isAutoRefresh) {
      set({ isLoading: true, error: null });
    }
    try {
      const services = await api.discoverServices();
      set({ services, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  startAutoRefresh: (intervalMs) => {
    const state = get();
    // Clear existing interval if any
    if (state.autoRefreshInterval !== null) {
      clearInterval(state.autoRefreshInterval);
    }
    const ms = intervalMs ?? state.refreshIntervalMs;
    const interval = window.setInterval(() => {
      get().fetchServices();
    }, ms);
    set({ autoRefreshInterval: interval, refreshIntervalMs: ms });
  },

  stopAutoRefresh: () => {
    const state = get();
    if (state.autoRefreshInterval !== null) {
      clearInterval(state.autoRefreshInterval);
      set({ autoRefreshInterval: null });
    }
  },

  setRefreshInterval: (intervalMs) => {
    const state = get();
    set({ refreshIntervalMs: intervalMs });
    // Restart auto-refresh if it's running
    if (state.autoRefreshInterval !== null) {
      state.startAutoRefresh(intervalMs);
    }
  },

  selectService: (service) => {
    set({ selectedService: service });
  },

  startService: async (serviceId) => {
    try {
      await api.startService(serviceId);
      // Refresh services after action
      await get().fetchServices();
    } catch (error) {
      set({ error: String(error) });
    }
  },

  stopService: async (serviceId) => {
    try {
      await api.stopService(serviceId);
      await get().fetchServices();
    } catch (error) {
      set({ error: String(error) });
    }
  },

  restartService: async (serviceId) => {
    try {
      await api.restartService(serviceId);
      await get().fetchServices();
    } catch (error) {
      set({ error: String(error) });
    }
  },

  killService: async (serviceId) => {
    try {
      const service = get().services.find(s => s.id === serviceId);
      if (service?.pid) {
        await api.killProcess(service.pid);
        await get().fetchServices();
      }
    } catch (error) {
      set({ error: String(error) });
    }
  },

  toggleAutostart: async (serviceId: string, enable: boolean) => {
    const service = get().services.find(s => s.id === serviceId);
    if (!service) {
      set({ error: "Service nicht gefunden" });
      return;
    }

    try {
      if (enable) {
        await api.enableServiceAutostart(serviceId, service.service_type);
      } else {
        await api.disableServiceAutostart(serviceId, service.service_type);
      }
      // Refresh services to get updated autostart status
      await get().fetchServices();
    } catch (error) {
      set({ error: String(error) });
      throw error;
    }
  },
}));

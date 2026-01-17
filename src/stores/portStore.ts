import { create } from "zustand";
import type { PortInfo } from "../lib/tauri/types";
import * as api from "../lib/tauri/commands";

interface PortState {
  ports: PortInfo[];
  isLoading: boolean;
  error: string | null;

  // Actions
  fetchPortUsage: () => Promise<void>;
  scanPorts: (start: number, end: number) => Promise<void>;
  findFreePorts: (count: number) => Promise<number[]>;
}

export const usePortStore = create<PortState>((set) => ({
  ports: [],
  isLoading: false,
  error: null,

  fetchPortUsage: async () => {
    set({ isLoading: true, error: null });
    try {
      const ports = await api.getPortUsage();
      set({ ports, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  scanPorts: async (start, end) => {
    set({ isLoading: true, error: null });
    try {
      const ports = await api.scanPorts(start, end);
      set({ ports, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  findFreePorts: async (count) => {
    try {
      return await api.findFreePorts(count);
    } catch (error) {
      set({ error: String(error) });
      return [];
    }
  },
}));

import { useEffect, useCallback } from "react";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useServiceStore } from "../../stores/serviceStore";
import { usePortStore } from "../../stores/portStore";
import type { Service } from "../tauri/types";

// Event types matching the Rust enum
interface ServicesDiscoveredEvent {
  type: "ServicesDiscovered";
  payload: Service[];
}

interface ServiceStatusChangedEvent {
  type: "ServiceStatusChanged";
  payload: {
    service_id: string;
    old_status: string;
    new_status: string;
  };
}

interface ServiceAddedEvent {
  type: "ServiceAdded";
  payload: Service;
}

interface ServiceRemovedEvent {
  type: "ServiceRemoved";
  payload: {
    service_id: string;
  };
}

interface ServicePortsChangedEvent {
  type: "ServicePortsChanged";
  payload: {
    service_id: string;
    ports: number[];
  };
}

type ServiceEvent =
  | ServicesDiscoveredEvent
  | ServiceStatusChangedEvent
  | ServiceAddedEvent
  | ServiceRemovedEvent
  | ServicePortsChangedEvent;

/**
 * Hook that subscribes to real-time service events from the backend
 * and automatically updates the Zustand stores.
 */
export function useRealtime() {
  const portStore = usePortStore();

  const handleServiceEvent = useCallback(
    (event: ServiceEvent) => {
      switch (event.type) {
        case "ServicesDiscovered":
          // Initial full list of services
          useServiceStore.setState({ services: event.payload, isLoading: false });
          break;

        case "ServiceStatusChanged":
          // Update just the status of a specific service
          useServiceStore.setState((state) => ({
            services: state.services.map((service) =>
              service.id === event.payload.service_id
                ? { ...service, status: event.payload.new_status as Service["status"] }
                : service
            ),
          }));
          break;

        case "ServiceAdded":
          // Add new service to the list
          useServiceStore.setState((state) => ({
            services: [...state.services, event.payload],
          }));
          break;

        case "ServiceRemoved":
          // Remove service from the list
          useServiceStore.setState((state) => ({
            services: state.services.filter(
              (service) => service.id !== event.payload.service_id
            ),
            // Clear selection if the removed service was selected
            selectedService:
              state.selectedService?.id === event.payload.service_id
                ? null
                : state.selectedService,
          }));
          break;

        case "ServicePortsChanged":
          // Update ports for a specific service
          useServiceStore.setState((state) => ({
            services: state.services.map((service) =>
              service.id === event.payload.service_id
                ? { ...service, ports: event.payload.ports }
                : service
            ),
          }));
          // Also refresh port usage when ports change
          portStore.fetchPortUsage();
          break;
      }
    },
    [portStore]
  );

  useEffect(() => {
    let unlisten: UnlistenFn | null = null;

    const setupListener = async () => {
      try {
        unlisten = await listen<ServiceEvent>("service-event", (event) => {
          handleServiceEvent(event.payload);
        });
      } catch (error) {
        console.error("Failed to set up real-time listener:", error);
      }
    };

    setupListener();

    // Cleanup on unmount
    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [handleServiceEvent]);

  // Return nothing - this hook is purely for side effects
  return null;
}

/**
 * Hook that provides the current connection status
 * for the real-time updates.
 */
export function useRealtimeStatus() {
  // Could be extended to track connection health
  return {
    isConnected: true, // Tauri events are always available when app is running
  };
}

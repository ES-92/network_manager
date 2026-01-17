import { useEffect } from "react";
import { useServiceStore } from "../../stores/serviceStore";

/**
 * Hook for accessing and managing services.
 * Automatically fetches services on mount if they haven't been loaded yet.
 */
export function useServices() {
  const store = useServiceStore();
  const {
    services,
    selectedService,
    isLoading,
    error,
    fetchServices,
    selectService,
    startService,
    stopService,
    restartService,
    killService,
  } = store;

  useEffect(() => {
    // Fetch services on mount if not already loaded
    if (services.length === 0 && !isLoading) {
      fetchServices();
    }
  }, [services.length, isLoading, fetchServices]);

  return {
    services,
    selectedService,
    isLoading,
    error,
    refresh: fetchServices,
    selectService,
    startService,
    stopService,
    restartService,
    killService,
  };
}

/**
 * Filter services by status
 */
export function useServicesByStatus(status: "running" | "stopped" | "error" | "unknown") {
  const { services } = useServices();
  return services.filter((s) => s.status === status);
}

/**
 * Get running services count
 */
export function useRunningServicesCount() {
  const { services } = useServices();
  return services.filter((s) => s.status === "running").length;
}

/**
 * Get services grouped by type
 */
export function useServicesGroupedByType() {
  const { services } = useServices();
  return services.reduce((groups, service) => {
    const type = service.service_type;
    if (!groups[type]) {
      groups[type] = [];
    }
    groups[type].push(service);
    return groups;
  }, {} as Record<string, typeof services>);
}

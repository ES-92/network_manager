import { useEffect } from "react";
import { usePortStore } from "../../stores/portStore";

/**
 * Hook for accessing and managing port information.
 * Automatically fetches port usage on mount if not already loaded.
 */
export function usePorts() {
  const store = usePortStore();
  const { ports, isLoading, fetchPortUsage, scanPorts, findFreePorts, error } = store;

  useEffect(() => {
    // Fetch port usage on mount if not already loaded
    if (ports.length === 0 && !isLoading) {
      fetchPortUsage();
    }
  }, [ports.length, isLoading, fetchPortUsage]);

  return {
    ports,
    isLoading,
    error,
    refresh: fetchPortUsage,
    scanPorts,
    findFreePorts,
  };
}

/**
 * Get ports grouped by protocol
 */
export function usePortsByProtocol() {
  const { ports } = usePorts();
  return {
    tcp: ports.filter((p) => p.protocol === "tcp"),
    udp: ports.filter((p) => p.protocol === "udp"),
  };
}

/**
 * Check if a specific port is in use
 */
export function useIsPortInUse(port: number) {
  const { ports } = usePorts();
  return ports.some((p) => p.port === port);
}

/**
 * Get commonly used ports (well-known ports < 1024)
 */
export function useWellKnownPorts() {
  const { ports } = usePorts();
  return ports.filter((p) => p.port < 1024);
}

/**
 * Get high ports (> 1024)
 */
export function useHighPorts() {
  const { ports } = usePorts();
  return ports.filter((p) => p.port >= 1024);
}

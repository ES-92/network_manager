import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Badge } from "../components/ui/badge";
import { Switch } from "../components/ui/switch";
import { useServiceStore } from "../stores/serviceStore";
import { Play, Square, RefreshCw, Skull, Pause } from "lucide-react";
import type { ServiceStatus, ServiceType } from "../lib/tauri/types";
import { ServiceInfoButton } from "../components/services/ServiceInfoButton";
import { ResourceStats } from "../components/services/ResourceBar";

const statusColors: Record<ServiceStatus, "success" | "secondary" | "destructive" | "outline"> = {
  running: "success",
  stopped: "secondary",
  error: "destructive",
  unknown: "outline",
};

const typeLabels: Record<ServiceType, string> = {
  docker: "Docker",
  systemd: "Systemd",
  launchd: "Launchd",
  windows_service: "Windows",
  process: "Process",
};

export function Services() {
  const {
    services,
    fetchServices,
    startService,
    stopService,
    restartService,
    killService,
    toggleAutostart,
    isLoading,
    autoRefreshInterval,
    startAutoRefresh,
    stopAutoRefresh,
  } = useServiceStore();

  const [togglingAutostart, setTogglingAutostart] = useState<Set<string>>(new Set());
  const isAutoRefreshing = autoRefreshInterval !== null;

  const handleToggleAutostart = async (serviceId: string, enable: boolean) => {
    const service = services.find(s => s.id === serviceId);
    console.log("Toggle autostart:", { serviceId, enable, service_type: service?.service_type });
    setTogglingAutostart(prev => new Set(prev).add(serviceId));
    try {
      await toggleAutostart(serviceId, enable);
      console.log("Toggle autostart success");
    } catch (error) {
      console.error("Failed to toggle autostart:", error);
    } finally {
      setTogglingAutostart(prev => {
        const next = new Set(prev);
        next.delete(serviceId);
        return next;
      });
    }
  };

  // Check if autostart is supported for a service type
  const supportsAutostart = (serviceType: string): boolean => {
    return ["launchd", "systemd", "windows_service", "docker"].includes(serviceType);
  };

  useEffect(() => {
    fetchServices();
    // Start auto-refresh when component mounts (5 seconds for better performance)
    startAutoRefresh(5000);

    // Cleanup on unmount
    return () => {
      stopAutoRefresh();
    };
  }, []);

  const toggleAutoRefresh = () => {
    if (isAutoRefreshing) {
      stopAutoRefresh();
    } else {
      startAutoRefresh(3000);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Services</h1>
          <p className="text-muted-foreground">
            Manage and monitor all services on your system
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant={isAutoRefreshing ? "default" : "outline"}
            onClick={toggleAutoRefresh}
            size="sm"
          >
            {isAutoRefreshing ? (
              <>
                <Pause className="mr-2 h-4 w-4" />
                Live
              </>
            ) : (
              <>
                <Play className="mr-2 h-4 w-4" />
                Live
              </>
            )}
          </Button>
          <Button onClick={fetchServices} disabled={isLoading} variant="outline">
            <RefreshCw className={`mr-2 h-4 w-4 ${isLoading ? "animate-spin" : ""}`} />
            Refresh
          </Button>
        </div>
      </div>

      {/* Services List */}
      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <RefreshCw className="h-8 w-8 animate-spin text-muted-foreground" />
        </div>
      ) : services.length === 0 ? (
        <Card>
          <CardContent className="flex flex-col items-center justify-center py-12">
            <p className="text-muted-foreground">No services found</p>
            <Button variant="outline" className="mt-4" onClick={fetchServices}>
              Try Again
            </Button>
          </CardContent>
        </Card>
      ) : (
        <div className="grid gap-4">
          {services.map((service) => (
            <Card key={service.id}>
              <CardHeader className="pb-2">
                <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
                  <div className="flex items-center gap-2 flex-wrap">
                    <ServiceInfoButton service={service} />
                    <CardTitle className="text-base truncate max-w-[200px] sm:max-w-[300px]">{service.name}</CardTitle>
                    <Badge variant={statusColors[service.status]} className="shrink-0">
                      {service.status}
                    </Badge>
                    <Badge variant="outline" className="shrink-0">{typeLabels[service.service_type]}</Badge>
                  </div>
                  <div className="flex items-center gap-1 flex-wrap">
                    {service.status === "stopped" && (
                      <Button
                        size="sm"
                        variant="outline"
                        onClick={() => startService(service.id)}
                      >
                        <Play className="h-4 w-4" />
                      </Button>
                    )}
                    {service.status === "running" && (
                      <>
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => stopService(service.id)}
                        >
                          <Square className="h-4 w-4" />
                        </Button>
                        <Button
                          size="sm"
                          variant="outline"
                          onClick={() => restartService(service.id)}
                        >
                          <RefreshCw className="h-4 w-4" />
                        </Button>
                      </>
                    )}
                    {service.pid && (
                      <Button
                        size="sm"
                        variant="destructive"
                        onClick={() => killService(service.id)}
                      >
                        <Skull className="h-4 w-4" />
                      </Button>
                    )}
                  </div>
                </div>
              </CardHeader>
              <CardContent>
                {/* Resource Usage */}
                {(service.cpu_usage !== null || service.memory_bytes !== null) && (
                  <div className="mb-4 p-3 rounded-lg bg-muted/50">
                    <ResourceStats
                      cpuUsage={service.cpu_usage}
                      memoryBytes={service.memory_bytes}
                      memoryPercent={service.memory_percent}
                      compact
                    />
                  </div>
                )}

                <div className="flex flex-wrap gap-4 text-sm">
                  <div className="min-w-[80px]">
                    <p className="text-muted-foreground text-xs">Ports</p>
                    <p className="font-mono font-medium text-sm">
                      {service.ports.length > 0 ? service.ports.slice(0, 3).map(p => `:${p}`).join(", ") : "-"}
                      {service.ports.length > 3 && ` +${service.ports.length - 3}`}
                    </p>
                  </div>
                  <div className="min-w-[60px]">
                    <p className="text-muted-foreground text-xs">PID</p>
                    <p className="font-mono font-medium text-sm">
                      {service.pid || "-"}
                    </p>
                  </div>
                  {supportsAutostart(service.service_type) && (
                    <div className="min-w-[80px]">
                      <p className="text-muted-foreground text-xs">Autostart</p>
                      <div className="flex items-center gap-1">
                        <Switch
                          checked={service.auto_start}
                          onCheckedChange={(checked) => handleToggleAutostart(service.id, checked)}
                          disabled={togglingAutostart.has(service.id)}
                        />
                        <span className="text-xs">
                          {togglingAutostart.has(service.id) ? "..." : service.auto_start ? "On" : "Off"}
                        </span>
                      </div>
                    </div>
                  )}
                </div>
                {service.description && (
                  <p className="mt-3 text-sm text-muted-foreground truncate">
                    {service.description}
                  </p>
                )}
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}

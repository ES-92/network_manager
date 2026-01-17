import { useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "../components/ui/card";
import { Badge } from "../components/ui/badge";
import { useServiceStore } from "../stores/serviceStore";
import { usePortStore } from "../stores/portStore";
import { Server, Network, Activity, AlertCircle } from "lucide-react";
import { ServiceInfoButton } from "../components/services/ServiceInfoButton";
import { Recommendations } from "../components/dashboard/Recommendations";
import { ResourceStats } from "../components/services/ResourceBar";

export function Dashboard() {
  const { services, fetchServices, isLoading: servicesLoading } = useServiceStore();
  const { ports, fetchPortUsage, isLoading: portsLoading } = usePortStore();

  useEffect(() => {
    fetchServices();
    fetchPortUsage();
  }, [fetchServices, fetchPortUsage]);

  const runningServices = services.filter((s) => s.status === "running").length;
  const stoppedServices = services.filter((s) => s.status === "stopped").length;
  const errorServices = services.filter((s) => s.status === "error").length;
  const occupiedPorts = ports.filter((p) => p.status === "occupied").length;

  const isLoading = servicesLoading || portsLoading;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
        <p className="text-muted-foreground">
          Overview of your services and network status
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Services</CardTitle>
            <Server className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {isLoading ? "..." : services.length}
            </div>
            <p className="text-xs text-muted-foreground">
              Discovered on this system
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Running</CardTitle>
            <Activity className="h-4 w-4 text-green-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600">
              {isLoading ? "..." : runningServices}
            </div>
            <p className="text-xs text-muted-foreground">
              Active services
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Stopped</CardTitle>
            <AlertCircle className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {isLoading ? "..." : stoppedServices}
            </div>
            <p className="text-xs text-muted-foreground">
              Inactive services
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Ports in Use</CardTitle>
            <Network className="h-4 w-4 text-blue-500" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-blue-600">
              {isLoading ? "..." : occupiedPorts}
            </div>
            <p className="text-xs text-muted-foreground">
              Occupied network ports
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Recent Services */}
      <div className="grid gap-4 md:grid-cols-2">
        {/* Running Services - Top CPU Users */}
        <Card>
          <CardHeader>
            <CardTitle>Top CPU-Verbraucher</CardTitle>
          </CardHeader>
          <CardContent>
            {isLoading ? (
              <p className="text-muted-foreground">Loading...</p>
            ) : services.filter((s) => s.status === "running").length === 0 ? (
              <p className="text-muted-foreground">No running services found</p>
            ) : (
              <div className="space-y-2">
                {services
                  .filter((s) => s.status === "running")
                  .sort((a, b) => (b.cpu_usage ?? 0) - (a.cpu_usage ?? 0))
                  .slice(0, 5)
                  .map((service) => (
                    <div
                      key={service.id}
                      className="rounded-md border p-3 space-y-2"
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-2 min-w-0 flex-1">
                          <ServiceInfoButton service={service} size="sm" />
                          <div className="min-w-0">
                            <p className="font-medium truncate">{service.name}</p>
                            <p className="text-xs text-muted-foreground">
                              {service.service_type}
                              {service.ports.length > 0 && ` - Port ${service.ports.join(", ")}`}
                            </p>
                          </div>
                        </div>
                        <Badge variant="success">Running</Badge>
                      </div>
                      {(service.cpu_usage !== null || service.memory_bytes !== null) && (
                        <ResourceStats
                          cpuUsage={service.cpu_usage}
                          memoryBytes={service.memory_bytes}
                          memoryPercent={service.memory_percent}
                          compact
                        />
                      )}
                    </div>
                  ))}
              </div>
            )}
          </CardContent>
        </Card>

        {/* Port Usage */}
        <Card>
          <CardHeader>
            <CardTitle>Active Ports</CardTitle>
          </CardHeader>
          <CardContent>
            {isLoading ? (
              <p className="text-muted-foreground">Loading...</p>
            ) : ports.length === 0 ? (
              <p className="text-muted-foreground">No active ports found</p>
            ) : (
              <div className="space-y-2">
                {ports.slice(0, 5).map((port) => (
                  <div
                    key={port.port}
                    className="flex items-center justify-between rounded-md border p-3"
                  >
                    <div>
                      <p className="font-mono font-medium">:{port.port}</p>
                      <p className="text-xs text-muted-foreground">
                        {port.process_name || "Unknown process"}
                        {port.pid && ` (PID: ${port.pid})`}
                      </p>
                    </div>
                    <Badge>{port.protocol.toUpperCase()}</Badge>
                  </div>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      </div>

      {/* AI Recommendations */}
      <Recommendations services={services} />

      {/* Errors Section */}
      {errorServices > 0 && (
        <Card className="border-destructive">
          <CardHeader>
            <CardTitle className="text-destructive">Services with Errors</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {services
                .filter((s) => s.status === "error")
                .map((service) => (
                  <div
                    key={service.id}
                    className="flex items-center justify-between rounded-md border border-destructive/50 bg-destructive/10 p-3"
                  >
                    <div>
                      <p className="font-medium">{service.name}</p>
                      <p className="text-xs text-muted-foreground">
                        {service.description || "No description available"}
                      </p>
                    </div>
                    <Badge variant="destructive">Error</Badge>
                  </div>
                ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

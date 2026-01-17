import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Badge } from "../components/ui/badge";
import { usePortStore } from "../stores/portStore";
import { RefreshCw, Search } from "lucide-react";

export function Ports() {
  const { ports, fetchPortUsage, scanPorts, findFreePorts, isLoading } = usePortStore();
  const [searchTerm, setSearchTerm] = useState("");
  const [freePorts, setFreePorts] = useState<number[]>([]);
  const [isScanning, setIsScanning] = useState(false);

  useEffect(() => {
    fetchPortUsage();
  }, [fetchPortUsage]);

  const filteredPorts = ports.filter(
    (port) =>
      port.port.toString().includes(searchTerm) ||
      port.process_name?.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleFindFreePorts = async () => {
    const free = await findFreePorts(10);
    setFreePorts(free);
  };

  const handleScanPorts = async () => {
    setIsScanning(true);
    await scanPorts(1, 1024); // Scan common ports
    setIsScanning(false);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Ports</h1>
          <p className="text-muted-foreground">
            View and manage network port usage
          </p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={handleFindFreePorts}>
            Find Free Ports
          </Button>
          <Button variant="outline" onClick={handleScanPorts} disabled={isScanning}>
            {isScanning ? "Scanning..." : "Scan Range"}
          </Button>
          <Button onClick={fetchPortUsage} disabled={isLoading}>
            <RefreshCw className={`mr-2 h-4 w-4 ${isLoading ? "animate-spin" : ""}`} />
            Refresh
          </Button>
        </div>
      </div>

      {/* Search */}
      <div className="relative max-w-md">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <input
          type="text"
          placeholder="Search by port or process name..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="h-10 w-full rounded-md border bg-background pl-10 pr-4 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
        />
      </div>

      {/* Free Ports */}
      {freePorts.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Available Free Ports</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex flex-wrap gap-2">
              {freePorts.map((port) => (
                <Badge key={port} variant="outline" className="font-mono">
                  :{port}
                </Badge>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Ports Table */}
      <Card>
        <CardHeader>
          <CardTitle>
            Occupied Ports ({filteredPorts.length})
          </CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="flex items-center justify-center py-8">
              <RefreshCw className="h-6 w-6 animate-spin text-muted-foreground" />
            </div>
          ) : filteredPorts.length === 0 ? (
            <p className="py-8 text-center text-muted-foreground">
              {searchTerm ? "No ports matching your search" : "No ports in use"}
            </p>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b text-left text-sm text-muted-foreground">
                    <th className="pb-3 font-medium">Port</th>
                    <th className="pb-3 font-medium">Protocol</th>
                    <th className="pb-3 font-medium">Process</th>
                    <th className="pb-3 font-medium">PID</th>
                    <th className="pb-3 font-medium">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {filteredPorts.map((port) => (
                    <tr key={port.port} className="border-b last:border-0">
                      <td className="py-3 font-mono font-medium">:{port.port}</td>
                      <td className="py-3">
                        <Badge variant="outline">
                          {port.protocol.toUpperCase()}
                        </Badge>
                      </td>
                      <td className="py-3">
                        {port.process_name || (
                          <span className="text-muted-foreground">Unknown</span>
                        )}
                      </td>
                      <td className="py-3 font-mono">
                        {port.pid || "-"}
                      </td>
                      <td className="py-3">
                        <Badge
                          variant={port.status === "occupied" ? "default" : "secondary"}
                        >
                          {port.status}
                        </Badge>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

import { useEffect, useState, useRef } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Badge } from "../components/ui/badge";
import { Cpu, MemoryStick, MonitorSpeaker, RefreshCw } from "lucide-react";
import * as api from "../lib/tauri/commands";
import type { SystemStats, GpuProvider } from "../lib/tauri/types";

interface StatsHistory {
  timestamps: number[];
  cpuUsage: number[];
  memoryUsage: number[];
  gpuUsage: number[];
}

const MAX_HISTORY_POINTS = 360; // 30 minutes at 5s intervals
const UPDATE_INTERVAL = 5000; // 5 seconds

function formatBytes(bytes: number): string {
  const units = ["B", "KB", "MB", "GB", "TB"];
  let size = bytes;
  let unitIndex = 0;
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }
  return `${size.toFixed(1)} ${units[unitIndex]}`;
}

function MiniChart({ data, color, label }: { data: number[]; color: string; label: string }) {
  const maxValue = Math.max(...data, 1);
  const width = 300;
  const height = 60;
  const padding = 4;

  const points = data.map((value, index) => {
    const x = padding + (index / (data.length - 1 || 1)) * (width - 2 * padding);
    const y = height - padding - (value / maxValue) * (height - 2 * padding);
    return `${x},${y}`;
  }).join(" ");

  const currentValue = data[data.length - 1] || 0;

  return (
    <div className="space-y-1">
      <div className="flex justify-between items-center text-xs text-muted-foreground">
        <span>{label}</span>
        <span className="font-mono">{currentValue.toFixed(1)}%</span>
      </div>
      <svg width={width} height={height} className="bg-muted/30 rounded">
        {data.length > 1 && (
          <>
            <defs>
              <linearGradient id={`gradient-${label}`} x1="0%" y1="0%" x2="0%" y2="100%">
                <stop offset="0%" stopColor={color} stopOpacity="0.3" />
                <stop offset="100%" stopColor={color} stopOpacity="0" />
              </linearGradient>
            </defs>
            <polygon
              points={`${padding},${height - padding} ${points} ${width - padding},${height - padding}`}
              fill={`url(#gradient-${label})`}
            />
            <polyline
              points={points}
              fill="none"
              stroke={color}
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </>
        )}
      </svg>
    </div>
  );
}

function UsageBar({ value, label, color }: { value: number; label: string; color: string }) {
  return (
    <div className="space-y-1">
      <div className="flex justify-between text-sm">
        <span className="text-muted-foreground">{label}</span>
        <span className="font-mono font-medium">{value.toFixed(1)}%</span>
      </div>
      <div className="h-2 bg-muted rounded-full overflow-hidden">
        <div
          className="h-full transition-all duration-300"
          style={{ width: `${Math.min(value, 100)}%`, backgroundColor: color }}
        />
      </div>
    </div>
  );
}

export function Performance() {
  const [stats, setStats] = useState<SystemStats | null>(null);
  const [history, setHistory] = useState<StatsHistory>({
    timestamps: [],
    cpuUsage: [],
    memoryUsage: [],
    gpuUsage: [],
  });
  const [isLive, setIsLive] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [gpuProvider, setGpuProvider] = useState<GpuProvider>("auto");
  const intervalRef = useRef<number | null>(null);

  const fetchStats = async () => {
    try {
      const newStats = await api.getSystemStats();
      setStats(newStats);
      setError(null);

      setHistory((prev) => {
        const newTimestamps = [...prev.timestamps, newStats.timestamp];
        const newCpuUsage = [...prev.cpuUsage, newStats.cpu.usage_percent];
        const newMemoryUsage = [...prev.memoryUsage, newStats.memory.usage_percent];
        const newGpuUsage = [...prev.gpuUsage, newStats.gpus[0]?.usage_percent ?? 0];

        // Keep only last MAX_HISTORY_POINTS
        return {
          timestamps: newTimestamps.slice(-MAX_HISTORY_POINTS),
          cpuUsage: newCpuUsage.slice(-MAX_HISTORY_POINTS),
          memoryUsage: newMemoryUsage.slice(-MAX_HISTORY_POINTS),
          gpuUsage: newGpuUsage.slice(-MAX_HISTORY_POINTS),
        };
      });
    } catch (err) {
      setError(String(err));
    }
  };

  useEffect(() => {
    fetchStats();

    if (isLive) {
      intervalRef.current = window.setInterval(fetchStats, UPDATE_INTERVAL);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [isLive]);

  const handleGpuProviderChange = async (provider: GpuProvider) => {
    try {
      await api.setGpuProvider(provider);
      setGpuProvider(provider);
    } catch (err) {
      setError(String(err));
    }
  };

  const getCpuColor = (usage: number) => {
    if (usage > 80) return "#ef4444";
    if (usage > 50) return "#f59e0b";
    return "#22c55e";
  };

  const getMemoryColor = (usage: number) => {
    if (usage > 90) return "#ef4444";
    if (usage > 70) return "#f59e0b";
    return "#3b82f6";
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Performance</h1>
          <p className="text-muted-foreground">
            Live CPU, Memory und GPU Auslastung
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant={isLive ? "default" : "outline"}
            onClick={() => setIsLive(!isLive)}
            size="sm"
          >
            {isLive ? "Live" : "Pausiert"}
          </Button>
          <Button variant="outline" size="sm" onClick={fetchStats}>
            <RefreshCw className="h-4 w-4" />
          </Button>
        </div>
      </div>

      {error && (
        <Card className="border-destructive">
          <CardContent className="pt-4">
            <p className="text-destructive text-sm">{error}</p>
          </CardContent>
        </Card>
      )}

      {/* Stats Overview */}
      <div className="grid gap-4 md:grid-cols-3">
        {/* CPU Card */}
        <Card>
          <CardHeader className="pb-2">
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg flex items-center gap-2">
                <Cpu className="h-5 w-5" />
                CPU
              </CardTitle>
              {stats && (
                <Badge variant={stats.cpu.usage_percent > 80 ? "destructive" : "secondary"}>
                  {stats.cpu.usage_percent.toFixed(1)}%
                </Badge>
              )}
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            {stats && (
              <>
                <MiniChart
                  data={history.cpuUsage}
                  color={getCpuColor(stats.cpu.usage_percent)}
                  label="CPU Auslastung"
                />
                <div className="grid grid-cols-2 gap-2 text-sm">
                  <div>
                    <p className="text-muted-foreground">Kerne</p>
                    <p className="font-mono font-medium">{stats.cpu.core_count}</p>
                  </div>
                  <div>
                    <p className="text-muted-foreground">Frequenz</p>
                    <p className="font-mono font-medium">
                      {stats.cpu.frequency_mhz ? `${stats.cpu.frequency_mhz} MHz` : "-"}
                    </p>
                  </div>
                </div>
                {/* Per-core usage */}
                <div className="space-y-1">
                  <p className="text-xs text-muted-foreground">Pro Kern:</p>
                  <div className="grid grid-cols-4 gap-1">
                    {stats.cpu.per_core_usage.slice(0, 8).map((usage, i) => (
                      <div
                        key={i}
                        className="h-1 rounded-full"
                        style={{
                          backgroundColor: getCpuColor(usage),
                          opacity: 0.3 + (usage / 100) * 0.7,
                        }}
                        title={`Core ${i}: ${usage.toFixed(1)}%`}
                      />
                    ))}
                  </div>
                </div>
              </>
            )}
          </CardContent>
        </Card>

        {/* Memory Card */}
        <Card>
          <CardHeader className="pb-2">
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg flex items-center gap-2">
                <MemoryStick className="h-5 w-5" />
                Arbeitsspeicher
              </CardTitle>
              {stats && (
                <Badge variant={stats.memory.usage_percent > 90 ? "destructive" : "secondary"}>
                  {stats.memory.usage_percent.toFixed(1)}%
                </Badge>
              )}
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            {stats && (
              <>
                <MiniChart
                  data={history.memoryUsage}
                  color={getMemoryColor(stats.memory.usage_percent)}
                  label="RAM Auslastung"
                />
                <div className="grid grid-cols-2 gap-2 text-sm">
                  <div>
                    <p className="text-muted-foreground">Belegt</p>
                    <p className="font-mono font-medium">{formatBytes(stats.memory.used_bytes)}</p>
                  </div>
                  <div>
                    <p className="text-muted-foreground">Gesamt</p>
                    <p className="font-mono font-medium">{formatBytes(stats.memory.total_bytes)}</p>
                  </div>
                </div>
                <UsageBar
                  value={stats.memory.usage_percent}
                  label="RAM"
                  color={getMemoryColor(stats.memory.usage_percent)}
                />
                {stats.memory.swap_total_bytes > 0 && (
                  <UsageBar
                    value={(stats.memory.swap_used_bytes / stats.memory.swap_total_bytes) * 100}
                    label="Swap"
                    color="#8b5cf6"
                  />
                )}
              </>
            )}
          </CardContent>
        </Card>

        {/* GPU Card */}
        <Card>
          <CardHeader className="pb-2">
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg flex items-center gap-2">
                <MonitorSpeaker className="h-5 w-5" />
                GPU
              </CardTitle>
              <div className="flex items-center gap-2">
                <select
                  value={gpuProvider}
                  onChange={(e) => handleGpuProviderChange(e.target.value as GpuProvider)}
                  className="text-xs bg-muted rounded px-2 py-1"
                >
                  <option value="auto">Auto</option>
                  <option value="apple">Apple</option>
                  <option value="nvidia">NVIDIA</option>
                  <option value="amd">AMD</option>
                  <option value="none">Keine</option>
                </select>
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            {stats?.gpus && stats.gpus.length > 0 ? (
              stats.gpus.map((gpu, index) => (
                <div key={index} className="space-y-2">
                  <p className="text-sm font-medium truncate">{gpu.name}</p>
                  {gpu.usage_percent !== null ? (
                    <UsageBar value={gpu.usage_percent} label="GPU Last" color="#10b981" />
                  ) : (
                    <p className="text-xs text-muted-foreground">
                      GPU-Auslastung nicht verfügbar
                    </p>
                  )}
                  {gpu.memory_used_bytes !== null && gpu.memory_total_bytes !== null && (
                    <div className="text-xs text-muted-foreground">
                      VRAM: {formatBytes(gpu.memory_used_bytes)} / {formatBytes(gpu.memory_total_bytes)}
                    </div>
                  )}
                  {gpu.temperature_celsius !== null && (
                    <div className="text-xs text-muted-foreground">
                      Temperatur: {gpu.temperature_celsius}°C
                    </div>
                  )}
                </div>
              ))
            ) : (
              <div className="text-center py-4 text-muted-foreground">
                <p className="text-sm">Keine GPU erkannt</p>
                <p className="text-xs mt-1">Wähle den GPU-Typ manuell aus</p>
              </div>
            )}
            {history.gpuUsage.some((v) => v > 0) && (
              <MiniChart data={history.gpuUsage} color="#10b981" label="GPU History" />
            )}
          </CardContent>
        </Card>
      </div>

      {/* History Charts */}
      <Card>
        <CardHeader>
          <CardTitle>Verlauf (letzte 30 Minuten)</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 md:grid-cols-3">
            <MiniChart data={history.cpuUsage} color="#22c55e" label="CPU" />
            <MiniChart data={history.memoryUsage} color="#3b82f6" label="RAM" />
            <MiniChart data={history.gpuUsage} color="#10b981" label="GPU" />
          </div>
          <p className="text-xs text-muted-foreground mt-4 text-center">
            {history.timestamps.length} Datenpunkte • Update alle {UPDATE_INTERVAL / 1000}s
          </p>
        </CardContent>
      </Card>
    </div>
  );
}

import { cn } from "../../lib/utils";

interface ResourceBarProps {
  value: number | null;
  max?: number;
  label: string;
  unit?: string;
  showValue?: boolean;
  colorClass?: string;
  size?: "sm" | "md";
}

export function ResourceBar({
  value,
  max = 100,
  label,
  unit = "%",
  showValue = true,
  colorClass,
  size = "sm",
}: ResourceBarProps) {
  if (value === null || value === undefined) {
    return (
      <div className="text-xs text-muted-foreground">
        {label}: -
      </div>
    );
  }

  const percentage = Math.min((value / max) * 100, 100);

  // Determine color based on percentage
  const getColorClass = () => {
    if (colorClass) return colorClass;
    if (percentage > 80) return "bg-red-500";
    if (percentage > 60) return "bg-amber-500";
    if (percentage > 40) return "bg-yellow-500";
    return "bg-green-500";
  };

  const formatValue = (val: number) => {
    if (unit === "MB") {
      return `${(val / (1024 * 1024)).toFixed(1)} MB`;
    }
    if (unit === "GB") {
      return `${(val / (1024 * 1024 * 1024)).toFixed(2)} GB`;
    }
    return `${val.toFixed(1)}${unit}`;
  };

  return (
    <div className={cn("space-y-1", size === "sm" ? "text-xs" : "text-sm")}>
      <div className="flex justify-between text-muted-foreground">
        <span>{label}</span>
        {showValue && <span className="font-mono">{formatValue(value)}</span>}
      </div>
      <div className={cn(
        "w-full rounded-full bg-secondary overflow-hidden",
        size === "sm" ? "h-1.5" : "h-2"
      )}>
        <div
          className={cn("h-full rounded-full transition-all duration-300", getColorClass())}
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  );
}

interface ResourceStatsProps {
  cpuUsage: number | null;
  memoryBytes: number | null;
  memoryPercent: number | null;
  compact?: boolean;
}

export function ResourceStats({ cpuUsage, memoryBytes, memoryPercent, compact = false }: ResourceStatsProps) {
  if (compact) {
    // Compact inline display
    return (
      <div className="flex items-center gap-3 text-xs">
        <div className="flex items-center gap-1.5">
          <span className="text-muted-foreground">CPU:</span>
          <div className="w-16 h-1.5 rounded-full bg-secondary overflow-hidden">
            <div
              className={cn(
                "h-full rounded-full transition-all",
                cpuUsage === null ? "bg-muted" :
                cpuUsage > 80 ? "bg-red-500" :
                cpuUsage > 50 ? "bg-amber-500" : "bg-green-500"
              )}
              style={{ width: `${Math.min(cpuUsage ?? 0, 100)}%` }}
            />
          </div>
          <span className="font-mono w-12 text-right">
            {cpuUsage !== null ? `${cpuUsage.toFixed(1)}%` : "-"}
          </span>
        </div>
        <div className="flex items-center gap-1.5">
          <span className="text-muted-foreground">RAM:</span>
          <div className="w-16 h-1.5 rounded-full bg-secondary overflow-hidden">
            <div
              className={cn(
                "h-full rounded-full transition-all",
                memoryPercent === null ? "bg-muted" :
                memoryPercent > 80 ? "bg-red-500" :
                memoryPercent > 50 ? "bg-amber-500" : "bg-green-500"
              )}
              style={{ width: `${Math.min(memoryPercent ?? 0, 100)}%` }}
            />
          </div>
          <span className="font-mono w-16 text-right">
            {memoryBytes !== null
              ? `${(memoryBytes / (1024 * 1024)).toFixed(0)} MB`
              : "-"}
          </span>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      <ResourceBar
        value={cpuUsage}
        label="CPU"
        unit="%"
      />
      <ResourceBar
        value={memoryPercent}
        label="RAM"
        unit="%"
      />
      {memoryBytes !== null && (
        <div className="text-xs text-muted-foreground text-right">
          {(memoryBytes / (1024 * 1024)).toFixed(1)} MB
        </div>
      )}
    </div>
  );
}

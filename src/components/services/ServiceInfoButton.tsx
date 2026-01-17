import { useState } from "react";
import { Info, Loader2, X } from "lucide-react";
import { Button } from "../ui/button";
import { explainProcess } from "../../lib/tauri/commands";
import type { Service } from "../../lib/tauri/types";

interface ServiceInfoButtonProps {
  service: Service;
  size?: "sm" | "default";
}

export function ServiceInfoButton({ service, size = "sm" }: ServiceInfoButtonProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [explanation, setExplanation] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleClick = async () => {
    if (isOpen) {
      setIsOpen(false);
      return;
    }

    setIsOpen(true);
    setIsLoading(true);
    setError(null);

    try {
      const result = await explainProcess(
        service.name,
        service.path,
        service.description
      );
      setExplanation(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Fehler beim Laden der Erklärung");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="relative inline-block">
      <Button
        variant="ghost"
        size={size === "sm" ? "icon" : "default"}
        className={size === "sm" ? "h-6 w-6" : "h-8 w-8"}
        onClick={handleClick}
        title="Was ist dieser Prozess?"
      >
        <Info className={size === "sm" ? "h-3.5 w-3.5" : "h-4 w-4"} />
      </Button>

      {isOpen && (
        <div className="absolute z-50 w-80 rounded-lg border bg-popover p-4 text-popover-foreground shadow-lg left-0 top-full mt-1">
          <div className="flex items-start justify-between gap-2 mb-2">
            <h4 className="font-semibold text-sm">{service.name}</h4>
            <Button
              variant="ghost"
              size="icon"
              className="h-5 w-5 -mr-2 -mt-1"
              onClick={() => setIsOpen(false)}
            >
              <X className="h-3 w-3" />
            </Button>
          </div>

          {isLoading ? (
            <div className="flex items-center gap-2 text-sm text-muted-foreground py-2">
              <Loader2 className="h-4 w-4 animate-spin" />
              <span>Erkläre Prozess...</span>
            </div>
          ) : error ? (
            <div className="text-sm text-destructive">{error}</div>
          ) : explanation ? (
            <p className="text-sm text-muted-foreground leading-relaxed">
              {explanation}
            </p>
          ) : null}

          {service.path && (
            <p className="text-xs text-muted-foreground mt-3 pt-2 border-t font-mono truncate">
              {service.path}
            </p>
          )}
        </div>
      )}
    </div>
  );
}

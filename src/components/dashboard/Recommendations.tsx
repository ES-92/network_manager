import { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "../ui/card";
import { Button } from "../ui/button";
import { Badge } from "../ui/badge";
import {
  Lightbulb,
  StopCircle,
  Power,
  Gauge,
  Shield,
  AlertTriangle,
  Info,
  RefreshCw,
  Loader2,
  Sparkles,
} from "lucide-react";
import { getServiceRecommendations, checkOllamaStatus } from "../../lib/tauri/commands";
import type { Service, ServiceRecommendation, RecommendationType } from "../../lib/tauri/types";

interface RecommendationsProps {
  services: Service[];
}

const recommendationConfig: Record<
  RecommendationType,
  { icon: React.ElementType; color: string; bgColor: string }
> = {
  stop_service: {
    icon: StopCircle,
    color: "text-red-500",
    bgColor: "bg-red-500/10",
  },
  disable_autostart: {
    icon: Power,
    color: "text-amber-500",
    bgColor: "bg-amber-500/10",
  },
  reduce_resources: {
    icon: Gauge,
    color: "text-orange-500",
    bgColor: "bg-orange-500/10",
  },
  security_concern: {
    icon: Shield,
    color: "text-purple-500",
    bgColor: "bg-purple-500/10",
  },
  performance_impact: {
    icon: AlertTriangle,
    color: "text-yellow-500",
    bgColor: "bg-yellow-500/10",
  },
  info: {
    icon: Info,
    color: "text-blue-500",
    bgColor: "bg-blue-500/10",
  },
};

export function Recommendations({ services }: RecommendationsProps) {
  const [recommendations, setRecommendations] = useState<ServiceRecommendation[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [ollamaAvailable, setOllamaAvailable] = useState(false);
  const [hasLoaded, setHasLoaded] = useState(false);

  useEffect(() => {
    checkOllamaStatus().then(setOllamaAvailable);
  }, []);

  const loadRecommendations = async () => {
    if (services.length === 0) return;

    setIsLoading(true);
    try {
      const result = await getServiceRecommendations(services);
      setRecommendations(result);
      setHasLoaded(true);
    } catch (err) {
      console.error("Failed to load recommendations:", err);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    if (services.length > 0 && !hasLoaded) {
      loadRecommendations();
    }
  }, [services, hasLoaded]);

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Sparkles className="h-5 w-5 text-violet-500" />
            <CardTitle>KI-Empfehlungen</CardTitle>
          </div>
          <div className="flex items-center gap-2">
            {ollamaAvailable ? (
              <Badge variant="outline" className="text-green-600 border-green-300">
                Ollama aktiv
              </Badge>
            ) : (
              <Badge variant="outline" className="text-muted-foreground">
                Ollama offline
              </Badge>
            )}
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8"
              onClick={loadRecommendations}
              disabled={isLoading}
            >
              {isLoading ? (
                <Loader2 className="h-4 w-4 animate-spin" />
              ) : (
                <RefreshCw className="h-4 w-4" />
              )}
            </Button>
          </div>
        </div>
        <CardDescription>
          Optimierungsvorschläge basierend auf Ihren laufenden Services
        </CardDescription>
      </CardHeader>
      <CardContent>
        {isLoading ? (
          <div className="flex items-center justify-center py-8 text-muted-foreground">
            <Loader2 className="h-6 w-6 animate-spin mr-2" />
            <span>Analysiere Services...</span>
          </div>
        ) : recommendations.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-8 text-center text-muted-foreground">
            <Lightbulb className="h-10 w-10 mb-3 opacity-50" />
            <p className="text-sm">Keine Empfehlungen verfügbar</p>
            <p className="text-xs mt-1">
              {ollamaAvailable
                ? "Alle Services sehen gut aus!"
                : "Starten Sie Ollama für KI-basierte Empfehlungen"}
            </p>
          </div>
        ) : (
          <div className="space-y-3">
            {recommendations.map((rec, index) => {
              const config = recommendationConfig[rec.recommendation_type] || recommendationConfig.info;
              const Icon = config.icon;

              return (
                <div
                  key={`${rec.service_id}-${index}`}
                  className={`rounded-lg border p-4 ${config.bgColor}`}
                >
                  <div className="flex items-start gap-3">
                    <div className={`mt-0.5 ${config.color}`}>
                      <Icon className="h-5 w-5" />
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-1">
                        <h4 className="font-medium text-sm">{rec.title}</h4>
                        <Badge variant="outline" className="text-xs">
                          {rec.service_name}
                        </Badge>
                      </div>
                      <p className="text-sm text-muted-foreground">
                        {rec.description}
                      </p>
                      {rec.action && (
                        <div className="mt-2">
                          <code className="text-xs bg-muted px-2 py-1 rounded font-mono">
                            {rec.action}
                          </code>
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </CardContent>
    </Card>
  );
}

import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Badge } from "../components/ui/badge";
import {
  Shield, ShieldAlert, ShieldCheck, ShieldX,
  RefreshCw, AlertTriangle, Lock, Unlock, Globe, Database, Cpu
} from "lucide-react";
import * as api from "../lib/tauri/commands";
import { useServiceStore } from "../stores/serviceStore";
import type { SecurityScanResult, SecurityIssue, SecuritySeverity } from "../lib/tauri/types";

const severityColors: Record<SecuritySeverity, { bg: string; text: string; icon: typeof ShieldAlert }> = {
  critical: { bg: "bg-red-500/10", text: "text-red-500", icon: ShieldX },
  high: { bg: "bg-orange-500/10", text: "text-orange-500", icon: ShieldAlert },
  medium: { bg: "bg-yellow-500/10", text: "text-yellow-500", icon: AlertTriangle },
  low: { bg: "bg-blue-500/10", text: "text-blue-500", icon: Shield },
  info: { bg: "bg-muted", text: "text-muted-foreground", icon: Shield },
};

const categoryIcons: Record<string, typeof Lock> = {
  unencrypted_connection: Unlock,
  public_exposure: Globe,
  missing_authentication: Lock,
  privilege_escalation: Cpu,
  default_credentials: Lock,
  outdated_software: AlertTriangle,
  insecure_configuration: AlertTriangle,
  data_leakage: Database,
};

function SecurityIssueCard({ issue }: { issue: SecurityIssue }) {
  const { bg, text, icon: SeverityIcon } = severityColors[issue.severity];
  const CategoryIcon = categoryIcons[issue.category] || Shield;

  return (
    <Card className={`${bg} border-l-4 ${text.replace("text-", "border-")}`}>
      <CardContent className="pt-4">
        <div className="flex items-start gap-3">
          <div className={`p-2 rounded-full ${bg}`}>
            <SeverityIcon className={`h-5 w-5 ${text}`} />
          </div>
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 flex-wrap">
              <h3 className="font-medium">{issue.title}</h3>
              <Badge variant="outline" className="text-xs">
                <CategoryIcon className="h-3 w-3 mr-1" />
                {issue.category.replace(/_/g, " ")}
              </Badge>
              {issue.port && (
                <Badge variant="secondary" className="text-xs font-mono">
                  Port {issue.port}
                </Badge>
              )}
            </div>
            <p className="text-sm text-muted-foreground mt-1">{issue.description}</p>
            <div className="mt-3 p-2 bg-muted/50 rounded text-sm">
              <strong className="text-xs uppercase text-muted-foreground">Empfehlung:</strong>
              <p className="mt-1">{issue.recommendation}</p>
            </div>
            {issue.service_name && (
              <p className="text-xs text-muted-foreground mt-2">
                Service: {issue.service_name}
              </p>
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

export function Security() {
  const [scanResult, setScanResult] = useState<SecurityScanResult | null>(null);
  const [llmAnalysis, setLlmAnalysis] = useState<string | null>(null);
  const [isScanning, setIsScanning] = useState(false);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { services, fetchServices } = useServiceStore();

  const runScan = async () => {
    setIsScanning(true);
    setError(null);
    try {
      await fetchServices();
      const result = await api.scanSecurity();
      setScanResult(result);
    } catch (err) {
      setError(String(err));
    } finally {
      setIsScanning(false);
    }
  };

  const runLlmAnalysis = async () => {
    setIsAnalyzing(true);
    setError(null);
    try {
      const servicesJson = JSON.stringify(
        services.slice(0, 30).map((s) => ({
          name: s.name,
          ports: s.ports,
          status: s.status,
          type: s.service_type,
        }))
      );
      const analysis = await api.getSecurityAnalysis(servicesJson);
      setLlmAnalysis(analysis);
    } catch (err) {
      setError(String(err));
    } finally {
      setIsAnalyzing(false);
    }
  };

  useEffect(() => {
    runScan();
  }, []);

  const criticalAndHigh = scanResult?.issues.filter(
    (i) => i.severity === "critical" || i.severity === "high"
  ) || [];
  const mediumAndLow = scanResult?.issues.filter(
    (i) => i.severity === "medium" || i.severity === "low" || i.severity === "info"
  ) || [];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Sicherheit</h1>
          <p className="text-muted-foreground">
            Schwachstellenanalyse und Sicherheitsempfehlungen
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button onClick={runScan} disabled={isScanning} variant="outline">
            <RefreshCw className={`h-4 w-4 mr-2 ${isScanning ? "animate-spin" : ""}`} />
            Scan
          </Button>
          <Button onClick={runLlmAnalysis} disabled={isAnalyzing} variant="default">
            <Shield className={`h-4 w-4 mr-2 ${isAnalyzing ? "animate-pulse" : ""}`} />
            KI-Analyse
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

      {/* Summary Cards */}
      <div className="grid gap-4 md:grid-cols-4">
        <Card className={scanResult && scanResult.critical_count > 0 ? "border-red-500" : ""}>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm flex items-center gap-2">
              <ShieldX className="h-4 w-4 text-red-500" />
              Kritisch
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-3xl font-bold text-red-500">
              {scanResult?.critical_count ?? "-"}
            </div>
          </CardContent>
        </Card>

        <Card className={scanResult && scanResult.high_count > 0 ? "border-orange-500" : ""}>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm flex items-center gap-2">
              <ShieldAlert className="h-4 w-4 text-orange-500" />
              Hoch
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-3xl font-bold text-orange-500">
              {scanResult?.high_count ?? "-"}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm flex items-center gap-2">
              <AlertTriangle className="h-4 w-4 text-yellow-500" />
              Mittel
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-3xl font-bold text-yellow-500">
              {scanResult?.medium_count ?? "-"}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm flex items-center gap-2">
              <ShieldCheck className="h-4 w-4 text-green-500" />
              Gescannt
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-xl font-bold">
              {scanResult ? (
                <span>
                  {scanResult.services_scanned} Services, {scanResult.ports_scanned} Ports
                </span>
              ) : (
                "-"
              )}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* LLM Analysis */}
      {llmAnalysis && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Shield className="h-5 w-5" />
              KI-Sicherheitsanalyse
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="prose prose-sm max-w-none dark:prose-invert">
              <pre className="whitespace-pre-wrap text-sm bg-muted p-4 rounded-lg">
                {llmAnalysis}
              </pre>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Critical and High Issues */}
      {criticalAndHigh.length > 0 && (
        <div className="space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <ShieldAlert className="h-5 w-5 text-red-500" />
            Kritische Probleme ({criticalAndHigh.length})
          </h2>
          <div className="space-y-3">
            {criticalAndHigh.map((issue) => (
              <SecurityIssueCard key={issue.id} issue={issue} />
            ))}
          </div>
        </div>
      )}

      {/* Medium and Low Issues */}
      {mediumAndLow.length > 0 && (
        <div className="space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <AlertTriangle className="h-5 w-5 text-yellow-500" />
            Weitere Hinweise ({mediumAndLow.length})
          </h2>
          <div className="space-y-3">
            {mediumAndLow.map((issue) => (
              <SecurityIssueCard key={issue.id} issue={issue} />
            ))}
          </div>
        </div>
      )}

      {/* No Issues */}
      {scanResult && scanResult.issues.length === 0 && (
        <Card className="border-green-500">
          <CardContent className="pt-6 text-center">
            <ShieldCheck className="h-12 w-12 text-green-500 mx-auto mb-4" />
            <h3 className="text-xl font-semibold text-green-500">Keine Probleme gefunden</h3>
            <p className="text-muted-foreground mt-2">
              Der Security-Scan hat keine offensichtlichen Schwachstellen entdeckt.
            </p>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

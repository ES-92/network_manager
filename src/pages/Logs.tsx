import { useState, useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "../components/ui/card";
import { Button } from "../components/ui/button";
import { Badge } from "../components/ui/badge";
import {
  FileText,
  Bot,
  RefreshCw,
  AlertCircle,
  CheckCircle2,
  XCircle,
  Sparkles,
  Download,
  Trash2,
  Search,
  ChevronDown,
} from "lucide-react";
import * as api from "../lib/tauri/commands";
import type { AuditEntry } from "../lib/tauri/types";

type AnalysisType = "error" | "pattern" | "anomaly" | "performance" | "security";

const analysisTypes: { value: AnalysisType; label: string; description: string }[] = [
  { value: "error", label: "Error Detection", description: "Find and explain errors in logs" },
  { value: "pattern", label: "Pattern Analysis", description: "Identify recurring patterns" },
  { value: "anomaly", label: "Anomaly Detection", description: "Detect unusual behavior" },
  { value: "performance", label: "Performance Analysis", description: "Identify performance issues" },
  { value: "security", label: "Security Analysis", description: "Find security concerns" },
];

export function Logs() {
  // Audit logs state
  const [auditLogs, setAuditLogs] = useState<AuditEntry[]>([]);
  const [logsLoading, setLogsLoading] = useState(false);
  const [logsError, setLogsError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState("");

  // LLM state
  const [ollamaStatus, setOllamaStatus] = useState<"checking" | "connected" | "disconnected">("checking");
  const [ollamaModels, setOllamaModels] = useState<string[]>([]);
  const [selectedModel, setSelectedModel] = useState<string>("");
  const [logInput, setLogInput] = useState("");
  const [analysisType, setAnalysisType] = useState<AnalysisType>("error");
  const [analysisResult, setAnalysisResult] = useState<string | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [analysisError, setAnalysisError] = useState<string | null>(null);

  // Fetch audit logs on mount
  useEffect(() => {
    fetchAuditLogs();
    checkOllamaStatus();
  }, []);

  const fetchAuditLogs = async () => {
    setLogsLoading(true);
    setLogsError(null);
    try {
      const logs = await api.getAuditLogs(100);
      setAuditLogs(logs);
    } catch (error) {
      setLogsError(String(error));
    } finally {
      setLogsLoading(false);
    }
  };

  const checkOllamaStatus = async () => {
    setOllamaStatus("checking");
    try {
      const isConnected = await api.checkOllamaStatus();
      setOllamaStatus(isConnected ? "connected" : "disconnected");

      if (isConnected) {
        const models = await api.listOllamaModels();
        setOllamaModels(models);
        if (models.length > 0 && !selectedModel) {
          // Prefer mistral if available
          const mistral = models.find((m) => m.toLowerCase().includes("mistral"));
          setSelectedModel(mistral || models[0]);
        }
      }
    } catch {
      setOllamaStatus("disconnected");
    }
  };

  const handleAnalyze = async () => {
    if (!logInput.trim()) {
      setAnalysisError("Please enter some log content to analyze");
      return;
    }

    setIsAnalyzing(true);
    setAnalysisError(null);
    setAnalysisResult(null);

    try {
      if (selectedModel) {
        await api.setOllamaModel(selectedModel);
      }
      const result = await api.analyzeLogs(logInput, analysisType);
      setAnalysisResult(result);
    } catch (error) {
      setAnalysisError(String(error));
    } finally {
      setIsAnalyzing(false);
    }
  };

  const handleExportLogs = async () => {
    try {
      const exportData = await api.exportAuditLogs("json");
      // Create download link
      const blob = new Blob([exportData], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `audit-logs-${new Date().toISOString().split("T")[0]}.json`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (error) {
      setLogsError(String(error));
    }
  };

  const filteredLogs = auditLogs.filter(
    (log) =>
      log.operation.toLowerCase().includes(searchTerm.toLowerCase()) ||
      log.event_type.toLowerCase().includes(searchTerm.toLowerCase()) ||
      log.user.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const getEventIcon = (eventType: string, success: boolean) => {
    if (!success) return <XCircle className="h-4 w-4 text-destructive" />;

    switch (eventType) {
      case "service_start":
      case "service_restart":
        return <CheckCircle2 className="h-4 w-4 text-green-500" />;
      case "service_stop":
      case "process_kill":
        return <AlertCircle className="h-4 w-4 text-amber-500" />;
      case "llm_analysis":
        return <Bot className="h-4 w-4 text-purple-500" />;
      default:
        return <FileText className="h-4 w-4 text-muted-foreground" />;
    }
  };

  const formatTimestamp = (timestamp: string) => {
    return new Date(timestamp).toLocaleString();
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Logs & Analysis</h1>
        <p className="text-muted-foreground">
          View audit logs and AI-powered log analysis
        </p>
      </div>

      {/* LLM Analysis */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Bot className="h-5 w-5" />
              <CardTitle>AI Log Analysis</CardTitle>
              <Badge
                variant={
                  ollamaStatus === "connected"
                    ? "success"
                    : ollamaStatus === "checking"
                    ? "secondary"
                    : "destructive"
                }
              >
                {ollamaStatus === "connected"
                  ? "Ollama Connected"
                  : ollamaStatus === "checking"
                  ? "Checking..."
                  : "Ollama Offline"}
              </Badge>
            </div>
            <Button variant="outline" size="sm" onClick={checkOllamaStatus}>
              <RefreshCw className="mr-2 h-4 w-4" />
              Refresh Status
            </Button>
          </div>
          <CardDescription>
            Paste log content below and let AI analyze it for errors, patterns, and anomalies.
            Requires Ollama running locally on port 11434.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {ollamaStatus === "disconnected" && (
            <div className="rounded-md border border-amber-200 bg-amber-50 p-4 dark:border-amber-900 dark:bg-amber-950">
              <div className="flex items-start gap-3">
                <AlertCircle className="mt-0.5 h-5 w-5 text-amber-600 dark:text-amber-400" />
                <div>
                  <p className="font-medium text-amber-800 dark:text-amber-200">
                    Ollama is not running
                  </p>
                  <p className="mt-1 text-sm text-amber-700 dark:text-amber-300">
                    Start Ollama with a model like Mistral to enable AI analysis:
                  </p>
                  <code className="mt-2 block rounded bg-amber-100 px-2 py-1 text-sm dark:bg-amber-900">
                    ollama run mistral
                  </code>
                </div>
              </div>
            </div>
          )}

          {ollamaStatus === "connected" && (
            <>
              {/* Model Selector */}
              <div className="flex items-center gap-4">
                <label className="text-sm font-medium">Model:</label>
                <select
                  value={selectedModel}
                  onChange={(e) => setSelectedModel(e.target.value)}
                  className="rounded-md border bg-background px-3 py-2 text-sm"
                >
                  {ollamaModels.map((model) => (
                    <option key={model} value={model}>
                      {model}
                    </option>
                  ))}
                </select>
              </div>

              {/* Analysis Type */}
              <div className="flex flex-wrap gap-2">
                {analysisTypes.map((type) => (
                  <Button
                    key={type.value}
                    variant={analysisType === type.value ? "default" : "outline"}
                    size="sm"
                    onClick={() => setAnalysisType(type.value)}
                    title={type.description}
                  >
                    {type.label}
                  </Button>
                ))}
              </div>

              {/* Log Input */}
              <div>
                <label className="mb-2 block text-sm font-medium">
                  Paste log content to analyze:
                </label>
                <textarea
                  value={logInput}
                  onChange={(e) => setLogInput(e.target.value)}
                  placeholder="Paste your log content here...&#10;&#10;Example:&#10;2024-01-15 10:30:45 ERROR Failed to connect to database&#10;2024-01-15 10:30:46 WARN Retrying connection...&#10;2024-01-15 10:30:50 INFO Connection established"
                  className="h-48 w-full rounded-md border bg-background p-3 font-mono text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
                />
              </div>

              {/* Analyze Button */}
              <div className="flex items-center gap-2">
                <Button onClick={handleAnalyze} disabled={isAnalyzing || !logInput.trim()}>
                  {isAnalyzing ? (
                    <>
                      <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
                      Analyzing...
                    </>
                  ) : (
                    <>
                      <Sparkles className="mr-2 h-4 w-4" />
                      Analyze Logs
                    </>
                  )}
                </Button>
                {logInput && (
                  <Button variant="ghost" size="sm" onClick={() => setLogInput("")}>
                    <Trash2 className="mr-2 h-4 w-4" />
                    Clear
                  </Button>
                )}
              </div>

              {/* Error */}
              {analysisError && (
                <div className="rounded-md border border-destructive/50 bg-destructive/10 p-4">
                  <p className="text-sm text-destructive">{analysisError}</p>
                </div>
              )}

              {/* Results */}
              {analysisResult && (
                <div className="rounded-md border bg-muted/50 p-4">
                  <div className="mb-2 flex items-center gap-2">
                    <Bot className="h-4 w-4 text-purple-500" />
                    <span className="text-sm font-medium">Analysis Result</span>
                    <Badge variant="outline" className="ml-auto">
                      {analysisTypes.find((t) => t.value === analysisType)?.label}
                    </Badge>
                  </div>
                  <div className="prose prose-sm dark:prose-invert max-w-none">
                    <pre className="whitespace-pre-wrap rounded bg-background p-3 text-sm">
                      {analysisResult}
                    </pre>
                  </div>
                </div>
              )}
            </>
          )}
        </CardContent>
      </Card>

      {/* Audit Logs */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <FileText className="h-5 w-5" />
              <CardTitle>Audit Logs</CardTitle>
              <Badge variant="secondary">{filteredLogs.length} entries</Badge>
            </div>
            <div className="flex items-center gap-2">
              <Button variant="outline" size="sm" onClick={handleExportLogs}>
                <Download className="mr-2 h-4 w-4" />
                Export
              </Button>
              <Button variant="outline" size="sm" onClick={fetchAuditLogs} disabled={logsLoading}>
                <RefreshCw className={`mr-2 h-4 w-4 ${logsLoading ? "animate-spin" : ""}`} />
                Refresh
              </Button>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          {/* Search */}
          <div className="mb-4 relative">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <input
              type="text"
              placeholder="Search logs..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="h-10 w-full max-w-md rounded-md border bg-background pl-10 pr-4 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
            />
          </div>

          {logsError && (
            <div className="mb-4 rounded-md border border-destructive/50 bg-destructive/10 p-4">
              <p className="text-sm text-destructive">{logsError}</p>
            </div>
          )}

          {logsLoading ? (
            <div className="flex items-center justify-center py-12">
              <RefreshCw className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
          ) : filteredLogs.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-12 text-center">
              <FileText className="mb-4 h-12 w-12 text-muted-foreground" />
              <p className="text-lg font-medium">No audit logs yet</p>
              <p className="text-muted-foreground">
                {searchTerm
                  ? "No logs match your search"
                  : "Logs will appear here as you use the application"}
              </p>
            </div>
          ) : (
            <div className="space-y-2">
              {filteredLogs.map((log) => (
                <div
                  key={log.id}
                  className="flex items-start gap-3 rounded-md border p-3 hover:bg-muted/50"
                >
                  {getEventIcon(log.event_type, log.success)}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="font-medium">{log.operation}</span>
                      <Badge variant="outline" className="text-xs">
                        {log.event_type.replace(/_/g, " ")}
                      </Badge>
                      {!log.success && (
                        <Badge variant="destructive" className="text-xs">
                          Failed
                        </Badge>
                      )}
                    </div>
                    <div className="mt-1 flex items-center gap-4 text-xs text-muted-foreground">
                      <span>{formatTimestamp(log.timestamp)}</span>
                      <span>User: {log.user}</span>
                      {log.service_id && <span>Service: {log.service_id}</span>}
                    </div>
                    {log.error_message && (
                      <p className="mt-1 text-sm text-destructive">{log.error_message}</p>
                    )}
                  </div>
                  <ChevronDown className="h-4 w-4 text-muted-foreground" />
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

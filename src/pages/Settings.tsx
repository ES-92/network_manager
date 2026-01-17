import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "../components/ui/card";
import { Button } from "../components/ui/button";
import { useUIStore } from "../stores/uiStore";
import { Moon, Sun, Monitor, Shield, Bot, Clock } from "lucide-react";
import type { ThemeMode } from "../lib/tauri/types";

const themeOptions: { value: ThemeMode; label: string; icon: typeof Sun }[] = [
  { value: "light", label: "Light", icon: Sun },
  { value: "dark", label: "Dark", icon: Moon },
  { value: "system", label: "System", icon: Monitor },
];

export function Settings() {
  const { theme, setTheme, refreshInterval, setRefreshInterval } = useUIStore();

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
        <p className="text-muted-foreground">
          Configure your preferences and security settings
        </p>
      </div>

      {/* Appearance */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Sun className="h-5 w-5" />
            Appearance
          </CardTitle>
          <CardDescription>
            Customize how the application looks
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div>
              <label className="text-sm font-medium">Theme</label>
              <div className="mt-2 flex gap-2">
                {themeOptions.map((option) => {
                  const Icon = option.icon;
                  return (
                    <Button
                      key={option.value}
                      variant={theme === option.value ? "default" : "outline"}
                      onClick={() => setTheme(option.value)}
                      className="flex-1"
                    >
                      <Icon className="mr-2 h-4 w-4" />
                      {option.label}
                    </Button>
                  );
                })}
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Refresh Settings */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Clock className="h-5 w-5" />
            Auto Refresh
          </CardTitle>
          <CardDescription>
            Configure automatic data refresh interval
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div>
              <label className="text-sm font-medium">Refresh Interval</label>
              <div className="mt-2 flex gap-2">
                {[5000, 10000, 30000, 60000].map((interval) => (
                  <Button
                    key={interval}
                    variant={refreshInterval === interval ? "default" : "outline"}
                    onClick={() => setRefreshInterval(interval)}
                  >
                    {interval / 1000}s
                  </Button>
                ))}
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Security */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Shield className="h-5 w-5" />
            Security
          </CardTitle>
          <CardDescription>
            Security and privacy settings
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Audit Logging</p>
                <p className="text-sm text-muted-foreground">
                  Log all service control operations
                </p>
              </div>
              <Button variant="outline" disabled>
                Enabled
              </Button>
            </div>
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Confirm Kill Operations</p>
                <p className="text-sm text-muted-foreground">
                  Require confirmation before killing processes
                </p>
              </div>
              <Button variant="outline" disabled>
                Enabled
              </Button>
            </div>
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Encrypted Configuration</p>
                <p className="text-sm text-muted-foreground">
                  Store sensitive data encrypted
                </p>
              </div>
              <Button variant="outline" disabled>
                Enabled
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Ollama Integration */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Bot className="h-5 w-5" />
            Ollama LLM Integration
          </CardTitle>
          <CardDescription>
            Configure local LLM for log analysis
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Enable Ollama</p>
                <p className="text-sm text-muted-foreground">
                  Use local LLM for intelligent log analysis
                </p>
              </div>
              <Button variant="outline">
                Disabled
              </Button>
            </div>
            <div>
              <label className="text-sm font-medium">Endpoint</label>
              <input
                type="text"
                value="http://localhost:11434"
                disabled
                className="mt-1 block w-full rounded-md border bg-muted px-3 py-2 text-sm"
              />
            </div>
            <div>
              <label className="text-sm font-medium">Model</label>
              <input
                type="text"
                value="mistral:7b-instruct"
                disabled
                className="mt-1 block w-full rounded-md border bg-muted px-3 py-2 text-sm"
              />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

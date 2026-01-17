import { Moon, Sun, RefreshCw, Search } from "lucide-react";
import { Button } from "../ui/button";
import { useUIStore } from "../../stores/uiStore";
import { useServiceStore } from "../../stores/serviceStore";
import { usePortStore } from "../../stores/portStore";

export function TopBar() {
  const { theme, setTheme } = useUIStore();
  const { fetchServices, isLoading: servicesLoading } = useServiceStore();
  const { fetchPortUsage, isLoading: portsLoading } = usePortStore();

  const isLoading = servicesLoading || portsLoading;

  const handleRefresh = async () => {
    await Promise.all([fetchServices(), fetchPortUsage()]);
  };

  const toggleTheme = () => {
    if (theme === "light") {
      setTheme("dark");
    } else if (theme === "dark") {
      setTheme("system");
    } else {
      setTheme("light");
    }
  };

  return (
    <header className="flex h-16 items-center justify-between border-b bg-card px-6">
      {/* Search */}
      <div className="flex items-center gap-2">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <input
            type="text"
            placeholder="Search services, ports..."
            className="h-10 w-64 rounded-md border bg-background pl-10 pr-4 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
          />
        </div>
      </div>

      {/* Actions */}
      <div className="flex items-center gap-2">
        {/* Refresh */}
        <Button
          variant="ghost"
          size="icon"
          onClick={handleRefresh}
          disabled={isLoading}
          aria-label="Refresh data"
        >
          <RefreshCw className={`h-5 w-5 ${isLoading ? "animate-spin" : ""}`} />
        </Button>

        {/* Theme Toggle */}
        <Button
          variant="ghost"
          size="icon"
          onClick={toggleTheme}
          aria-label={`Current theme: ${theme}. Click to toggle.`}
        >
          {theme === "dark" ? (
            <Moon className="h-5 w-5" />
          ) : theme === "light" ? (
            <Sun className="h-5 w-5" />
          ) : (
            <div className="relative h-5 w-5">
              <Sun className="absolute h-5 w-5 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
              <Moon className="absolute h-5 w-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
            </div>
          )}
        </Button>
      </div>
    </header>
  );
}

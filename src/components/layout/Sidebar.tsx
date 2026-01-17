import { Link, useLocation } from "react-router-dom";
import { cn } from "../../lib/utils";
import { useUIStore } from "../../stores/uiStore";
import {
  LayoutDashboard,
  Server,
  Network,
  Cpu,
  Shield,
  FileText,
  Settings,
  ChevronLeft,
  ChevronRight,
} from "lucide-react";

const navItems = [
  { path: "/", label: "Dashboard", icon: LayoutDashboard },
  { path: "/services", label: "Services", icon: Server },
  { path: "/ports", label: "Ports", icon: Network },
  { path: "/performance", label: "Performance", icon: Cpu },
  { path: "/security", label: "Sicherheit", icon: Shield },
  { path: "/logs", label: "Logs", icon: FileText },
  { path: "/settings", label: "Settings", icon: Settings },
];

export function Sidebar() {
  const location = useLocation();
  const { sidebarCollapsed, toggleSidebar } = useUIStore();

  return (
    <aside
      className={cn(
        "flex flex-col border-r bg-card transition-all duration-300",
        sidebarCollapsed ? "w-16" : "w-64"
      )}
    >
      {/* Logo */}
      <div className="flex h-16 items-center justify-between border-b px-4">
        {!sidebarCollapsed && (
          <span className="text-lg font-semibold text-primary">
            Network Manager
          </span>
        )}
        <button
          onClick={toggleSidebar}
          className="rounded-md p-2 hover:bg-accent"
          aria-label={sidebarCollapsed ? "Expand sidebar" : "Collapse sidebar"}
        >
          {sidebarCollapsed ? (
            <ChevronRight className="h-4 w-4" />
          ) : (
            <ChevronLeft className="h-4 w-4" />
          )}
        </button>
      </div>

      {/* Navigation */}
      <nav className="flex-1 space-y-1 p-2">
        {navItems.map((item) => {
          const isActive = location.pathname === item.path;
          const Icon = item.icon;

          return (
            <Link
              key={item.path}
              to={item.path}
              className={cn(
                "flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors",
                isActive
                  ? "bg-primary text-primary-foreground"
                  : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
              )}
            >
              <Icon className="h-5 w-5 shrink-0" />
              {!sidebarCollapsed && <span>{item.label}</span>}
            </Link>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="border-t p-4">
        {!sidebarCollapsed && (
          <p className="text-xs text-muted-foreground">
            v0.1.0 - Service Manager
          </p>
        )}
      </div>
    </aside>
  );
}

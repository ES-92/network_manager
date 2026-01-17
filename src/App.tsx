import { BrowserRouter, Routes, Route } from "react-router-dom";
import { useEffect } from "react";
import { AppLayout } from "./components/layout/AppLayout";
import { Dashboard } from "./pages/Dashboard";
import { Services } from "./pages/Services";
import { Ports } from "./pages/Ports";
import { Performance } from "./pages/Performance";
import { Security } from "./pages/Security";
import { Logs } from "./pages/Logs";
import { Settings } from "./pages/Settings";
import { useUIStore } from "./stores/uiStore";
import { useRealtime } from "./lib/hooks";
import "./index.css";

function App() {
  const { theme } = useUIStore();

  // Initialize real-time updates from the backend
  useRealtime();

  // Apply theme on mount and when it changes
  useEffect(() => {
    const root = document.documentElement;
    if (theme === "dark") {
      root.classList.add("dark");
    } else if (theme === "light") {
      root.classList.remove("dark");
    } else {
      // System preference
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      if (prefersDark) {
        root.classList.add("dark");
      } else {
        root.classList.remove("dark");
      }
    }
  }, [theme]);

  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AppLayout />}>
          <Route path="/" element={<Dashboard />} />
          <Route path="/services" element={<Services />} />
          <Route path="/ports" element={<Ports />} />
          <Route path="/performance" element={<Performance />} />
          <Route path="/security" element={<Security />} />
          <Route path="/logs" element={<Logs />} />
          <Route path="/settings" element={<Settings />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;

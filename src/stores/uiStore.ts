import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { ThemeMode } from "../lib/tauri/types";

interface UIState {
  theme: ThemeMode;
  sidebarCollapsed: boolean;
  refreshInterval: number;

  // Actions
  setTheme: (theme: ThemeMode) => void;
  toggleSidebar: () => void;
  setRefreshInterval: (ms: number) => void;
}

export const useUIStore = create<UIState>()(
  persist(
    (set) => ({
      theme: "system",
      sidebarCollapsed: false,
      refreshInterval: 5000,

      setTheme: (theme) => {
        set({ theme });
        // Apply theme to document
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
      },

      toggleSidebar: () => {
        set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed }));
      },

      setRefreshInterval: (ms) => {
        set({ refreshInterval: ms });
      },
    }),
    {
      name: "network-manager-ui",
    }
  )
);

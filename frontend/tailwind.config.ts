import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        surface: {
          DEFAULT: "#0f1117",
          1: "#161b27",
          2: "#1d2333",
          3: "#252d3d",
        },
        accent: {
          DEFAULT: "#6366f1",
          hover: "#4f52d9",
          muted: "#6366f120",
        },
        success: "#22c55e",
        warning: "#f59e0b",
        danger: "#ef4444",
        muted: "#64748b",
      },
      fontFamily: {
        mono: ["JetBrains Mono", "Fira Code", "Consolas", "monospace"],
      },
    },
  },
  plugins: [],
} satisfies Config;

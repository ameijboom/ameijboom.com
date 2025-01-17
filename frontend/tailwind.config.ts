import type { Config } from "tailwindcss";
import flyonui from "flyonui";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  plugins: [flyonui],
  flyonui: {
    themes: false,
    darkTheme: "dark"
  },
  theme: {
    fontFamily: {
      sans: ['var(--font-monaspace-neon)', 'Inter', 'sans-serif']
    }
  }
};
export default config;

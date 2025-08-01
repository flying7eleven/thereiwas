import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import viteTsconfigPaths from "vite-tsconfig-paths";
import browserslistToEsbuild from "browserslist-to-esbuild";

export default defineConfig(() => {
  return {
    define: {
      __BUILD_TIMESTAMP__: new Date().getTime(),
    },
    build: {
      outDir: "build",
      sourcemap: false,
      target: browserslistToEsbuild([">0.2%", "not dead", "not op_mini all"]),
    },
    assetsInclude: ["**/*.png", "**/*.jpg"],
    plugins: [
      react({
        jsxImportSource: "@emotion/react",
        babel: {
          plugins: ["@emotion/babel-plugin"],
        },
      }),
      viteTsconfigPaths(),
    ],
    sourcemap: true,
  };
});

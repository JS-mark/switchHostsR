import { defineConfig, mergeConfig } from "vite";
import { getBaseConfig } from "./vite-config/index";

// https://vitejs.dev/config/
export default defineConfig((env) => {
  const { command, mode, ssrBuild } = env;
  console.log("111", command, mode, ssrBuild);
  return mergeConfig(getBaseConfig(env), {}, true);
});

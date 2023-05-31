import { type App } from "vue";
import Svg from "@/components/svg.vue";

export const useGlobalComp = (app: App) => {
  app.component("SvgIcon", Svg);
};

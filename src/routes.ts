import { RouteRecordRaw } from "vue-router";
import StHome from "./views/StHome.vue";
import StCalculationTools from "./views/StCalculationTools.vue";
import StBeam from "./views/calculation-tools/StBeam.vue";

export default [{
  path: '/',
  component: StHome,
}, {
  path: '/calculation-tools',
  component: StCalculationTools,
}, {
    path: '/calculation-tools/beam',
    component: StBeam,
}] as readonly RouteRecordRaw[]

import { RouteRecordRaw } from "vue-router";
import Home from "./views/Home.vue";
import CalculationTools from "./views/CalculationTools.vue";

export default [{
  path: '/',
  component: Home,
}, {
  path: '/calculation-tools',
  component: CalculationTools,
}] as readonly RouteRecordRaw[]

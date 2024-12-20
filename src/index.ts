import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { createVuetify } from 'vuetify'
import init from '#wasm'
import routes from './routes'
import Main from './Main.vue'

const router = createRouter({
  history: createWebHistory(),
  routes,
})

const vuetify = createVuetify()

init().then(() => {
  const app = createApp(Main)
  app.use(router)
  app.use(vuetify)
  app.mount('body')
})

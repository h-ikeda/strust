import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import { createVuetify } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg'
import init from '#wasm'
import routes from './routes'
import StMain from './StMain.vue'

const router = createRouter({
  history: createWebHistory(),
   routes,
})

const vuetify = createVuetify({
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    }
  }
})

init().then(() => {
  const app = createApp(StMain)
  app.use(router)
  app.use(vuetify)
  app.mount('body')
})

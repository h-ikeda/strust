import init from 'strust'
import { createApp } from 'vue'
import Main from './Main.vue'

init().then(() => {
  const app = createApp(Main)
  app.mount('body')
})

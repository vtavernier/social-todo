import Vue from 'vue'
import { api } from '~/utils/api'

Vue.use({
  install(Vue, _options) {
    Vue.mixin({
      fetchOnServer() {
        return api.isServerBackend || (this as any).$store.state.staticRendered
      },
    })
  },
})

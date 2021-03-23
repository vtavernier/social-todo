import { Store } from 'vuex'
import { initializeStores } from '~/utils/store-accessor'
import { initializeApi } from '~/utils/api'

const initializer = (store: Store<any>) => initializeStores(store)

export const plugins = [initializer]
export * from '~/utils/store-accessor'

export const state = () => {
  return {
    staticRendered: false,
  }
}

export const mutations = {
  setStaticRendered(state: any, value: boolean) {
    state.staticRendered = value
  },
}

export const actions = {
  nuxtServerInit({ commit }: { commit: any }, { req: _req }: { req: any }) {
    const backend = process.env.SOCIAL_TODO_BACKEND
    if (backend) {
      commit('setStaticRendered', true)
      initializeApi(backend)
    }
  },
}

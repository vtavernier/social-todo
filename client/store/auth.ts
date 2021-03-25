import { Module, VuexModule, Mutation, Action } from 'vuex-module-decorators'
import { api, UserDetails, LoginRequest } from '~/utils/api'

@Module({
  name: 'auth',
  namespaced: true,
  stateFactory: true,
})
export default class AuthModule extends VuexModule {
  user: UserDetails | null = null
  loaded = false

  @Mutation
  setUser(user: UserDetails | null) {
    this.user = user
    this.loaded = true
  }

  @Action({ rawError: true })
  async fetch() {
    if (!this.loaded) {
      this.context.commit('setUser', (await api.fetchAuth()).user)
    }
  }

  @Action({ commit: 'setUser', rawError: true })
  async login(rq: LoginRequest) {
    return (await api.login(rq)).user
  }

  @Action({ commit: 'setUser', rawError: true })
  async logout() {
    await api.logout()
    return null
  }
}

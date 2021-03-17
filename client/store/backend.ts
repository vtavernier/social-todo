import { Module, VuexModule, MutationAction } from 'vuex-module-decorators'
import api from '~/utils/api'

@Module({
  name: 'backend',
  namespaced: true,
  stateFactory: true,
})
export default class BackendModule extends VuexModule {
  version: string = ''

  @MutationAction({ mutate: ['version'], rawError: true })
  async fetch() {
    return await api.fetchBackend()
  }
}

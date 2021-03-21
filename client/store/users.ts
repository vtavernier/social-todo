import {
  Module,
  VuexModule,
  MutationAction,
  Action,
  Mutation,
} from 'vuex-module-decorators'
import { api, UserDetails } from '~/utils/api'

@Module({
  name: 'users',
  namespaced: true,
  stateFactory: true,
})
export default class UsersModule extends VuexModule {
  users: Array<UserDetails> | null = null
  currentUser: UserDetails | null = null

  @MutationAction({ mutate: ['users'], rawError: true })
  async fetchAll() {
    return await api.fetchUsers()
  }

  @Action({ commit: 'setCurrentUser', rawError: true })
  async fetchCurrent({ id }: { id: number }) {
    return await api.fetchUser(id)
  }

  @Mutation
  setCurrentUser(user: UserDetails) {
    this.currentUser = user
  }
}

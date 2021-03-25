import { Store } from 'vuex'
import { getModule } from 'vuex-module-decorators'

import AuthModule from '~/store/auth'
import BackendModule from '~/store/backend'
import UsersModule from '~/store/users'

let authStore: AuthModule
let backendStore: BackendModule
let usersStore: UsersModule

function initializeStores(store: Store<any>): void {
  authStore = getModule(AuthModule, store)
  backendStore = getModule(BackendModule, store)
  usersStore = getModule(UsersModule, store)
}

export { initializeStores, authStore, backendStore, usersStore }

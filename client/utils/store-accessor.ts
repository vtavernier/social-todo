import { Store } from 'vuex'
import { getModule } from 'vuex-module-decorators'

import BackendModule from '~/store/backend'
import UsersModule from '~/store/users'

let backendStore: BackendModule
let usersStore: UsersModule

function initializeStores(store: Store<any>): void {
  backendStore = getModule(BackendModule, store)
  usersStore = getModule(UsersModule, store)
}

export { initializeStores, backendStore, usersStore }

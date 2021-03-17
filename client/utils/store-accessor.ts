import { Store } from 'vuex'
import { getModule } from 'vuex-module-decorators'

import BackendModule from '~/store/backend'

let backendStore: BackendModule

function initialiseStores(store: Store<any>): void {
  backendStore = getModule(BackendModule, store)
}

export { initialiseStores, backendStore }

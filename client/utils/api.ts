import axios from 'axios'

class Api {
  axios = axios.create()

  public async fetchBackend() {
    return (await this.axios.get('/api/v1/')).data
  }
}

export default new Api()

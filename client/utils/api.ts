import axios, { AxiosInstance } from 'axios'

export interface BackendInfo {
  version: string
}

export enum UserRole {
  Admin = 'admin',
  User = 'user',
}

export interface UserDetails {
  id: number
  name: string
  role: UserRole
  // TODO: Handle as a date
  createdAt: string
}

export interface UserList {
  users: Array<UserDetails>
}

export class Api {
  axios: AxiosInstance
  public isServerBackend: boolean

  constructor(baseURL?: string) {
    const suffix = '/api/v1/'

    if (baseURL === undefined) {
      this.axios = axios.create({ baseURL: suffix })
      this.isServerBackend = false
    } else {
      this.axios = axios.create({ baseURL: `${baseURL}${suffix}` })
      this.isServerBackend = true
    }
  }

  public async fetchBackend() {
    return (await this.axios.get('/')).data as BackendInfo
  }

  public async fetchUsers() {
    return (await this.axios.get('/users/')).data as UserList
  }

  public async fetchUser(id: number) {
    return (await this.axios.get(`/users/${id}/`)).data as UserDetails
  }
}

export let api = new Api()

export function initializeApi(baseURL?: string) {
  api = new Api(baseURL)
}

import axios from 'axios'

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

class Api {
  axios = axios.create()

  public async fetchBackend() {
    return (await this.axios.get('/api/v1/')).data as BackendInfo
  }

  public async fetchUsers() {
    return (await this.axios.get('/api/v1/users/')).data as UserList
  }

  public async fetchUser(id: number) {
    return (await this.axios.get(`/api/v1/users/${id}/`)).data as UserDetails
  }
}

export const api = new Api()

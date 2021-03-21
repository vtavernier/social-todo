<template>
  <v-row>
    <v-col cols="12" sm="8" md="6">
      <h1>List of users</h1>
      <ul v-if="users !== null">
        <li v-for="user in users" :key="user.id">
          <nuxt-link :to="`/users/${user.id}/`">{{ user.name }}</nuxt-link>
        </li>
      </ul>
      <v-skeleton-loader v-else type="list-item@3"></v-skeleton-loader>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import Vue from 'vue'
import { mapState } from 'vuex'
import { Component } from 'nuxt-property-decorator'

import { usersStore } from '~/store'

@Component({ computed: { ...mapState('users', ['users']) } })
export default class Users extends Vue {
  async mounted() {
    await usersStore.fetchAll()
  }
}
</script>

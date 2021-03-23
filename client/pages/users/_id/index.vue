<template>
  <v-row justify="center">
    <v-col cols="12" sm="8" md="6">
      <v-card v-if="currentUser">
        <v-card-title>
          {{ currentUser.name }}
        </v-card-title>

        <v-list v-if="currentUser">
          <v-list-item>
            <v-list-item-content>
              <v-list-item-title>{{ currentUser.id }}</v-list-item-title>
              <v-list-item-subtitle>Id</v-list-item-subtitle>
            </v-list-item-content>
          </v-list-item>

          <v-list-item>
            <v-list-item-content>
              <v-list-item-title>{{ currentUser.role }}</v-list-item-title>
              <v-list-item-subtitle>Role</v-list-item-subtitle>
            </v-list-item-content>
          </v-list-item>

          <v-list-item>
            <v-list-item-content>
              <v-list-item-title>{{ currentUser.createdAt }}</v-list-item-title>
              <v-list-item-subtitle>Registered</v-list-item-subtitle>
            </v-list-item-content>
          </v-list-item>
        </v-list>
      </v-card>
      <v-skeleton-loader v-else type="card"></v-skeleton-loader>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import Vue from 'vue'
import { mapState } from 'vuex'
import { Component } from 'nuxt-property-decorator'

import { usersStore } from '~/store'

@Component({ computed: { ...mapState('users', ['currentUser']) } })
export default class UserDetails extends Vue {
  async fetch() {
    await usersStore.fetchCurrent({ id: parseInt(this.$route.params.id) })
  }
}
</script>

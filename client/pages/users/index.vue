<template>
  <v-row justify="center">
    <v-col cols="12" sm="8" md="6">
      <v-card>
        <v-card-title v-text="'List of users'" />

        <v-list v-if="users !== null">
          <v-list-item
            v-for="(user, i) in users"
            :key="i"
            :to="`/users/${user.id}/`"
          >
            <v-list-item-content>
              <v-list-item-title v-text="user.name" />
              <v-list-item-subtitle v-text="user.role" />
            </v-list-item-content>
          </v-list-item>
        </v-list>

        <v-skeleton-loader v-else type="list-item@3"></v-skeleton-loader>
      </v-card>
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

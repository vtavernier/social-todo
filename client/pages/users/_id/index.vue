<template>
  <v-row>
    <v-col cols="12" sm="8" md="6">
      <h1>
        User details<template v-if="currentUser"
          >: {{ currentUser.name }}</template
        >
      </h1>
      <template v-if="currentUser">
        <dl>
          <dt>Name:</dt>
          <dd>{{ currentUser.name }}</dd>
          <dt>Role:</dt>
          <dd>{{ currentUser.role }}</dd>
        </dl>
      </template>
      <v-skeleton-loader v-else type="article"></v-skeleton-loader>
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
  async mounted() {
    await usersStore.fetchCurrent({ id: parseInt(this.$route.params.id) })
  }
}
</script>

<template>
  <v-row justify="center" align="center">
    <v-col sm="12" md="8">
      <v-card>
        <v-card-title class="headline">Welcome to social-todo</v-card-title>
        <v-card-text>
          <p>
            This app is a take on the traditional <em>to-do list</em> example,
            with users (authentication) and social features, to make for a more
            interesting challenge.
          </p>

          <p v-if="staticRendered">
            This version of the app has been rendered as static pages to be
            hosted on GitHub Pages. This means features that rely on a backend
            being present will not work, and thus this version is read-only.
            Feel free to check out the repository if you want to try it with all
            its features!
          </p>
        </v-card-text>
        <v-card-actions></v-card-actions>
      </v-card>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import Vue from 'vue'
import { mapState } from 'vuex'
import { Component } from 'nuxt-property-decorator'

import { backendStore } from '~/store'

@Component({
  computed: {
    ...mapState('backend', ['version']),
    ...mapState(['staticRendered']),
  },
})
export default class Index extends Vue {
  async fetch() {
    await backendStore.fetch()
  }
}
</script>

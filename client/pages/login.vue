<template>
  <v-row justify="center" align="center">
    <v-col sm="8" md="6">
      <v-form ref="form" @submit.prevent="submit">
        <v-card>
          <v-card-title>Login</v-card-title>
          <v-card-text>
            <v-text-field v-model="name" name="name" label="Name" />
            <v-text-field
              v-model="password"
              name="password"
              label="Password"
              type="password"
            />
          </v-card-text>
          <v-card-actions
            ><v-btn
              type="submit"
              block
              :loading="loading"
              :disabled="loading"
              color="primary"
              >Submit</v-btn
            ></v-card-actions
          >
        </v-card>
      </v-form>
    </v-col>
  </v-row>
</template>

<script lang="ts">
import Vue from 'vue'
import { Component } from 'nuxt-property-decorator'

import { authStore } from '~/store'

@Component
export default class Login extends Vue {
  name = ''
  password = ''
  loading = false

  async submit() {
    this.loading = true

    try {
      await authStore.login({ name: this.name, password: this.password })
      this.$router.push('/')
    } finally {
      this.loading = false
    }
  }
}
</script>

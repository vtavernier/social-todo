<template>
  <v-app dark>
    <v-navigation-drawer
      v-model="drawer"
      :mobile-breakpoint="0"
      :temporary="$vuetify.breakpoint.xs"
      app
    >
      <v-list>
        <v-list-item
          v-for="(item, i) in items"
          :key="i"
          :to="item.to"
          router
          exact
        >
          <v-list-item-action>
            <v-icon>{{ item.icon }}</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title v-text="item.title" />
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
    <v-app-bar app>
      <v-app-bar-nav-icon
        v-if="$vuetify.breakpoint.xs"
        @click.stop="drawer = !drawer"
      />
      <v-toolbar-title
        class="mr-2"
        style="cursor: pointer"
        @click.stop="$router.push({ path: '/' })"
        v-text="title"
      />
      <template v-if="!$vuetify.breakpoint.xs">
        <v-btn
          v-for="(item, i) in items.filter((item) => item.to !== '/')"
          :key="i"
          :to="item.to"
          router
          exact
          >{{ item.title }}</v-btn
        >
      </template>
      <v-spacer />
      <template v-if="currentUser">
        <v-avatar class="mr-2" color="primary">{{
          currentUser.name[0]
        }}</v-avatar>
        <v-btn @click.stop="logout">Logout</v-btn>
      </template>
      <template v-else>
        <v-btn to="/login" router exact>Login</v-btn>
      </template>
    </v-app-bar>
    <v-main>
      <v-container>
        <nuxt />
      </v-container>
    </v-main>
    <v-footer app>
      <span
        >&copy; {{ new Date().getFullYear() }}
        <a
          href="https://github.com/vtavernier"
          target="_blank"
          rel="noopener noreferrer"
          >@vtavernier</a
        ></span
      >
    </v-footer>
  </v-app>
</template>

<script>
import { authStore } from '~/store'

export default {
  data() {
    return {
      drawer: false,
      items: [
        {
          icon: 'mdi-apps',
          title: 'Home',
          to: '/',
        },
        {
          icon: 'mdi-account-multiple',
          title: 'Users',
          to: '/users',
        },
      ],
      title: 'social-todo',
    }
  },

  computed: {
    currentUser() {
      return authStore.user
    },
  },

  async mounted() {
    await authStore.fetch()
  },

  methods: {
    async logout() {
      await authStore.logout()
      this.$router.push('/')
    },
  },
}
</script>

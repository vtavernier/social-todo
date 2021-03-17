# [social-todo-client](https://github.com/vtavernier/social-todo/tree/master/client)

![Client Status](https://github.com/vtavernier/social-todo/workflows/client/badge.svg)

Vue frontend using [Nuxt.js](https://nuxtjs.org) for
[social-todo](https://github.com/vtavernier/social-todo).

## Usage

```bash
# install dependencies
$ yarn install

# serve with hot reload at localhost:3000
$ yarn dev

# build for production and launch server
$ yarn build
$ yarn start

# generate static project
$ yarn generate
```

## History

This frontend was created using the following steps:

```bash
$ yarn create nuxt-app client
# [...]
✨  Generating Nuxt.js project in client
? Project name: social-todo
? Programming language: TypeScript
? Package manager: Yarn
? UI framework: Vuetify.js
? Nuxt.js modules: Axios - Promise based HTTP client
? Linting tools: ESLint, Prettier
? Testing framework: Jest
? Rendering mode: Universal (SSR / SSG)
? Deployment target: Static (Static/JAMStack hosting)
? Development tools: (Press <space> to select, <a> to toggle all, <i> to invert selection)
? Continuous integration: GitHub Actions (GitHub only)
? What is your GitHub username? vtavernier
? Version control system: Git
# [...]
```

And then:

* Remove `@nuxt/typescript-runtime`
  (https://github.com/nuxt/create-nuxt-app/issues/719)
* Remove the READMEs in all the subfolders (`find -mindepth 2 -name README.md -! -path './node_modules/*' -delete`)
* Change `test/Logo.spec.js` to `test/Logo.spec.ts` to check that TypeScript
  and tests are working. Requires:
  * Adding the proper `testRegex` to `jest.config.js`
    (https://vue-test-utils.vuejs.org/guides/using-with-typescript.html#placing-test-files)
  * Add `@types/jest` dependency (`yarn add -D @types/jest`) and register it in
    the TypeScript config (`tsconfig.json`)
  * Fix the `lint:js` script by adding `.ts` in `package.json`
  * Add `nuxt-property-decorator` (`yarn add nuxt-property-decorator`) for
    TypeScript in Vue components
  * Add the shim for Vue components (`types/vue-shim.d.ts`)
  * Add `vuex-module-decorators` (https://typescript.nuxtjs.org/cookbook/store/)
  * Follow instructions from
    https://github.com/championswimmer/vuex-module-decorators#accessing-modules-with-nuxtjs
    to add relevant accessors
* Add `@nuxtjs/proxy` and relevant config in `nuxt.config.js` to forward API
  requests to the backend in development mode

## Author

Vincent Tavernier <vince.tavernier@gmail.com>

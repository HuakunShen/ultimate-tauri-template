// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  css: ["element-plus/theme-chalk/dark/css-vars.css"],
  ssr: false,
  modules: ["@element-plus/nuxt", "@nuxtjs/tailwindcss", "@vueuse/nuxt"],
  elementPlus: {
    /** Options */
  },
});

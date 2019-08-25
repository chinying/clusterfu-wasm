import Vue from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import "./registerServiceWorker";
const VueGoogleMaps = require('vue2-google-maps')

Vue.config.productionTip = false;
Vue.use(VueGoogleMaps, {
  load: {
    key: process.env.VUE_APP_GOOGLE_MAPS_API_KEY
  }
})

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount("#app");

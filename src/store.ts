import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    dataWithCoordinates: [] as Array<Array<string>>
  },
  mutations: {
    setDataWithCoordinates (state, val: Array<Array<string>>) {
      state.dataWithCoordinates = val
    }
  },
  actions: {}
});

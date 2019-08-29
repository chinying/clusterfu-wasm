import Vue from "vue";
import Vuex from "vuex";

import { ClusterResponse } from './types/cluster'

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    dataWithCoordinates: [] as Array<Array<string>>,
    clusters: [] as Array<ClusterResponse>
  },
  mutations: {
    setDataWithCoordinates (state, val: Array<Array<string>>) {
      state.dataWithCoordinates = val
    },
    setClusters (state, val: Array<ClusterResponse>)  {
      state.clusters = val
    }
  },
  actions: {}
});

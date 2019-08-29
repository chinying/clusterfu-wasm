<template>
  <div>
    <GmapCircle
      :center="center"
      :radius="computeRadius(population, radiusMultiplier)"
      :options="options()"
      v-on="$listeners"
    ></GmapCircle>

  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { WeightedClusterCenter } from '../types/cluster'
const VueGoogleMaps = require('vue2-google-maps')

export default Vue.extend({
  name: 'ClusterOfPoints',
  components: {GMapCircle: VueGoogleMaps.Circle},
  props: {
    'radiusMultiplier': Number,
    'color': String, 
    'center': Object,
    'population': Number
  },
  methods: {
    options () {
      return {
        strokeOpacity: 0.35,
        strokeColor: '#000',
        fillColor: this.$props.color || '#35b200',
        fillOpacity: 0.5
      }
    },

    computeRadius (score: number, radius: number) {
      return Math.sqrt(score) * radius
    }
  }
})
</script>

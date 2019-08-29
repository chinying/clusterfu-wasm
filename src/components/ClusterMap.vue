<template>
  <div>
    <div id="map">
      <GmapMap
        :center="{lat:1.3521, lng:103.8198}"
        :zoom="12"
        map-type-id="roadmap"
        style="width: 100%; height: 100vh;"
        :options="{styles: mapConfig}"
        id="gmap"
        ref="mapRef"
      >
        <ClusterOfPoints
          v-for="(origin, index) in clusters"
          :key="`origin_${index}`"
          :center="origin.center"
          :radiusMultiplier="50"
          :population="origin.weight"
        ></ClusterOfPoints> 
      </GmapMap>
    </div>
    <div id="floating_menu">
      something
      <p v-for="origin in clusters"
      :key="JSON.stringify(origin.center)">
        {{ origin.center.lat + ',' + origin.center.lng + ' - ' +  origin.weight }}
      </p>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import ClusterOfPoints from './Cluster.vue'
const VueGoogleMaps = require('vue2-google-maps')
const mapConfig = require('./mapStyles.json')

import { ClusterResponse, WeightedClusterCenter } from '../types/cluster'
import { XYToLatLng } from '../utils'

export default Vue.extend({
  name: 'ClusterMap',
  components: {
    GmapMap: VueGoogleMaps.Map,
    GmapMarker: VueGoogleMaps.Marker,
    GmapCircle: VueGoogleMaps.Circle,
    ClusterOfPoints
  },
  data () {
    return {
      mapConfig
    }
  },
  computed: {
    points (): Array<Array<string>> {
      return this.$store.state.dataWithCoordinates
    },
    clusters(): Array<WeightedClusterCenter> {
      return this.$store.state.clusters
        .map((cluster: ClusterResponse) => {
          let coordinates = XYToLatLng(cluster.x, cluster.y)
          return {
            center: {
              lat: Number(coordinates.lat),
              lng: Number(coordinates.lng),
            },
            weight: cluster.weight
          }
        })
    }
  }
})
</script>

<style lang="scss" scoped>
#floating_menu {
  position: absolute;
  text-align: left;
  top: 100px;
  min-height: 40vh;
  max-height: 80vh;
  width: 30%;
  background: white;
  overflow-x: scroll !important;
  word-break: break-word;
}
</style>
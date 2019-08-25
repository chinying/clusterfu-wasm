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
      ></GmapMap>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
const VueGoogleMaps = require('vue2-google-maps')
const mapConfig = require('./mapStyles.json')

export default Vue.extend({
  name: 'ClusterMap',
  components: {
    GmapMap: VueGoogleMaps.Map,
    GmapMarker: VueGoogleMaps.Marker,
    GmapCircle: VueGoogleMaps.Circle,
  },
  data () {
    return {
      mapConfig
    }
  }
})
</script>
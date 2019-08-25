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
    <div id="floating_menu">
      something
      {{ points }}
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
  },
  computed: {
    points (): Array<Array<string>> {
      return this.$store.state.dataWithCoordinates
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
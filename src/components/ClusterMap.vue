<template>
  <div>
    <div id="map">
      <GmapMap
        :center="{ lat: 1.3521, lng: 103.8198 }"
        :zoom="12"
        map-type-id="roadmap"
        style="width: 100%; height: 100vh;"
        :options="{ styles: mapConfig }"
        id="gmap"
        ref="mapRef"
      >
        <ClusterOfPoints
          v-for="(destination, index) in destinationClusters"
          :key="`destination_${index}`"
          :center="destination.center"
          :radiusMultiplier="50"
          :population="destination.weight"
          @click="toggleDestination(destination)"
        ></ClusterOfPoints>
      </GmapMap>
    </div>
    <div id="floating_menu">
      something
      <p v-for="destination in destinationClusters" :key="JSON.stringify(destination.center)">
        {{
          destination.center.lat + "," + destination.center.lng + " - " + destination.weight
        }}
      </p>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import ClusterOfPoints from "./Cluster.vue";
const VueGoogleMaps = require("vue2-google-maps");
const mapConfig = require("./mapStyles.json");

import { ClusterResponse, WeightedClusterCenter } from "../types/cluster";
import { XYToLatLng } from "../utils";

export default Vue.extend({
  name: "ClusterMap",
  components: {
    GmapMap: VueGoogleMaps.Map,
    GmapMarker: VueGoogleMaps.Marker,
    GmapCircle: VueGoogleMaps.Circle,
    ClusterOfPoints
  },
  data() {
    return {
      mapConfig,
      selectedOrigins: {},
      selectedDestinations: {},
      originClusters: [] as Array<WeightedClusterCenter>
    };
  },
  methods: {
    async toggleDestination (cluster: WeightedClusterCenter) {
      let selectedDestinations = this.$data.selectedDestinations
      this.$data.originClusters = []
      this.$data.selectedOrigins = {}
      // toggle selected origin
      const destKey = `${cluster.x}_${cluster.y}`
      if (!(destKey in selectedDestinations)) {
        this.$set(selectedDestinations, destKey, cluster)
        let latlng = XYToLatLng(cluster.x, cluster.y)
        // only set but don't delete since they don't need to be recomputed
        // this.$set(this.$data.clusterNames, destKey, await this.reverseGeocode(latlng.lat, latlng.lng))
      } else {
        this.$delete(selectedDestinations, destKey)
      }

      // // call destination clusters
      this.findOriginClusters()
      // // recompute suggestions based on new destinations
      // this.suggestions()
    },
    // https://stackoverflow.com/questions/50930796/how-to-get-typescript-method-callback-working-in-vuejs
    findOriginClusters: _.debounce(async function(this: any) {
      const clusters = this.$data.selectedDestinations
      console.log(clusters)
    }, 300),
  },
  computed: {
    points(): Array<Array<string>> {
      return this.$store.state.dataWithCoordinates;
    },
    destinationClusters(): Array<WeightedClusterCenter> {
      return this.$store.state.destinationClusters.map((cluster: ClusterResponse) => {
        let coordinates = XYToLatLng(cluster.x, cluster.y);
        return {
          x: cluster.x,
          y: cluster.y,
          center: {
            lat: Number(coordinates.lat),
            lng: Number(coordinates.lng)
          },
          weight: cluster.weight
        };
      });
    }
  }
});
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

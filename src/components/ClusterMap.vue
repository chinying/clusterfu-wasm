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
          :population="destination.normalizedWeight"
          :color="'mediumaquamarine'"
          @click="toggleDestination(destination)"
        ></ClusterOfPoints>

        <ClusterOfPoints
          v-for="(origin, index) in originClusters"
          :key="`origin_${index}`"
          :center="origin.center"
          :radiusMultiplier="50"
          :population="origin.normalizedWeight"
          :color="'crimson'"
          @click="toggleOrigin(origin)"
        ></ClusterOfPoints>
      </GmapMap>
    </div>
    <div id="floating_menu">
      <h5 class="m-1 collapse-header" id="destination-cluster-header">
        Destination Cluster(s)
      </h5>
      <ul
        v-for="dest in selectedDestinationsArray"
        :key="dest.x + '_' + dest.y"
      >
        <li>
          <span class="deletion-hover"
            ><a href="#" @click.prevent="toggleDestination(dest)"
              >Remove</a
            ></span
          >
          <a href="#" @click="panMap(dest.coords.lat, dest.coords.lng)">
            <span v-if="`clusterNames[${dest.x}_${dest.y}] !== undefined`">{{
              clusterNames[`${dest.x}_${dest.y}`]
            }}</span>
            <span v-else>{{ dest.coords.lat }} {{ dest.coords.lng }}</span>
          </a>
        </li>
      </ul>

      <h5 class="m-1 collapse-header" id="origin-cluster-header">
        Origin Cluster(s)
      </h5>
      <p v-if="suggestionsSize > 0">{{ suggestionsSize }} suggestions</p>
      <div
        v-for="(suggestedGroup, groupTime) in suggestions()"
        :key="groupTime"
      >
        <p style="font-weight: 700; text-align: left; padding-left: 1em;">
          {{ groupTime }}
        </p>
        <p
          v-for="(row, suggestionIndex) in suggestedGroup"
          :key="suggestionIndex"
          class="mx-4"
        >
          {{ formatSuggestion(row) | oneline }}
        </p>
      </div>
      <!-- <p v-if="selectedDestinationsArray.length > 0 && selectedOriginsArray.length === 0">
        Please select a red cluster on the map
      </p> -->
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import ClusterOfPoints from "./Cluster.vue";
const VueGoogleMaps = require("vue2-google-maps");
// import * as VueGoogleMaps from 'vue2-google-maps'
const mapConfig = require("./mapStyles.json");
import * as _ from "lodash";
import {
  point,
  Point as TurfPoint,
  distance as turfDistance,
  Feature
} from "@turf/turf";

import { ClusterResponse, WeightedClusterCenter } from "../types/cluster";
import { XYToLatLng, mapToArray } from "../utils";

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
      originClusters: [],
      filteredOD: []
    };
  },
  methods: {
    async toggleDestination(cluster: WeightedClusterCenter) {
      let selectedDestinations = this.$data.selectedDestinations;
      this.$data.originClusters = [];
      this.$data.selectedOrigins = {};
      // toggle selected origin
      const destKey = `${cluster.x}_${cluster.y}`;
      if (!(destKey in selectedDestinations)) {
        this.$set(selectedDestinations, destKey, cluster);
        let latlng = XYToLatLng(cluster.x, cluster.y);
        // only set but don't delete since they don't need to be recomputed
        // this.$set(this.$data.clusterNames, destKey, await this.reverseGeocode(latlng.lat, latlng.lng))
      } else {
        this.$delete(selectedDestinations, destKey);
      }

      // // call destination clusters
      this.findOriginClusters();
      // // recompute suggestions based on new destinations
      // this.suggestions()
    },

    async toggleOrigin(cluster: WeightedClusterCenter) {
      let selectedOrigins = this.$data.selectedOrigins;
      // toggle selected origin
      const originKey = `${cluster.x}_${cluster.y}`;
      if (!(originKey in selectedOrigins)) {
        this.$set(selectedOrigins, originKey, cluster);
      } else {
        this.$delete(selectedOrigins, originKey);
      }
    },

    suggestions() {
      let selectedOrigins = this.$data.selectedOrigins;
      let clusterDistance = 300;
      if (Object.keys(selectedOrigins).length > 0) {
        const originArray = Object.values(
          this.$data.selectedOrigins
        ) as WeightedClusterCenter[];
        const clusters = originArray.map((c: WeightedClusterCenter) =>
          point([c.center.lng, c.center.lat])
        );
        let isNearSomeClusterCentre = this.isNearSomeClusterCentreFactory(
          clusters,
          clusterDistance,
          1,
          2
        );
        const suggestions = this.$data.filteredOD.filter(
          isNearSomeClusterCentre
        );

        if (suggestions.length > 0 && this.$data.timeColIndex !== null) {
          return _.groupBy(suggestions, p => p[this.$data.timeColIndex]);
        } else return { Uncategorised: suggestions.map((s: any) => s) };
      } else {
        return {};
      }
    },

    // https://stackoverflow.com/questions/50930796/how-to-get-typescript-method-callback-working-in-vuejs
    findOriginClusters: _.debounce(async function(this: any) {
      const destinationArray = Object.values(
        this.$data.selectedDestinations
      ) as WeightedClusterCenter[];
      const clusters = destinationArray.map((c: WeightedClusterCenter) =>
        point([c.center.lng, c.center.lat])
      );

      const clusterDistance = 300; // FIXME: user input

      let isNearSomeClusterCentre = this.isNearSomeClusterCentreFactory(
        clusters,
        clusterDistance,
        3,
        4
      );
      const entriesWithinCluster = this.csvPoints.filter(
        isNearSomeClusterCentre
      );

      this.$data.filteredOD = entriesWithinCluster;
      // avoid calling /cluster endpoint if there are no clusters left
      if (entriesWithinCluster.length === 0) {
        // clear origin clusters if any
        this.$data.originClusters = [];
        // empty the selectedOrigins object
        for (let key in this.$data.selectedOrigins) {
          if (this.$data.selectedOrigins.hasOwnProperty(key)) {
            this.$delete(this.$data.selectedOrigins, key);
          }
        }
        return;
      }

      // we post the origins associated with these filtered OD pairs
      // let postToClustering = entriesWithinCluster.map((point: any) => {
      //   return {x: parseFloat(point[1]), y: parseFloat(point[2]), weight: 1}
      // })
      let postToClustering = entriesWithinCluster.reduce(
        (acc: Array<Array<number>>, point: any) => {
          acc[0].push(parseFloat(point[1]));
          acc[1].push(parseFloat(point[2]));
          acc[2].push(1);
          return acc;
        },
        [[], [], []]
      );

      const originClusters = await this.computeClusters(
        postToClustering,
        clusterDistance
      );
      this.$data.originClusters = originClusters;
    }, 300),

    async computeClusters(
      points: Array<Array<number>>,
      clusterDistance: number
    ) {
      const wasm = await import("clusterfu-binary");

      try {
        // this.$store.commit('setLoading', true)
        // this.$store.commit('setLoaderText', 'Computing clusters')
        const clusterResults = wasm.cluster(
          Float64Array.from(points[0]),
          Float64Array.from(points[1]),
          Float64Array.from(points[2]),
          clusterDistance
        );

        const clusters = JSON.parse(clusterResults).map((cluster: string) =>
          JSON.parse(cluster)
        ) as Array<ClusterResponse>;

        let weights = clusters.map(v => v.weight);
        let ratio = Math.max(...weights) / 100;
        const what = clusters.map(el => {
          const normalizedWeight = Math.round(el.weight / ratio) + 3; // +3 as base value
          const latlng = XYToLatLng(el.x, el.y);
          const center = {
            lat: latlng.lat,
            lng: latlng.lng
          };
          return {
            x: el.x,
            y: el.y,
            weight: el.weight,
            normalizedWeight,
            center
          };
        });
        return what;
      } catch (err) {
        console.error(err);
      } finally {
        // this.$store.commit('setLoading', false)
        // this.$store.dispatch('clearLoaderText')
      }
    },

    isNearSomeClusterCentreFactory(
      clusters: Feature<TurfPoint>[],
      clusterDistance: number,
      xIndex: number,
      yIndex: number
    ) {
      return (point: any[]) => {
        // convert points from xy pairs to turf.point object
        let p = this.pointsFactory(point, xIndex, yIndex);
        if (p === null) return false;
        let distances = clusters.map(c =>
          turfDistance(p, c, { units: "kilometres" })
        );
        return distances.some((dist: number) => dist < clusterDistance);
      };
    },

    pointsFactory(_point: Array<any>, xIndex: number, yIndex: number) {
      let [destX, destY] = [xIndex, yIndex].map(i => parseFloat(_point[i]));
      let p = XYToLatLng(destX, destY);
      return point([p.lng, p.lat]);
    },

    panMap(lat: number, lng: number) {
      this.$refs.mapRef.panTo({ lat, lng });
    },

    formatSuggestion(arr: Array<any>) {
      // only return Origin postal, timing, etc.
      return [arr[5]].concat(arr.slice(7));
    }
  },
  computed: {
    csvPoints(): Array<Array<string>> {
      return this.$store.state.dataWithCoordinates;
    },
    destinationClusters(): Array<WeightedClusterCenter> {
      const clusters = this.$store.state.destinationClusters;
      const weights = clusters.map((c: ClusterResponse) => c.weight);
      const ratio = Math.max(...weights) / 100;
      return clusters.map((cluster: ClusterResponse) => {
        let coordinates = XYToLatLng(cluster.x, cluster.y);
        const normalizedWeight = Math.round(cluster.weight / ratio) + 3; // +3 as base value

        return {
          x: cluster.x,
          y: cluster.y,
          center: {
            lat: Number(coordinates.lat),
            lng: Number(coordinates.lng)
          },
          weight: cluster.weight,
          normalizedWeight
        };
      });
    },

    selectedOriginsArray(): Array<WeightedClusterCenter> {
      return mapToArray(this.$data.selectedOrigins)
    },

    selectedDestinationsArray(): Array<WeightedClusterCenter> {
      return mapToArray(this.$data.selectedDestinations)
    },

    suggestionsSize(): number {
      return _.flatMap(this.suggestions()).length;
    }
  },

  filters: {
    oneline(arr: Array<string>) {
      return arr.filter(a => a !== "").join(", ");
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

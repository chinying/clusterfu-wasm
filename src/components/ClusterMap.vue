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

        <GmapMarker
          v-for="origin in selectedOriginsArray"
          :key="origin.x + ',' + origin.y"
          :position="{ lat: origin.center.lat, lng: origin.center.lng }"
        ></GmapMarker>
        <GmapMarker
          v-for="dest in selectedDestinationsArray"
          :key="dest.x + ',' + dest.y"
          :position="{ lat: dest.center.lat, lng: dest.center.lng }"
        ></GmapMarker>
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
          <a href="#" @click="panMap(dest.center.lat, dest.center.lng)">
            <span v-if="`clusterNames[${dest.x}_${dest.y}] !== undefined`">{{
              clusterNames[`${dest.x}_${dest.y}`]
            }}</span>
            <span v-else>{{ dest.center.lat }} {{ dest.center.lng }}</span>
          </a>
        </li>
      </ul>

      <h5 class="m-1 collapse-header" id="origin-cluster-header">
        Origin Cluster(s)
      </h5>
      <ul
        v-for="origin in selectedOriginsArray"
        :key="origin.x + '_' + origin.y"
      >
        <li>
          <span class="deletion-hover"
            ><a href="#" @click.prevent="toggleOrigin(origin)">Remove</a></span
          >
          <a href="#" @click="panMap(origin.center.lat, origin.center.lng)">
            <span
              v-if="`clusterNames[${origin.x}_${origin.y}] !== undefined`"
              >{{ clusterNames[`${origin.x}_${origin.y}`] }}</span
            >
            <span v-else>{{ origin.center.lat }} {{ origin.center.lng }}</span>
          </a>
        </li>
      </ul>

      <p v-if="suggestionsSize > 0">{{ suggestionsSize }} suggestions</p>
      <div
        v-for="(suggestedGroup, groupTime) in suggestions()"
        :key="groupTime"
      >
        <p
          v-if="groupTime"
          style="font-weight: 700; text-align: left; padding-left: 1em;"
        >
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
import * as _ from "lodash";
import {
  point,
  Point as TurfPoint,
  distance as turfDistance,
  Feature
} from "@turf/turf";
import ClusterOfPoints from "./Cluster.vue";

import { ClusterResponse, WeightedClusterCenter } from "../types/cluster";
import { XYToLatLng, mapToArray } from "../utils";

const VueGoogleMaps = require("vue2-google-maps");
// import * as VueGoogleMaps from 'vue2-google-maps'
const mapConfig = require("./mapStyles.json");

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
      selectedOrigins: {} as { [key: string]: WeightedClusterCenter },
      selectedDestinations: {} as { [key: string]: WeightedClusterCenter },
      originClusters: [] as Array<WeightedClusterCenter>,
      filteredOD: [],
      clusterNames: {}
    };
  },
  methods: {
    async toggleDestination(cluster: WeightedClusterCenter) {
      const { selectedDestinations } = this.$data;
      this.$data.originClusters = [];
      this.$data.selectedOrigins = {};
      // toggle selected origin
      const destKey = `${cluster.x}_${cluster.y}`;
      if (!(destKey in selectedDestinations)) {
        this.$set(selectedDestinations, destKey, cluster);
        const latlng = XYToLatLng(cluster.x, cluster.y);
        // only set but don't delete since they don't need to be recomputed
        this.$set(
          this.$data.clusterNames,
          destKey,
          `${latlng.lat}_${latlng.lng}`
        );
      } else {
        this.$delete(selectedDestinations, destKey);
      }

      // // call destination clusters
      this.findOriginClusters();
      // // recompute suggestions based on new destinations
      this.suggestions();
    },

    async toggleOrigin(cluster: WeightedClusterCenter) {
      const { selectedOrigins } = this.$data;
      // toggle selected origin
      const originKey = `${cluster.x}_${cluster.y}`;
      if (!(originKey in selectedOrigins)) {
        this.$set(selectedOrigins, originKey, cluster);
      } else {
        this.$delete(selectedOrigins, originKey);
      }
    },

    suggestions() {
      const { selectedOrigins } = this.$data;
      const clusterDistance = 300;
      if (Object.keys(selectedOrigins).length > 0) {
        const originArray = Object.values(
          this.$data.selectedOrigins
        ) as WeightedClusterCenter[];
        const clusters = originArray.map((c: WeightedClusterCenter) =>
          point([c.center.lng, c.center.lat])
        );
        const isNearSomeClusterCentre = this.isNearSomeClusterCentreFactory(
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
        }
        return { Uncategorised: suggestions.map((s: any) => s) };
      }
      return {};
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

      const isNearSomeClusterCentre = this.isNearSomeClusterCentreFactory(
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
        for (const key in this.$data.selectedOrigins) {
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
      const postToClustering = entriesWithinCluster.reduce(
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
      const startTime = Date.now();
      console.log("started cluster fn");
      const wasm = await import("clusterfu-binary");
      console.log("loaded module", Date.now() - startTime);
      try {
        // this.$store.commit('setLoading', true)
        // this.$store.commit('setLoaderText', 'Computing clusters')
        const clusterResults = wasm.cluster(
          Float64Array.from(points[0]),
          Float64Array.from(points[1]),
          Float64Array.from(points[2]),
          clusterDistance
        );
        console.log("finished calling cluster", Date.now() - startTime);

        const clusters = JSON.parse(clusterResults).map((cluster: string) =>
          JSON.parse(cluster)
        ) as Array<ClusterResponse>;

        const weights = clusters.map(v => v.weight);
        const ratio = Math.max(...weights) / 100;
        const weightedClusters = clusters.map(el => {
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
        console.log("finished formatting response", Date.now() - startTime);

        return weightedClusters;
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
        const p = this.pointsFactory(point, xIndex, yIndex);
        if (p === null) return false;
        const distances = clusters.map(c =>
          turfDistance(p, c, { units: "metres" })
        );
        return distances.some((dist: number) => dist < clusterDistance);
      };
    },

    pointsFactory(_point: Array<any>, xIndex: number, yIndex: number) {
      const [destX, destY] = [xIndex, yIndex].map(i => parseFloat(_point[i]));
      const p = XYToLatLng(destX, destY);
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
        const coordinates = XYToLatLng(cluster.x, cluster.y);
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
      return mapToArray(this.$data.selectedOrigins);
    },

    selectedDestinationsArray(): Array<WeightedClusterCenter> {
      return mapToArray(this.$data.selectedDestinations);
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

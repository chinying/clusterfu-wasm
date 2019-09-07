<template>
  <div class="hello">
    Distance <input type="text" v-model="radius" />
    <button @click="computeClusters"></button>

    <br />
    <input
      type="file"
      ref="fileInput"
      name="fileUploadField"
      class="input-file"
      @change="filesChange($event)"
      accept=".csv, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, application/vnd.ms-excel"
    />
    <!-- <button @click="browse()">Browse</button> -->
    <button @click="upload()">Upload</button>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import axios from "axios";

import * as Papa from "papaparse";
import unzip from "lodash/unzip";
import { ClusterResponse } from "../types/cluster";

export default Vue.extend({
  name: "Upload",
  props: {
    msg: String
  },
  data() {
    return {
      radius: 300,
      uploadedFile: undefined as File | undefined,
      fileContents: [],
      points: [] as any[] // FIXME
    };
  },
  mounted() {},
  methods: {
    async hi() {
      console.log("hi");
      const wasm = await import("clusterfu-binary");
      wasm.greet("hello");
      const a = wasm.add(this.$data.radius);
      console.log(a);
      // wasm.cluster(32.1)
    },

    filesChange(e: any) {
      // type: Event?
      this.$data.uploadedFile = e.target.files[0];
      if (this.$data.uploadedFile === undefined) {
        // no file selected
        // this.reset()
        return;
      }
      const self = this;
      Papa.parse(this.$data.uploadedFile, {
        complete(results: Papa.ParseResult) {
          self.$data.fileContents = results.data;
        },
        error(err) {
          console.error(err);
        }
      });
    },

    browse() {
      (this.$refs.fileInput as HTMLElement).click();
    },

    async upload() {
      try {
        const formData = new FormData();
        /*
            Add the form data we need to submit
        */
        formData.append("file", this.$data.uploadedFile);
        const resp = await axios.post("http://localhost:7000/upload", {
          rows: this.$data.fileContents
        });
        this.$data.points = resp.data;
        console.log(resp.data);
        console.log("got response");
        this.$store.commit("setDataWithCoordinates", resp.data);
      } catch (err) {
        console.error(err);
      }
    },

    async computeClusters() {
      console.time("cluster");
      const wasm = await import("clusterfu-binary");
      console.timeLog("cluster", "module loaded");
      const { points } = this.$data;
      const unzipped = unzip(points);
      console.log("unzipped", unzipped);
      const [xArray, yArray] = [unzipped[3], unzipped[4]] as [
        number[],
        number[]
      ];
      const weightsArray: number[] = new Array(xArray.length).fill(1);
      console.timeLog("cluster", "lodash unzip array for input");
      const clusterResults = wasm.cluster(
        Float64Array.from(xArray),
        Float64Array.from(yArray),
        Float64Array.from(weightsArray),
        this.$data.radius
      );
      console.timeLog("cluster", "feed input, run wasm.cluster");
      const clusters = JSON.parse(clusterResults).map((cluster: string) =>
        JSON.parse(cluster)
      ) as Array<ClusterResponse>;

      this.$store.commit("setDestinationClusters", clusters);
      console.timeLog("cluster");
      this.$router.push({ name: "map" });
    }
  }
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>

<template>
  <div class="hello">
    Distance <input type="text" v-model="radius" />
    <button @click="computeClusters"></button>

    <br />
    <input type="file"
      ref="fileInput"
      name="fileUploadField"
      class="input-file"
      @change="filesChange($event)"
      accept=".csv, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, application/vnd.ms-excel" />
    <!-- <button @click="browse()">Browse</button> -->
    <button @click="upload()">Upload</button>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import axios from 'axios'

import * as Papa from 'papaparse'
import unzip from 'lodash/unzip'

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
      clusters: [] as any[] // FIXME
    }
  },
  mounted() {
  },
  methods: {
    async hi () {
      console.log('hi')
      const wasm = await import('clusterfu-binary')
      wasm.greet('hello')
      const a = wasm.add(this.$data.radius)
      console.log(a)
      // wasm.cluster(32.1)
    },

    filesChange (e: any) { // type: Event?
      this.$data.uploadedFile = e.target.files[0]
      if (this.$data.uploadedFile === undefined) { // no file selected
        // this.reset()
        return
      }
      let self = this
      Papa.parse(this.$data.uploadedFile, {
        complete: function(results: Papa.ParseResult) {
          self.$data.fileContents = results.data
        },
        error: function(err) {
          console.error(err)
        }
      })
    },

    browse() {
      (this.$refs.fileInput as HTMLElement).click()
    },

    async upload() {
      try {
        let formData = new FormData();
        /*
            Add the form data we need to submit
        */
        formData.append('file', this.$data.uploadedFile);
        let resp = await axios.post('http://localhost:7000/upload',  { rows: this.$data.fileContents })
        this.$data.clusters = resp.data
        console.log(resp.data)
        console.log('got response')
        this.$store.commit('setDataWithCoordinates', resp.data)
      } catch (err) {
        console.error(err)
      }
    },

    async computeClusters() {
      const wasm = await import('clusterfu-binary')
      let clusters = this.$data.clusters
      let unzipped = unzip(clusters)
      let [xArray, yArray] = [unzipped[1], unzipped[2]] as [number[], number[]]
      // let [xArray, yArray] = [[10.1, 17.8, 10.2, 32.1, 32.1], [9.9, 10.0, 17.8, 13.2, 13.2]]
      let weightsArray: number[] = new Array(xArray.length).fill(1)
      console.log(Float64Array.from( weightsArray))
      const results = wasm.cluster(Float64Array.from(xArray), Float64Array.from(yArray), Float64Array.from(weightsArray), 100.0)
      console.log(results)
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

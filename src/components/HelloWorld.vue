<template>
  <div class="hello">
    Distance <input type="text" v-bind="radius" />
    <input type="text" />
    <button @click="hi"></button>

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

export default Vue.extend({
  name: "HelloWorld",
  props: {
    msg: String
  },
  data() {
    return {
      radius: 300,
      uploadedFile: undefined as File | undefined,
      fileContents: []
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
        complete: function(results: any) {
          console.log(results)
          console.log(self.$data)
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
      let formData = new FormData();
      /*
          Add the form data we need to submit
      */
      formData.append('file', this.$data.uploadedFile);
      let resp = await axios.post('http://localhost:7000/upload',  { rows: this.$data.fileContents })
      console.log(resp)
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

<template>
  <div class="hello">
      whodis
  </div>
</template>

<script lang="ts">
import Vue from "vue";

import { greet } from 'clusterfu-binary'

export default Vue.extend({
  name: "HelloWorld",
  props: {
    msg: String
  },
  mounted() {
    greet('hello')
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

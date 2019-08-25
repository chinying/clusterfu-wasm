<template>
  <div class="home">
    <UploadPanel />
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import UploadPanel from "@/components/UploadPanel.vue"; // @ is an alias to /src

export default Vue.extend({
  name: "home",
  components: {
    UploadPanel
  }
});
</script>

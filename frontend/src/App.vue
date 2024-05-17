<template>
  <div class="d-flex h-100 w-100">
    <div class="side">
      <Sidebar :menuItems="menu"/>
    </div>
    <div class="content">
      <Content/>
    </div>
  </div>
  <router-view/>
</template>

<script setup>
import { ref } from "vue";
import Sidebar from "./components/Sidebar.vue";
import Content from "./components/Content.vue";

let menu = ref(["Dashboard","About","Contact","Gallery"])

</script>

<style scoped>
.content{
  background-color: aliceblue;
  width: calc(100vw - 200px);
  height: 100vh;
}
</style>
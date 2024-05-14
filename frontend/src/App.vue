<template>
  <Sidebar :menuItems="menu"/>
  <router-view/>
</template>

<script setup>
import { ref } from "vue";
import Sidebar from "./components/Sidebar.vue";

let menu = ref(["Dashboard","About","Contact","Gallery"])

</script>./components/Sidebar.vue
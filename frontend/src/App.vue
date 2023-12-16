<template>
  <Nav :menuItems="menu"/>
  <router-view/>
</template>

<script setup>
import Nav from "./components/Nav.vue"
import { ref } from "vue";

let menu = ref(["Dashboard","About","Contact","Gallery"])

</script>
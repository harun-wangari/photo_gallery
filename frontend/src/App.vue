<template>
  <router-view/>
</template>

<script setup>
import { ref } from "vue";

let menu = ref(["Dashboard","About","Contact","Gallery"])

</script>

<style scoped>
.content{
  background-color: aliceblue;
  width: calc(100vw - 200px);
  height: 100vh;
}
</style>
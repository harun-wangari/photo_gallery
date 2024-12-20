<template>
    
  <div class="d-flex h-100 w-100 overflow-hidden">
    <div class="side">
      <Sidebar :menuItems="menu"/>
    </div>
    <div class="content w-100">
      <Content/>
    </div>
  </div>
</template>

<script setup>
import Sidebar from "../components/Sidebar.vue";
import Content from "../components/Content.vue";

let menu = ["my pics", "others"]
</script>


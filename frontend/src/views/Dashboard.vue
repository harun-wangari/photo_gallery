<template>
    <div class="d-flex align-items-center justify-content-center h-100 bg-warning dashboard">
        <h2 class="h2">Hi there !!, {{ user.name }}</h2>
    </div>
    <div class="d-flex h-100 w-100">
    <div class="side">
      <Sidebar :menuItems="menu"/>
    </div>
    <div class="content">
      <Content/>
    </div>
  </div>
</template>

<script setup>
import Sidebar from "../components/Sidebar.vue";
import Content from "../components/Content.vue";
import { user } from "../assets/store.js";
</script>
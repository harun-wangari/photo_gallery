<template>
    <div class="d-flex align-items-center justify-content-center h-100 bg-warning dashboard">
        <h2 class="h2">Hi there !!, {{ user.name }}</h2>
    </div>
</template>

<script setup>
import { user } from "../assets/store.js";
</script>
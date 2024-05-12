<template class="con">
    <div class="d-flex align-items-center justify-content-center">
        <nav class="navbar navbar-expand-lg">
            <ul class="navbar-nav">
                <li v-for="item in menuItems" class="nav-item"><a href="#" class="nav-link">{{ item}}</a></li>
            </ul>
        </nav>
    </div>
</template>

<script setup>
const props = defineProps({menuItems:{type:Array}})
</script>
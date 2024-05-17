
import Thumbnail from './Thumbnail.vue';
<template>
    <div>
        <nav aria-label="breadcrumb" style="width: 100%;">
            <ol class="breadcrumb shadow p-2">
                <li class="breadcrumb-item"><a href="#" class="text-dark text-decoration-none ">Photos</a></li>
                <li class="breadcrumb-item active" aria-current="page">Library</li>
            </ol>
        </nav>
        <div class="main">
            <Thumbnail/>
            <Thumbnail/>
        </div>
    </div>
</template>

<script setup>
    import Thumbnail from './Thumbnail.vue';
</script>

<style scoped>
    .main{
        position: relative;
        display: flex;
        padding: 0;
        z-index: 1;
    }
</style>
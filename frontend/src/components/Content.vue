
import Thumbnail from './Thumbnail.vue';
<template>
    <div style="position: relative;height:100vh">
        <nav aria-label="breadcrumb" style="width: 100%;">
            <ol class="breadcrumb shadow p-2">
                <li class="breadcrumb-item"><a href="#" class="text-dark text-decoration-none ">Photos</a></li>
                <li class="breadcrumb-item active" aria-current="page">Library</li>
            </ol>
        </nav>
        <div class="main">
            <Thumbnail/>
            <Thumbnail/>
            <div class="photos">
                <ViewPhoto/>
            </div>
        </div>
    </div>
</template>

<script setup>
    import Thumbnail from './Thumbnail.vue';
    import ViewPhoto from './ViewPhoto.vue';
</script>

<style scoped>
    .main{
        display: flex;
        position: relative;
        padding: 0;
    }
    .photos{
        position: absolute;
        width: 100%;
        padding: 5px;
    }
</style>
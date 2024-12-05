
<template>
    <div style="position: relative;height:100vh" class="">
        <nav aria-label="breadcrumb" class=" bg-danger bg-opacity-75 bg-gradient" style="width: 100%;">
            <ol class="breadcrumb shadow p-2">
                <li class="breadcrumb-item"><a href="#" class="text-dark text-decoration-none ">{{navigation.category}}</a></li>
                <li class="breadcrumb-item active text-white" aria-current="page">{{navigation.album}}</li>
            </ol>
        </nav>
        <div class="main ">
            <Thumbnail/>
            <Thumbnail/>
            <div class="photos d-none">
                <ViewPhoto/>
            </div>
            <div class="photos d-none">
                <UploadPhoto/>
            </div>
        </div>
    </div>
</template>

<script setup>
    import Thumbnail from './Thumbnail.vue';
    import UploadPhoto from './UploadPhoto.vue';
    import ViewPhoto from './ViewPhoto.vue';
    import { navigation } from '../assets/store';
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
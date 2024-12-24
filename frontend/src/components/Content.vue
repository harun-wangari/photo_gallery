
<template>
    <div style="position: relative;height:100vh" class="">
        <nav aria-label="breadcrumb" class=" bg-danger bg-opacity-75 bg-gradient" style="width: 100%;">
            <ol class="breadcrumb shadow p-2">
                <li class="breadcrumb-item"><a href="#" class="text-dark text-decoration-none ">{{navigation.category}}</a></li>
                <li class="breadcrumb-item active text-white" aria-current="page">{{navigation.album}}</li>
            </ol>
        </nav>
        <div :class= "menu.uploadWindowIsActive || menu.viewPhotoWindowIsActive  ? 'main row' : 'main row overflow-y-auto'">
            <Thumbnail v-for = "file,index in media.activeAlbum" :image="file" :id="index" @dblclick="handleThumbnailDblClick" />
            <div  :class="menu.viewPhotoWindowIsActive? 'photos' : 'photos d-none'" >
                <ViewPhoto/>
            </div>
            <div :class= "menu.uploadWindowIsActive ? 'photos d-flex' : 'photos d-none'">
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
    import { useUserStore,useMenuStore,useMediaStore } from '../assets/store';

    const media = useMediaStore();
    const user = useUserStore();
    const menu = useMenuStore();
    media.setFiles(user.id.toString())

    const handleThumbnailDblClick = (e) => {
        menu.viewPhotoWindowIsActive = true;
        let picture = media.activeAlbum[e.currentTarget.id]
        Object.assign({},picture,{"index":e.currentTarget.id})
        media.setPicture(picture);
    }


</script>

<style scoped>
    .main{
        display: flex;
        position: relative;
        padding: 0;
        height: 92vh;
    }
    .photos{
        position: absolute;
        margin-left: 10px;
        width: 98.5%;
        padding: 5px;
    }
</style>
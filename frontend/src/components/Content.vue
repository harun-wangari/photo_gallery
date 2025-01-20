
<template>
    <div style="position: relative;height:100vh" class="container">
        <nav aria-label="breadcrumb"  style="width: 100%;">
            <ol class="breadcrumb shadow p-2">
                <li class="breadcrumb-item"><a href="#" class="text-dark text-decoration-none ">{{navigation.category}}</a></li>
                <li class="breadcrumb-item active text-white" aria-current="page">{{navigation.album}}</li>
            </ol>
        </nav>
        <div :class= "menu.uploadWindowIsActive || menu.viewPhotoWindowIsActive  ? 'main row' : 'main row overflow-y-auto'">
            <Thumbnail v-for = "file,index in media.activeAlbum" :image="file" :id="index" @dblclick="handleThumbnailDblClick" />
            <div class="fs-5 text-center text-white h-50" :class="media.activeAlbum.length > 0 ? 'd-none' : ''">No files Found</div>
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
    // import MessageBox from 'vue-msgbox';
    import { useUserStore,useMenuStore,useMediaStore,useNavigation } from '../assets/store';

    const media = useMediaStore();
    const user = useUserStore();
    const menu = useMenuStore();
    const navigation = useNavigation()
    media.setFiles()

    const handleThumbnailDblClick = (e) => {
        menu.viewPhotoWindowIsActive = true;
        let picture = media.activeAlbum[e.currentTarget.id]
        let updatePic = Object.assign({},picture,{index:parseInt(e.currentTarget.id)})
        media.setPicture(updatePic);
    }


</script>

<style scoped>
.container{
    position: relative;
    margin: 0;
    background: rgb(46, 45, 45);
    opacity: .95;
}
    .main{
        display: flex;
        position: relative;
        top: 0;
        padding: 0;
        height: 92vh;
        animation: enter 1s linear;
    }
    .breadcrumb{
        background-image: linear-gradient(180deg,#e42929 0%, #373838 100%);
    }
    .photos{
        position: absolute;
        margin-left: 10px;
        width: 98.5%;
        padding: 5px;
    }

   

    .photos.d-none{
        display: none;
        opacity: 0;
        transition-property: display opacity;
        transition-duration: 1s;
        transition-behavior: allow-discrete;
    }

    @keyframes enter {
        from{
        top:-20px;

        }
        to{
            top:0;
        }
        
    }


</style>
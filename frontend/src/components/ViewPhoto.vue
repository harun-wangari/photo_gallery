<template>
    <div class="d-flex main row">
        <div class="col-lg-12 flex shadow m-0" style="height: 35px;">
            <span class="bi bi-x-square fs-4 text-danger p-0 m-0" @click="handleCloseBtnClick"></span>
        </div>
        <div class="mainframe row col-lg-12 m-0 h-75">
            <div class="scrollbtn p-1 m-0">
                <span class="bi bi-caret-left-fill fs-2 scroll"></span>
            </div>
            <img :src="/images/ + media.picture.name" alt="" class="picture rounded col-xs-10 col-lg-10 m-0">
            <div class="scrollbtn p-1">
                <span class="bi bi-caret-right-fill fs-2 scroll"></span>
            </div>
        </div>
        <div class="thumbnailframe col-lg-12 h-25">
            <div class="picdetails text-white  text-center" style="font-size: 1em;height: 30px;">
                {{media.picture.name + "  ||  " + media.picture.date_uploaded}}
            </div>
            <div class="thumbnails">
                <div class="thumbnail h-100" v-for = "picture in pictures">
                    <img :src="/images/ + media.activeAlbum[picture].name" alt="" class="w-100 h-100" data-toggle="tooltip" data-placement="bottom" delay="0" title="image name">
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
    import {useMenuStore,useMediaStore} from '../assets/store';
    import { onMounted } from 'vue';

    const menu = useMenuStore();
    const media = useMediaStore();
    let currentPic = media.picture.index
    console.log(media.picture)
    let pictures = [currentPic+1,currentPic+2,currentPic+3]
    if(media.activeAlbum.length == 0){
        pictures = []
    }else if(media.activeAlbum.length < 4){
        media.activeAlbum.forEach((element,index) => {
            if(currentPic != index){
                pictures.push(index)
            }
        });
    }else{
        pictures = [currentPic+1,currentPic+2,currentPic+3]
    }

    const handleCloseBtnClick = () => {
        menu.viewPhotoWindowIsActive = false;
        media.picture.index = -1
    }

</script>

<style scoped>
    .main{
        position: relative;
        align-items: center;
        justify-content: center;
        margin: 0px;
        width: 100%;
        height: 100%;
        background-color: #09203f;background-image: linear-gradient(140deg, #ffffff 25%, #b6a9a9 50%, #645353 75%,#312d2d 100% );opacity: 0.7;
        opacity: 0.95;
        border: 1px solid #c3bdbd;
        border-radius: 4px;
        z-index: 1;
    }

    .mainframe{
        display: flex;
        height: calc(100% - 80px);
        padding: 10px;
    }

    .picture{
        width: calc(100% - 60px);
        height: 100%;
    }

    .thumbnailframe{
        height: 100%;
        padding: 10px;
    }

    .picdetails{
        display: flex;
        width: 100%;
        height: 30%;
        padding: 10px;
        align-items: center;
        justify-content: center;
    }

     .thumbnails{
        display: flex;
        width: 100%;
        height: 70%;
        padding: 10px;
        align-items: center;
        justify-content: center;
    }

    .thumbnail{
        width: 20%;
        height: 60%;
        border-radius: 5px;
        box-shadow: 2px 2px 12px 2px #222;
        padding: 6px;
    }

    .bi-caret-left-fill,.bi-caret-right-fill,.thumbnail :hover{
        cursor:pointer;
        
    }

    .scrollbtn{
        position: relative;
        height: 100%;
        width: 30px;
        top:translate(50,50%);
        color: rgb(190, 81, 54);
    }

    .scroll{
        position: relative;
        top: 45%;
        transform: translate( -50%);
    }
</style>
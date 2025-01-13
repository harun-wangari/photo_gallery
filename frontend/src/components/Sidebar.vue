<template>
    <div class="sidebar position-relative">
        <div class="navbar p-0 m-0" style="position: relative;">
            <div class="navbar-head">
                <div class="d-flex align-items-center justify-content-center pt-2">
                    <img src="/background.jpg" alt="" class="rounded-circle userpic">
                </div>
                <div class="user-info">
                    <p class="text-light m-0">{{user.lastname + " " + user.surname}}</p>
                    <p class="text-secondary">{{media.files.length}} files</p>
                </div>
                <div class="d-flex align-items-center justify-content-center">
                    <button type="button" @click="handleBtnUploadClick" class="btn btn-danger p-1 ps-4 pe-4 pb-1 m-1 btnupload">Upload</button>
                    <button type="button" @click="handleBtnLogoutClick" class="btn btn-logout p-0 ps-4 pe-4 pb-1 m-1 border border-dark" title="Logout" >
                        <span class="bi-box-arrow-right fs-5 text-white p-0" style="background: transparent"></span>
                    </button>
                </div> 
                
            </div>
            <div class="m-2">
                <form action="" class="m-2 me-1 ms-1">
                    <input type="text" class="form-control" placeholder="search files ...">
                </form>
                <h5>Categories</h5><hr>
                <ul class="navbar-nav" @click="handleCategoryClick" > 
                    <li class="nav-item m-1 "><a href="#" class="nav-link text-secondary p-2 category" :class="navigation.category == 'photo' ? 'text-white active' : ''"  id="photo">Photos</a></li>
                    <li class="nav-item m-1 "  ><a href="#" class="nav-link text-secondary p-2 category" :class="navigation.category == 'video' ? 'text-white active' : ''" id="video">Videos</a></li>
                </ul>
            </div>
            <div class="m-2 w-100  albums-nav" >
                <h5>Albums</h5><hr>
                <ul class="navbar-nav  overflow-y-auto" style="height:calc(100% - 50px)">
                    <li v-for="item in navigation.allAlbums" class="nav-item "><a href="#" class="nav-link text-secondary"  :class="navigation.album == item ? 'text-white active p-sticky-top' : ''" @click="handleAlbumClick">{{ item}}</a></li>
                </ul>
            </div>
           
        </div>
    </div>
</template>

<script setup>
import { useUserStore,useMenuStore, useMediaStore,useNavigation } from '../assets/store';
import  router  from '../router'

const user  = useUserStore();
const menu = useMenuStore();
const media = useMediaStore()
const navigation = useNavigation()

const handleBtnUploadClick = () => {
    menu.uploadWindowIsActive = true;
    if(navigation.album == "all"){
        navigation.album = navigation.allAlbums[0];
    }
}

const handleCategoryClick = (e) => {
    if (menu.uploadWindowIsActive || menu.viewPhotoWindowIsActive ){
       
    }else{
        if(e.target.id == "photo"){
        navigation.category = e.target.id
        }else if(e.target.id == "video"){
            navigation.category = e.target.id
        }
        if(navigation.album =="all"){
            media.activeAlbum = media.files.filter(file => file.file_type == navigation.category)
        }else{
            media.activeAlbum = media.files.filter(file => file.album == navigation.album && file.file_type == navigation.category)
        }

    }
        
}

const handleAlbumClick = (e) => {
    if(menu.uploadWindowIsActive || menu.viewPhotoWindowIsActive ){
      
    }else{
        navigation.album = e.currentTarget.innerHTML
        media.setActiveAlbum( media.files.filter((file) => file.album == e.currentTarget.innerHTML && file.file_type == navigation.category))
    }
}

const handleBtnLogoutClick = () => {
    user.setUser({id:0});
    localStorage.removeItem("token")
    router.push("/login")
}

</script>

<style scoped>

.sidebar{
    background-color: #222;
    padding: 0;
    margin: 0;
    color: aliceblue;
    height: 100%;
    width: 200px;
    overflow: hidden;
} 

.navbar-head{
    width: 100%;
    padding: 0;
    margin: 0;
    padding-bottom: 20px;
    background-color: black;
}

.albums-nav{
    height:370px;
    max-height: 40%;
    padding: 0;
    overflow-y: hidden;
}

.userpic{
    height: 120px;
    width: 120px;
}

.user-info{
    position: relative;
    text-align: center;
}

.btnupload{
    background-image: linear-gradient(135deg,#e42929 0%, #373838 100%);
    border: none;
}

.active{
    background-image: linear-gradient(140deg, #a73737 25%, #ec5e5e 50%, #f33f3f 75%,#fc7575 100% );
    opacity: 0.7;
    border-radius: 4px;
    color: white;
}

.category:hover{
    background-image: linear-gradient(140deg, #f8f5f5 25%, #a78282 50%, #913f3f 75%,#dfd4d4 100% );
    opacity: 0.7;
    border-radius: 4px;
    color:black !important ;
}

.btn-logout{
    background: linear-gradient(140deg, #030303 25%, #613030 50%, #ec6161 75%,#312d2d 100% )
}


</style>
<template>
    <div class="sidebar position-relative">
        <div class="navbar p-0 m-0" style="position: relative;">
            <div class="navbar-head">
                <div class="d-flex align-items-center justify-content-center pt-2">
                    <img src="/background.jpg" alt="" class="rounded-circle userpic">
                </div>
                <div class="user-info">
                    <p class="text-light m-0">Harun Njenga</p>
                    <p class="text-secondary">files 200</p>
                </div>
                <div class="d-flex align-items-center justify-content-center">
                    <button type="button" class="btn btn-danger p-0 ps-4 pe-4 pb-1">Upload</button>
                </div>
                
            </div>
            <div class="m-2">
                <form action="" class="m-2 me-1 ms-1">
                    <input type="text" class="form-control" placeholder="search files ...">
                </form>
                <h5>Categories</h5><hr>
                <ul class="navbar-nav">
                    <li class="nav-item"><a href="#" class="nav-link text-secondary">Videos</a></li>
                    <li class="nav-item"><a href="#" class="nav-link text-secondary">Photos</a></li>
                </ul>
            </div>
            <div class="m-2 w-100 albums-nav" >
                <h5>Albums</h5><hr>
                <ul class="navbar-nav">
                    <li v-for="item in menuItems" class="nav-item "><a href="#" class="nav-link text-secondary">{{ item}}</a></li>
                </ul>
            </div>
           
        </div>
    </div>
</template>

<script setup>
const props = defineProps({menuItems:{type:Array}})
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
    max-height: 100%;
    padding: 0;
    overflow-y: auto;
}

.userpic{
    height: 120px;
    width: 120px;
}

.user-info{
    position: relative;
    text-align: center;
}


</style>
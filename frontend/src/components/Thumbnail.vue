<template>
    <div class="thumbnail">
        <div class="image"><img src="/background.jpg" alt="" style="width: 100% ;height: 200px;"></div>
    </div>
</template>

<style scoped>
    .thumbnail{
        position: relative;
        margin: 10px;
        padding: 5px;
        width: 200px;
        height: 250px;
        left: 5px;
        top: 5px;
        border:  rgb(96, 85, 85) 1px solid;
        border-radius: 5px;
        box-shadow: 2px 2px 5px 3px #636262;
        
    }

    .thumbnail::before ,
    .thumbnail::after {
        content: "";
        position: absolute;
        transform: rotate(0deg);
        top:0;
        left: 0;
        width: 200px;
        height: 250px;
        padding: 5px;
        background: #ffffff;
        transition:all .5s ease;
        border:  rgb(96, 85, 85) 1px solid;
        box-shadow: 2px 2px 5px 3px #636262;
        z-index: -1;
    }

    .thumbnail:hover::after{
        transform: rotate(7deg);
    }

    .thumbnail:hover::before{
        transform: rotate(15deg);
    }

    .image{
        width: 100%;
        height: 100%;
    }
</style>
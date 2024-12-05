<template>
    <div class="thumbnail">
        <div class="image">
            <img src="/background.jpg" alt="" style="width: 100% ;height: 200px;">
            <div class="details">
                <span>photo details </span><br/>
                <span>12/05/2024</span>
            </div>
        </div>
        
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
        /* background-color: #ffffff; */
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
        background-color: #ffffff !important;
        transition:all .5s ease;
        border:  rgb(96, 85, 85) 1px solid;
        box-shadow: 2px 2px 5px 3px #636262;
        z-index: -1;
    }

    .thumbnail:hover::after{
        transform: rotate(7deg);
        background-color: #ffffff !important;
    }

    .thumbnail:hover::before{
        transform: rotate(15deg);
        background: #ffffff !important;
    }

    .image{
        width: 100%;
        height: 100%;
    }

    .details{
        color: white;
        font-size: 12px;
        font-family:'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;
    }
</style>
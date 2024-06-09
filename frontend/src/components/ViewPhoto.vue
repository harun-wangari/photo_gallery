<script setup>

let pictures = [0,1,2,3,4]

const handleCloseBtnClick = () => {

}

</script>
<template>
    <div class="main row">
        <div class="col-lg-12 flex" style="height: 35px; box-shadow: 10px 10px 10px#222;">
            <span class="bi bi-x-square fs-4 text-danger p-0 m-0" style="float:right;" @click="handleCloseBtnClick"></span>
        </div>
        <div class="mainframe col-sm-12 col-md-5 col-lg-5">
            <div class="scrollbtn">
                <span class="bi bi-caret-left-fill fs-2"></span>
            </div>
            <img src="/background.jpg" alt="" class="picture">
            <div class="scrollbtn">
                <span class="bi bi-caret-right-fill fs-2"></span>
            </div>
        </div>
        <div class="thumbnailframe col-sm-12 col-md-7 col-lg-7">
            <div class="picdetails">
                <div style="text-align: center;">picture details</div>
            </div>
            <div class="thumbnails">
                <div class="thumbnail" v-for = "picture in pictures">
                    <img src="/background.jpg" alt="" class="w-100 h-100" data-toggle="tooltip" data-placement="bottom" delay="0" title="image name">
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
    .main{
        position: absolute;
        display: grid;
        align-items: center;
        justify-content: center;
        margin: 0px;
        grid-row: 3;
        width: 100%;
        height: 90vh;
        background: linear-gradient(140deg, #f9f8f8 25%, #ded9d9 50%, #fbfbfb 75%,#e3dcdc 100% );
        opacity: 0.8;
        border: 1px solid #c3bdbd;
        border-radius: 4px;
        z-index: 1;
    }

    .mainframe{
        display: flex;
        height: 100%;
        padding: 10px;
    }

    .picture{
        width: calc(100% - 60px);
        height: 95%;
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
        height: 100%;
        width: 30px;
        padding-top: 50%;
        color: rgb(49, 133, 51);
    }
</style>
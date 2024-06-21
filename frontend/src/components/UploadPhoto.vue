<template>
    <div class="main row">
        <div class="col-lg-12 flex" style="height: 35px; box-shadow: 10px 10px 10px#222;">
            <span class="bi bi-x-square fs-4 text-danger p-0 m-0" style="float:right;" @click="handleCloseBtnClick"></span>
        </div>
    </div>
</template>

<style scoped>
.main{
        position: absolute;
        display: grid;
        align-items: center;
        grid-row: 3;
        justify-content: center;
        margin: 0px;
        width: 100%;
        height: 90vh;
        background: linear-gradient(140deg, #f9f8f8 25%, #ded9d9 50%, #fbfbfb 75%,#e3dcdc 100% );
        opacity: 0.8;
        border: 1px solid #c3bdbd;
        border-radius: 4px;
        z-index: 1;
    }

</style>
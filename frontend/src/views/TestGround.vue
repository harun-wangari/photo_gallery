<template>
    <div class="d-block bg-secondary h-100">
        
        <div class="bg-danger rounded-circle" style="height: 100px; width: 100px;"></div>
        <div class="bg-white w-100 line" style="height: 5px;"></div>

    </div>
</template>

<style scoped>
.rounded-circle{
    position: relative;
    animation: bounce 3s linear infinite ;
    top: calc(50% - 50px);
    left: 50%;
    transform: translate(-50% ,50%);
}

.line{
    position: relative; 
    top: 50%;
}

@keyframes bounce {
    10%{
        translate: 0 -100px;
    }
    20%{
        translate: 0 0px;
    }
    30%{
        translate: 0 -80px;
    }
    40%{
        translate: 0 0px;
    }
    50%{
        translate: 0 -60px;
    }
    60%{
        translate: 0 0px;
    }
    70%{
        translate: 0 -40px;
    }
    80%{
        translate: 0 0px;
    }
    90%{
        translate: 0 -20px;
    }
    100%{
        translate: 0 0px;
    }

}


</style>
<template>

    <div class="d-grid">
        <div class="card">1</div>
        <div class="card">2</div>
        <div class="card">3</div>
        <div class="card">4</div>
        <div class="card">5</div>
        <div class="card">6</div>
        <div class="card">7</div>
        <div class="card">8</div>
        <div class="card">9</div>
        <div class="card">10</div>
    </div>
    <div class="d-block bg-secondary h-50">
        
        <div class="bg-danger rounded-circle" style="height: 100px; width: 100px;"></div>
        <div class="bg-white w-100 line" style="height: 5px;"></div>

    </div>
</template>

<style scoped>

.d-grid{
    grid-template-columns:  repeat(auto-fit, minmax(200px, 1fr));
    gap: .5rem;
    padding: 10px;
   
}

.card{
    opacity: 1;
    height: 300px;
    background-image: linear-gradient(140deg, #8ae274 0%, #456d3b 50%);
}

.card:hover{
    animation: .1s linear pop forwards;
}

.card:not(:hover){
    animation: .1s linear dim forwards;
}

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

@keyframes pop{
    from{
        scale:1;
        /* box-shadow:  rgba(245, 247, 246, 0.1); */
    }
    to{
        scale:1.02;
        box-shadow: 4px 4px rgba(113, 163, 143, 0.5);
    }
}

@keyframes dim{
    from{
    opacity: 1;
    }
    to{
        opacity: .7;
    }
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
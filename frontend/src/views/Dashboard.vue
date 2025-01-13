<template>
    
  <div class="d-flex h-100 w-100 overflow-hidden">
    <div class="side">
      <Sidebar/>
    </div>
    <div class="con-content w-100">
      <Content/>
    </div>
  </div>
</template>

<style>
.con-content{
  background-image: url("/background1.jpg");
  background-repeat: no-repeat;
  background-size: cover;
}
</style>

<script setup>
import Sidebar from "../components/Sidebar.vue";
import Content from "../components/Content.vue";
import { useMediaStore, useUserStore } from "../assets/store";
import { onBeforeMount } from "vue";
import router from '@/router'

const user = useUserStore()

onBeforeMount( () => {
    const token = localStorage.getItem('token')
    fetch("http://localhost:3000/api/verify_token",{
        method:"GET", 
        headers:{
            "Content-Type":"application/json",
            "Authorization":"Bearer " + token
        }
    })
    .then((res) => {
        if(res.status == 200) { 
          res.json().then((data) => {
            user.setUser(data)
            localStorage.setItem('token',data.token)
            const media = useMediaStore()
            media.setFiles()
          }
        )
      }else{
            localStorage.removeItem('token')
            router.push('/login')
        }
    })
})

</script>


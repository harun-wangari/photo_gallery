import {createRouter,createWebHistory} from "vue-router"
import Login from "../views/Login.vue"
import Signup from "../views/Signup.vue"
import Dashboard from "../views/Dashboard.vue"
import pinia,{useUserStore} from '../assets/store'
import TestGround from "../views/TestGround.vue"
import { setActivePinia } from "pinia"

setActivePinia(pinia)

const routes = [
    {
        path:"/login",
        name:"Login",
        component: Login,
        beforeEnter: (to,from) => {
            const token = localStorage.getItem('token')
            if(token){
                router.push("/dashboard")
            }
        }
    },
    {
        path:"/signup",
        name:"Signup",
        component: Signup,
        beforeEnter: (to,from) => {
            const token = localStorage.getItem('token')
            if(token){
                router.push("/login")
            }
        }
    },
    {
        path:"/dashboard",
        name:"Dashboard",
        component: Dashboard,
        beforeEnter: (to,from) => {
            const token = localStorage.getItem('token')
            if(!token){
                router.push("/login")
            }
        }
    },
    {
        path:"/testground",
        name:"TestGround",
        component: TestGround,
    }
];

const router = createRouter({
    history:createWebHistory("/"),
    routes
})

export default router;
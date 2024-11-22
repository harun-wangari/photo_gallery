import {createRouter,createWebHistory} from "vue-router"
import Login from "../views/Login.vue"
import Signup from "../views/Signup.vue"
import Dashboard from "../views/Dashboard.vue"
import {user} from "../assets/store.js"

const routes = [
    {
        path:"/login",
        name:"Login",
        component: Login,
        beforeEnter: (to,from) => {
            if(user.isLoggedIn){
                router.push("/dashboard")
            }
        }
    },
    {
        path:"/signup",
        name:"Signup",
        component: Signup,
        beforeEnter: (to,from) => {
            if(user.isLoggedIn){
                router.push("/dashboard")
            }
        }
    },
    {
        path:"/dashboard",
        name:"Dashboard",
        component: Dashboard,
        beforeEnter: (to,from) => {
            if(!user.isLoggedIn){
                router.push("/login")
            }
        }
    }
];

const router = createRouter({
    history:createWebHistory("/"),
    routes
})

export default router;
import {createRouter,createWebHistory} from "vue-router"
import Login from "../views/Login.vue"
import Signup from "../views/Signup.vue"
import Dashboard from "../views/Dashboard.vue"
import {useUserStore} from '../assets/store'

const routes = [
    {
        path:"/login",
        name:"Login",
        component: Login,
        beforeEnter: (to,from) => {
            const user = useUserStore()
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
            const user = useUserStore()
            if(user.isLoggedIn){
                router.push("/login")
            }
        }
    },
    {
        path:"/dashboard",
        name:"Dashboard",
        component: Dashboard,
        beforeEnter: (to,from) => {
            const user = useUserStore()
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
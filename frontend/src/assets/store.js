import {reactive} from "vue"
import {createPinia,defineStore} from 'pinia'

const pinia  = createPinia()

// export const user = reactive({
//     id :0,
//     surname:"",
//     lastname:"",
//     email:"",
//     photo:"",
//     token:"",
//     isLoggedIn:false,
// })

export  const useUserStore = defineStore('user',{
    state: () => (
       { 
            id :0,
            surname:"",
            lastname:"",
            email:"",
            photo:"",
            token:"",
            isLoggedIn:false,
        }
    ),
    actions:{
        setUser(value){ 
            this.$patch(value)
        },
        getSurname(){
            return this.surname
        },
        getIsLoggedIn(){
            return this.isLoggedIn
        }
    }

})


export const navigation = reactive({
    category:"photos",
    album:"all"
})


export default pinia
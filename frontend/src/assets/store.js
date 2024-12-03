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
            console.log(value);
            this.id = value.id,
            this.surname = value.surname,
            this.lastname = value.lastname,
            this.email = value.email,
            this.photo = value.photo,
            this.token = value.token,
            this.isLoggedIn = value.isLoggedIn
        },
        getSurname(){
            this.surname
        },
        getIsLoggedIn(){
            this.isLoggedIn
        }
    }

})


export const navigation = reactive({
    category:"photos",
    album:"all"
})


export default pinia
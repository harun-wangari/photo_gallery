import {reactive} from "vue"
import {createPinia,defineStore} from 'pinia'
import { useToast } from "vue-toastification"

const pinia  = createPinia()
const toast = useToast()

export const useMenuStore = defineStore("menuItem",{
    state: () => (
       {
            uploadWindowIsActive:false,
            viewPhotoWindowIsActive:false,
            category:"",
        }
    )
})

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

export const useMediaStore = defineStore('media', {
    state: () => ({
        files: [],
        picture:{"index":-1},
        activeAlbum:[],
    }),
    actions:{
        setFiles(user_id){
            fetch("http://localhost:3000/api/get_all_files", {
                method:"POST",
                headers: {
                    "Content-Type":"application/json",
                },
                body: JSON.stringify({id:user_id})

            })
            .then(res => res)
            .then(res => {
                if (res.status  == 200) {
                    res.json().then(data => {
                        this.files = data
                        this.activeAlbum = data
                    })
                }else{
                    res.text().then( text => toast.error(text ,{autoClose:1000}))
                }
            })
        },
        setPicture(pic){
            this.picture = pic
        },
        setActiveAlbum(album){
            this.activeAlbum = album
        },
        getFiles(){
            return this.files
        }

    }
})




export const navigation = reactive({
    category:"photos",
    album:"all"
})


export default pinia
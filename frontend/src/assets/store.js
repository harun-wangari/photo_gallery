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
        files: new Array(),
        picture:{"index":-1},
        activeAlbum:[],
    }),
    actions:{
        setFiles(){
            let user = useUserStore()
            const token = user.token
            if(token){
                fetch("http://localhost:3000/api/get_all_files", {
                    method:"POST",
                    headers: {
                    "Content-Type":"application/json",
                    Authorization:"Bearer " + user.token
                }

                })
                .then(res => res)
                .then(res => {
                    if (res.status  == 200) {
                        res.json().then(data => {
                            this.files = data
                            this.activeAlbum = data
                            const navigation = useNavigation()
                            let albums = data.map(file => file.album)
                            let uniqueAlbums = [...new Set(albums)]
                            navigation.setAlbums(uniqueAlbums)
                        })
                    }
                })
            }
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




export const useNavigation = defineStore('navigation',{
    state : () => ({
        category:"photo",
        album:"all",
        allAlbums:[],
    }),
    actions:{
        setAlbums(albums){
            this.allAlbums = albums
        }
    }


})


export default pinia
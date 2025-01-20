<template>
    <div class="d-flex row main">

        <div class="col-lg-6 col-md-8 flex p-2 bg-dark rounded  uploadform"  :class= "!menu.uploadWindowIsActive ? 'fade' : ''">
            <div class="d-flex  shadow rounded bg-opacity-25 bg-gradient mb-2">
                <span class="bi bi-x-square fs-4 text-danger p-0 m-0" style="float:right;" @click="handleBtnCloseClick"></span>
            </div>
            <div class="rounded border border-dark mb-4 overflow-auto" style="height: 70%;">
                <div v-for ="file,index in files.list" class="filelist p-0 m-0" >
                    <div class="border bg-dark-subtle rounded mt-2 p-1" style="height: 40px;">
                        <span>{{ file.name }}</span>
                        <span class="bi bi-x-square fs-4 text-danger p-0 m-0" style="float:right;" @click="handleBtnRemoveFileClick" :id="index"></span>
                    </div>
                </div>
                <div :class="files.list.length == 0 ? 'border bg-dark-subtle rounded mt-2 p-1 text-center' : 'd-none'" style="height: 40px;">
                    <div>
                        <span>No file(s) selected for upload</span>
                    </div>
                    <div style="margin-top: 60px;">
                        <img src="/emptyfile.gif" alt="">
                    </div>
                </div>
            </div>
            <div class="text-white">Files will be uploaded in '{{navigation.album}}' folder. To change this click on a diffent album</div>
            <div class="row p-2">
                <input type="file" ref="uploadfile" accept="image/*,video/*" @change="handleProfilePicChange" hidden multiple/>
                <button class="btn btn-dark  bg-opacity-50 bg-gradient col-md-4 ms-1 me-1" @click="handleBtnSelectClick">Select File(s)</button>
                <button class="btn btn-danger  bg-opacity-50 bg-gradient col-md-4 ms-1 me-1" @click="handleSubmitBtnClick">Upload Files</button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { reactive, ref } from 'vue';
import { useMenuStore,useUserStore,useMediaStore,useNavigation } from '../assets/store';
import { useToast } from 'vue-toastification';
import Swal from 'sweetalert2' ;

const toast = useToast()

const menu = useMenuStore()
const user = useUserStore()
const media = useMediaStore()
const navigation = useNavigation()
const uploadfile = ref()
const files = reactive({
    list:[],
    addFile(file){
        this.list.push(file)
    },
    clearFiles(){
        this.list = []
    },
    removeFile(id){
        this.list.splice(id,1)
    }
})



const handleBtnCloseClick = () => {
    menu.uploadWindowIsActive = false;
    files.clearFiles()
}

const handleBtnSelectClick = () => {
    uploadfile.value.click()
}

const handleBtnRemoveFileClick = (e) => {
    files.removeFile(e.target.id)
}

const handleProfilePicChange = () => {
    const filelist = Array.from(uploadfile.value.files)
    const storefiles = files.list.map( f => f.name)
    const newFileList = filelist.filter( file => !storefiles.includes(file.name))
    console.log(newFileList)
    newFileList.forEach( f =>  {
        if(f.type == "image/jpeg" || "image/png" || "image/gif" ||  "video/mp4" ||"video/mpeg" || "video/ogg" ||  "video/webm"){
            files.addFile(f)
        }else{
            Swal.fire({
                icon:"warning",
                text:"Invalid File Type, '" + f.name + "' only images and videos are allowed",
                confirmButtonColor: 'red',
                animation: true,
                timer:'2000'
            })
        }
    })
    if(filelist.length != newFileList.length){
        toast.error('Duplicate File(s)  removed')
    }
   
}

const handleSubmitBtnClick = () => {

    const formData = new FormData()
    if(files.list.length > 0){
        formData.append("album",navigation.album)
        if(navigation.album == "all"){
            formData.append("album","my pics")
        }
        files.list.forEach( file => {
            formData.append('files', file)
        })
        let token = user.token
        fetch("http://localhost:3000/api/upload_files",{
            method: 'POST',
            body: formData,
            headers:{
                "Authorization":"Bearer " + token
            }
        })
        .then(res => res)
        .then(res => {
            if(res.status == 200){
                res.json().then(data => {
                    data.forEach(file => {
                        media.files.push(file)
                    })
                    files.clearFiles()
                    menu.uploadWindowIsActive = false
                    Swal.fire({
                        icon:"success",
                        text:"File(s) uploaded successfully",
                        confirmButtonColor: 'red',
                        animation: true,
                        timer:'2000'
                    })
                })

            }else{
                res.text().then(text => {
                    files.clearFiles()
                    Swal.fire({
                        icon:"error",
                        text:text,
                        confirmButtonColor: 'red',
                        animation: true,
                        timer:'2000'
                    })
                })
            }
        })
    }else{
        Swal.fire({
            icon:"warning",
            text:"No File(s) selected for upload",
            confirmButtonColor: 'red',
            animation: true,
            timer:'2000'
        })
    }
}

</script>

<style scoped>
.main{
        position: absolute;
        align-items: center;
        justify-content: center;
        margin: 0px;
        width: 100%;
        height: 90vh;
        background: linear-gradient(140deg, #444343 25%, #3b3939 50%, #413e3e 75%,#e7a2a2 100% );
        opacity: .95;
        border: 1px solid #c3bdbd;
        border-radius: 4px;
        z-index: 1;
        
    }


    .filelist{
        list-style: none;
    }

    .uploadform{
        height: calc(300px + 30vh);
        translate: 0 10px;
        animation: easein .5s;
        animation-timing-function: linear;
        box-shadow: 2px 2px rgb(105, 104, 104);
    }

    .uploadform.fade{
        animation: fade .5s;
        animation-timing-function: linear;
    }

    @keyframes easein{
        from{
            translate: 0 -40px;
            opacity: 0;
            display: none !important;
            scale: .6;
        }
        to{
            translate: 0 10px;
            opacity: .9;
            display: flex !important;
            scale: 1;
        }
    }

    @keyframes fade{
        from{
            translate: 0 10px;
            opacity: 0.9;
            scale: 1;
        }
        to{
            translate: 0 60px;
            opacity: 0;
            scale: .6;
        }
    }




</style>
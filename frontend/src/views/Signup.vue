<template>
    <div class="d-flex  h-100 body">
        <div class="d-flex w-100 h-100  align-items-center justify-content-center  content">
            <div class="col-sm-7 col-md-6 col-lg-5 col-xl-4">
                <form class="loginform p-2 rounded"   @submit.prevent="onSubmit" style="color: #09203f;background-image: linear-gradient(-225deg,#dfe9f3 0%, #ffffff 100%);background-color: rgba(105, 103, 103, 0.425);">
                    <h2 class="h2 text-red-500 text-opacity-75 mb-5">Create Account</h2>
                    <div class="d-flex ">
                        <div>
                            <div class="m-1">
                                <label class="text-dark text-opacity-75">Surname</label>
                                <input type="text" v-model="surname" class="input form-control shadow-none border border-none" />
                            </div>
                            <p class="text-danger ps-3 pe-3" style="font-size: 14px;" >{{ errors.surname}}</p>
                        </div>
                        <div>
                            <div class="m-1">
                                <label class="text-dark text-opacity-75">Last Name</label>
                                <input type="text" v-model="lastname" class="input form-control shadow-none border border-none" />
                            </div>
                            <p class="text-danger ps-3 pe-3" style="font-size: 14px;" >{{ errors.lastname}}</p>
                        </div>
                    </div>
                
                    <div>
                        <label class="text-dark text-opacity-75">Email</label>
                        <div class="inputs d-flex border border-dark-subtle rounded">
                            <i class="fs-4 text-danger pe-3 ps-3 bi-envelope-at"></i>
                            <input type="email" v-model="email" class="inputs form-control shadow-none border border-none" />
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;" >{{ errors.email}}</p>
                    </div>
                    <div>
                        <label class="text-dark text-opacity-75">Password</label>
                        <div  class="inputs d-flex  border border-dark-subtle rounded">
                            <i class="fs-4 text-danger  pe-3 ps-3 bi-lock"></i>
                            <input type="password" v-model="password" class="inputs form-control shadow-none border border-none">
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;">{{ errors.password }}</p>
                    </div>
                    <div>
                        <label class="text-dark text-opacity-75">Confirm Password</label>
                        <div  class="inputs d-flex  border border-dark-subtle rounded">
                            <i class="fs-4 text-danger  pe-3 ps-3 bi-lock"></i>
                            <input type="password" v-model="confirmpassword" class="inputs form-control shadow-none border border-none">
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;">{{ errors.comfirmpassword }}</p>
                    </div>
                    
                    <div class="d-flex text-right"  v-if="state.isProcessing == false" >
                        <input id="js-login-btn" value="Create Account" type="submit" class="btn btn-block text-white bg-danger bg-gradient bg-opacity-75 mt-3 w-100" >
                    </div>
                    <div class=" text-center bg-transparent" v-if="state.isProcessing == true">
                        <span class="pd-3">Processing.. </span><span class="spinner-grow text-danger text-opacity-70 text-gradient pt-3"></span>
                    </div>
                   
                </form>
            </div>
        </div>
    </div>
</template>

<style>
input:focus{
    box-shadow: 2px 2px 2px 1px rgb(58, 58, 58) !important;
}
</style>

<script setup>
import { reactive, watch } from "vue";
import {object,promise,string} from "zod";
import {useToast} from "vue-toastification";
import router from "../router/index.js";
import {useForm, useField} from "vee-validate";
import {toTypedSchema} from "@vee-validate/zod";

const toast = useToast()

const formSchema = toTypedSchema( 
    object({
        surname:string().min(1,{message:'Surname is required'}).min(3,{message:'Surname should be at least 3 characters'}),
        lastname:string().min(1,{message:'Lastname is required'}).min(3,{message:'Lastname should be at least 3 characters'}),
        email:string().min(1,{message:'email is required'}).email({message:'invalid email'}),
        password:string().min(1,{message:'password is required'}).min(4,{message:'password too short'}),
        confirmpassword:string()
    })
    .refine( data => data.password === data.confirmpassword, {
            message:"password don't match",
            path:["comfirmpassword"],
        }
    )
)

const {handleSubmit, errors, defineField} = useForm({
    validationSchema:formSchema,
});


const {value: surname} = useField('surname')
const {value: lastname} = useField('lastname')
const {value: email} = useField('email')
const {value: password} = useField('password')
const {value: confirmpassword} = useField('confirmpassword')

let state = reactive({isProcessing:false})

const onSubmit = handleSubmit ((data) => {
        state.isProcessing = true
        fetch("http://localhost:3000/api/create_user",{
            method:"POST",
            headers:{
                "Content-Type":"application/json",
            },
            body:JSON.stringify({
                surname: data.surname.charAt(0).toUpperCase() + data.surname.slice(1).toLowerCase(),  //capitalize first letter only
                lastname:data.lastname.charAt(0).toUpperCase() + data.lastname.slice(1).toLowerCase(), //capitalize first letter only
                email:data.email.toLowerCase(),
                password:data.password,
                photo:"image/user.jpg",
            })
        })
        .then((res) => res)
        .then((data) => {
            state.isProcessing = false
            if(data.status==200) {
                data.text().then(text => toast.success(text))
                router.push('/login')
            }else{
                data.text().then( text => {
                    toast.error(text ,{autoClose:1000})
                  
                })
            }
        })
    })

    watch(
        () => errors.value,
        () => {console.log(errors.value)}
        )
// };


</script>


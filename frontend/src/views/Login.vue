<template>
    <div class="d-flex  h-100 body">
        <div class="d-flex w-100 h-100  align-items-center justify-content-center  content">
            <div class="col-sm-7 col-md-6 col-lg-5 col-xl-4">
                <form class="loginform p-2 rounded bg-info"   @submit.prevent="onSubmit" style="color: #09203f;background-image: linear-gradient(140deg, #cacaca 25%, #bba7a7 50%, #9e8686 75%,#e7a2a2 100% );opacity: 0.7;">
                    <h2 class="h2 mb-5">Log in Form</h2>
                    <div>
                        <label class="text-dark text-opacity-75">Email</label>
                        <div class="inputs d-flex border border-dark-subtle rounded">
                            <i class="fs-4 text-danger pe-3 ps-3 bi-envelope-at"></i>
                            <input type="email" v-model="email" class="form-control shadow-none border border-none" />
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;" >{{ errors.email}}</p>
                    </div>
                    <div>
                        <label class="text-dark text-opacity-75">Password</label>
                        <div class="inputs d-flex  border border-dark-subtle rounded">
                            <i class="fs-4 text-danger pe-3 ps-3 bi-lock"></i>
                            <input type="password" v-model="password" class="form-control shadow-none border border-none">
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;">{{ errors.password }}</p>
                    </div>
                    <div>
                        <div class="d-flex mt-4 mb-4 " v-if="state.isProcessing == false">
                            <input type="submit" value="Log In"   class="form-control text-white bg-danger bg-gradient bg-opacity-75" style="background-image: linear-gradient(135deg,#9be15d 0%, #00e3ae 100%)"/>
                        </div>
                        <div class=" text-center bg-transparent mt-4 mb-4 " v-if="state.isProcessing == true">
                            <span class="pd-3">Processing.. </span><span class="spinner-grow text-danger text-opacity-70 text-gradient pt-3"></span>
                        </div>
                    </div>

                </form>
            </div>
        </div>
    </div>
</template>

<style>
.inputs:has(input:focus){
    box-shadow: 2px 2px 2px 1px rgb(58, 58, 58) !important;
}
</style>

<script setup>
import { reactive, watch } from "vue";
import {object,promise,string} from "zod";
import {useToast} from "vue-toastification";
import router from "../router";
import {useForm, useField} from "vee-validate";
import {toTypedSchema} from "@vee-validate/zod";
import { useUserStore } from "../assets/store.js";

const toast = useToast()
const user = useUserStore()

const formSchema = toTypedSchema( 
    object({
        email:string().min(1,{message:'email is required'}).email({message:'invalid email'}),
        password:string().min(1,{message:'password is required'}).min(4,{message:'password too short'}),
    })
)

const {handleSubmit, errors, defineField} = useForm({
    validationSchema:formSchema,
});

// const [email,emailAlt] = defineField('email')
// const [password,passwordAlt] = defineField('password')

const {value: email} = useField('email')
const {value: password} = useField('password')

let state =  reactive({isProcessing : false})

const onSubmit = handleSubmit ((data) => {
        state.isProcessing = true
        fetch("http://localhost:3000/api/login",{
            method:"POST",
            headers:{
                "Content-Type":"application/json",
            },
            body:JSON.stringify({
                email:data.email,
                password:data.password,
            })
        })
        .then((res) => res)
        .then((res) => {
            state.isProcessing = false
            if(res.status==200) {
                res.json().then(data => {
                    user.setUser({id:data.id,surname:data.surname,lastname:data.lastname,email:data.email,photo:data.photo,token:data.token})
                    localStorage.setItem("token",data.token)
                    router.push('/dashboard')
                    toast.success("Log in successfull, Welcome " + user.surname)
                })
            }else{
                res.text().then( text => {
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


<template>
    <div class="d-flex  h-100 body">
        <div class="d-flex w-100 h-100  align-items-center justify-content-center  content">
            <div class="col-sm-7 col-md-6 col-lg-5 col-xl-4">
                <form class="loginform p-2 rounded"   @submit.prevent="onSubmit" style="color: #09203f;background-image: linear-gradient(-225deg,#dfe9f3 0%, #ffffff 100%);background-color: rgba(105, 103, 103, 0.425);">
                    <h2 class="h2">Create Account</h2>
                    <div>
                        <label>Email</label>
                        <div class="d-flex ">
                            <i class="fs-3 text-white pe-3 ps-3 bi-envelope-at" style="background-color:  #537895"></i>
                            <input type="email" v-model="email" class="form-control" />
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;" >{{ errors.email}}</p>
                    </div>
                    <div>
                        <label>Password</label>
                        <div class="d-flex ">
                            <i class="fs-3 text-white pe-3 ps-3 bi-lock" style="background-color:  #537895"></i>
                            <input type="password" v-model="password" class="form-control">
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;">{{ errors.password }}</p>
                    </div>
                    <div>
                        <label>Confirm Password</label>
                        <div class="d-flex ">
                            <i class="fs-3 text-white pe-3 ps-3 bi-lock" style="background-color:  #537895"></i>
                            <input type="password" v-model="confirmpassword" class="form-control">
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;">{{ errors.comfirmpassword }}</p>
                    </div>
                    <div class="row no-gutters">
                        <div class="d-flex text-right" v-if="isProccessing == false">
                            <button id="js-login-btn" type="submit" class="btn btn-block btn-primary  mt-3 w-100">Add User</button>
                        </div>
                        <div class=" text-center bg-danger" v-if="isProccessing == true">
                            <span class="spinner-grow text-success p-1"></span>
                            <span class="spinner-grow text-success p-1"></span>
                        </div>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<script setup>
import { watch } from "vue";
import {object,promise,string} from "zod";
import {useToast} from "vue-toastification";
import router from "../router/index.js";
import { user } from "../assets/store.js";
import {useForm, useField} from "vee-validate";
import {toTypedSchema} from "@vee-validate/zod";

const toast = useToast()

const formSchema = toTypedSchema( 
    object({
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

// const [email,emailAlt] = defineField('email')
// const [password,passwordAlt] = defineField('password')

const {value: email} = useField('email')
const {value: password} = useField('password')
const {value: confirmpassword} = useField('confirmpassword')

let isProccessing = false

const onSubmit = handleSubmit ((data) => {
        isProccessing = true
        fetch("http://localhost:3000/api/create_user",{
            method:"POST",
            headers:{
                "Content-Type":"application/json",
            },
            body:JSON.stringify({
                email:data.email,
                password:data.password,
                photo:"image/user.jpg",
            })
        })
        .then((res) => res)
        .then((data) => {
            isProccessing = false
            if(data.status==200) {
                // user.name = data.email
                user.isLoggedIn = true
                router.push('/dashboard')
                toast.success("Log in successfull, Welcome ")
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
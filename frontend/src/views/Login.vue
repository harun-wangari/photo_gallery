<template>
    <div class="d-flex  h-100 body">
        <div class="d-flex w-100 h-100  align-items-center justify-content-center  content">
            <div class="col-sm-7 col-md-6 col-lg-5 col-xl-4">
                <form class="loginform p-2 rounded"   @submit.prevent="onSubmit" style="color: #09203f;background-image: linear-gradient(-225deg,#dfe9f3 0%, #ffffff 100%);background-color: rgba(105, 103, 103, 0.425);">
                    <h2 class="h2">Log in Form</h2>
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
                        <div class="d-flex mt-4 mb-4 ">
                            <input type="submit" value="Log In"   class="form-control text-white" style="background-image: linear-gradient(135deg,#9be15d 0%, #00e3ae 100%)"/>
                        </div>
                    </div>

                </form>
            </div>
        </div>
    </div>
</template>

<script setup>
import { watch } from "vue";
import {object,string} from "zod";
import {useToast} from "vue-toastification";
import router from "../router";
import { user } from "../assets/store.js";
import {useForm, useField} from "vee-validate";
import {toTypedSchema} from "@vee-validate/zod";

const toast = useToast()

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

const onSubmit = handleSubmit ((data) => {
   
        fetch("http://127.0.0.1:3000/login",{
            method:"POST",
            headers:{
                "Content-Type":"application/json",
            },
            body:JSON.stringify({
                email:data.email,
                password:data.password,
            })
        }).then((res) => res.json())
        .then((data) => {
            if(data.error == 0){
                user.name = data.user
                user.isLoggedIn = true
                router.push('/dashboard')
                toast.success("Log in successfull, Welcome " + data.user)
            }else{
                toast.error("invalid credentials" ,{autoClose:1000})
            }
        })
    })

    watch(
        () => errors.value,
        () => {console.log(errors.value)}
        )
// };


</script>
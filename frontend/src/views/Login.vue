<template>
    <div class="d-flex  h-100 body">
        <div class="d-flex w-100 h-100  align-items-center justify-content-center  content">
            <div class="col-sm-7 col-md-6 col-lg-5 col-xl-4">
                <form class="loginform p-2 rounded bg-info"   @submit.prevent="onSubmit">
                    <h2 class="h2 mb-5 text-white">Log in Form</h2>
                    <div>
                        <label class="text-white">Email</label>
                        <div class="inputs d-flex border border-dark-subtle rounded">
                            <i class="fs-4 text-danger pe-3 ps-3 bi-envelope-at"></i>
                            <input type="email" v-model="email" class="form-control shadow-none border border-none" />
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;" >{{ errors.email}}</p>
                    </div>
                    <div>
                        <label class="text-white">Password</label>
                        <div class="inputs d-flex  border border-dark-subtle rounded">
                            <i class="fs-4 text-danger pe-3 ps-3 bi-lock"></i>
                            <input type="password" v-model="password" class="form-control shadow-none border border-none">
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;">{{ errors.password }}</p>
                    </div>
                    <div>
                        <div class="d-flex mt-1 mb-2 con-login"  :class="state.isProcessing == true ? 'fade' : ''">
                            <input type="submit" value="Log In"  class="form-control  btnlogin" />
                        </div>
                        <div class=" text-center bg-transparent mt-1 mb-1 spinner bg-info"  :class="state.isProcessing == false? 'fade' : ''">
                            <span class="pd-3 text-white">Processing.. </span><span class=" spinner-grow text-danger text-gradient pt-3"></span>
                        </div>
                        <p class="text-white resetPassword"><span class="text-danger">Forgot Password? </span>Reset password</p>
                        <p class="text-white createAccount" @click="btnCreateAccount"><span class="text-danger">Don't have an account? </span>Create Account</p>
                    </div>

                </form>
            </div>
        </div>
    </div>
</template>

<style>
.loginform{
    background-image: linear-gradient(140deg, #1b1717 25%, #332a2a 50%, #492424 75%,#750202 100% );
    opacity: 0.9;
}

.con-login{
    position: relative;
    animation: enter .2s;
    z-index: 2;
}

.inputs:has(input:focus){
    box-shadow: 3px 2px  rgb(107, 104, 104) !important;
}

.btnlogin{
    background-image: linear-gradient(135deg,#f8f8f7 0%, #373838 100%);
    box-shadow: 2px 1px  rgb(238, 116, 116);
    border: none !important;
    color: black !important;
    font-weight: bolder !important;
}

.btnlogin:hover{
    background-image: linear-gradient(135deg,#e42929 0%, #373838 100%);
    color: white !important;
    box-shadow: 2px 1px  rgb(255, 254, 254);
}

.spinner{
    animation: enter .2s ;
    position: relative;
    top: -20px;
    z-index: 1;
}

.fade{
    animation: fade .2s forwards;
}

@keyframes enter {
    from {
        display: none;
        opacity: 0;
        top: -20px;
    }

    to{
        display: block;
        opacity: 0.9;
        top: 0px;
    }
}

@keyframes fade {
   from{
        display: block;
        opacity: 0.9;
        top: 0px;
    }
    to {
        display: none;
        opacity: 0;
        top: 20px;
    }
}

.createAccount:hover,.resetPassword:hover{
    scale: 1.01;
    cursor: pointer;
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

    const btnCreateAccount = (e) => {
        router.push("/signup")
    }

    watch(
        () => errors.value,
        () => {console.log(errors.value)}
        )
// };


</script>


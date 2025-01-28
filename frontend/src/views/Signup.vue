<template>
    <div class="d-flex  h-100 body">
        <div class="d-flex w-100 h-100  align-items-center justify-content-center  content">
            <div class="col-sm-7 col-md-6 col-lg-5 col-xl-4">
                <form class="loginform p-2 rounded"   @submit.prevent="onSubmit">
                    <h2 class="h2 text-white mb-5">Create Account</h2>
                    <div class="d-flex ">
                        <div>
                            <div class="m-1">
                                <label class="text-white">Surname</label>
                                <input type="text" v-model="firstname" class="input form-control shadow-none border border-none" />
                            </div>
                            <p class="text-danger ps-3 pe-3" style="font-size: 14px;" >{{ errors.firstname}}</p>
                        </div>
                        <div>
                            <div class="m-1">
                                <label class="text-white">Last Name</label>
                                <input type="text" v-model="lastname" class="input form-control shadow-none border border-none" />
                            </div>
                            <p class="text-danger ps-3 pe-3" style="font-size: 14px;" >{{ errors.lastname}}</p>
                        </div>
                    </div>
                
                    <div>
                        <label class="text-white">Email</label>
                        <div class="inputs d-flex border border-dark-subtle rounded">
                            <i class="fs-4 text-danger pe-3 ps-3 bi-envelope-at"></i>
                            <input type="email" v-model="email" class="inputs form-control shadow-none border border-none" />
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;" >{{ errors.email}}</p>
                    </div>
                    <div>
                        <label class="text-white">Password</label>
                        <div  class="inputs d-flex  border border-dark-subtle rounded">
                            <i class="fs-4 text-danger  pe-3 ps-3 bi-lock"></i>
                            <input type="password" v-model="password" class="inputs form-control shadow-none border border-none">
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;">{{ errors.password }}</p>
                    </div>
                    <div>
                        <label class="text-white">Confirm Password</label>
                        <div  class="inputs d-flex  border border-dark-subtle rounded">
                            <i class="fs-4 text-danger  pe-3 ps-3 bi-lock"></i>
                            <input type="password" v-model="confirmpassword" class="inputs form-control shadow-none border border-none">
                        </div>
                        <p class="text-danger ps-3 pe-3" style="font-size: 14px;">{{ errors.comfirmpassword }}</p>
                    </div>
                    
                    <div class="d-flex text-right"  v-if="state.isProcessing == false" >
                        <input id="js-login-btn" value="Create Account" type="submit" class="btn btnsignup mt-3 w-100" >
                    </div>
                    <div class=" text-center bg-transparent" v-if="state.isProcessing == true">
                        <span class="pd-3">Processing.. </span><span class="spinner-grow text-danger text-opacity-70 text-gradient pt-3"></span>
                    </div>
                    <p class="text-white createAccount mt-2" @click="btnLogin"><span class="text-danger">Already have an account? </span>Login</p>
                   
                </form>
            </div>
        </div>
    </div>
</template>

<style>
input:focus{
    box-shadow: 3px 2px  rgb(167, 165, 165) !important;
}

.btnsignup{
    background-image: linear-gradient(135deg,#f8f8f7 0%, #373838 100%);
    box-shadow: 2px 1px  rgb(238, 116, 116);
    border: none !important;
    color: black !important;
    font-weight: bolder !important;
}

.btnsignup:hover{
    background-image: linear-gradient(135deg,#e42929 0%, #373838 100%);
    color: white !important;
    box-shadow: 2px 1px  rgb(255, 254, 254);
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
        firstname:string().min(1,{message:'firstname is required'}).min(3,{message:'firstname should be at least 3 characters'}),
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


const {value: firstname} = useField('firstname')
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
                firstname: data.firstname.charAt(0).toUpperCase() + data.firstname.slice(1).toLowerCase(),  //capitalize first letter only
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

    const btnLogin = () => {
        router.push("/login")
    }

    watch(
        () => errors.value,
        () => {console.log(errors.value)}
        )
// };


</script>


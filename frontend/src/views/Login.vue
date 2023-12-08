<template>
    <div class="d-flex bg- h-75 align-items-center justify-content-center">
        <div class="col-sm-7 col-md-6 col-lg-5 ">
            <form class="bg-warning p-2 rounded" >
                <h2 class="h2">Log in Form</h2>
                <div>
                    <label>Email</label>
                    <div class="d-flex ">
                        <i class="fs-3 bg-dark text-white pe-3 ps-3 bi-envelope-at"></i>
                        <input type="email" v-model="form.email" class="form-control">
                    </div>
                    <p  class="text-danger"  v-if="errors?.email"><span v-for="error in errors?.email?._errors">{{ error }}</span></p>
                </div>
                <div>
                    <label>Password</label>
                    <div class="d-flex ">
                        <i class="fs-3 bg-dark text-white pe-3 ps-3 bi-lock"></i>
                        <input type="password" v-model="form.password" class="form-control">
                    </div>
                    <p class="text-danger" v-if="errors?.password"><span v-for="error in errors?.password?._errors">{{ error }}</span></p>
                </div>
                <div>
                    <div class="d-flex mt-4 mb-4 ">
                        <input type="button" value="Log In" @click="onSubmit" class="form-control bg-info"/>
                    </div>
                </div>

            </form>
        </div>
    </div>
</template>

<script setup>
import {ref} from "vue";
import {object,string} from "zod";
import {useToast} from "vue-toastification";

const toast = useToast();
const form = ref({
    email:"",
    password:"",
});

const formSchema = object({
    email:string().email(),
    password:string().min(4,{message:"pasword too short"}),
});

const errors = ref(null);

const onSubmit = () => {
    const valid = formSchema.safeParse(form.value);
    if(!valid.success){
        errors.value = valid.error.format();
        toast.error("error !!",{autoClose:1000});
    }else{
        errors.value = null;
        console.log(form.value.email)
        fetch("http://127.0.0.1:3000/login",{
            method:"POST",
            headers:{
                "Content-Type":"application/json",
            },
            body:JSON.stringify({
                email:form.value.email,
            })
        }).then((res) => res.json())
        .then((data) => {
            console.log(data)
        })
        toast.success("Log in successful" ,{autoClose:1000});
    }
};


</script>
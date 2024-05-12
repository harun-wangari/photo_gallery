import {createApp} from 'vue'
import App from './App.vue'
import router from "./router"
import Toast from "vue-toastification";
import "vue-toastification/dist/index.css"
import "./assets/main.css"

import "bootstrap/dist/css/bootstrap.css";
import "bootstrap/dist/js/bootstrap.js";
import "bootstrap-icons/font/bootstrap-icons.css"

const app = createApp(App)
app.use(router)
app.use(Toast)
app.mount('#app')
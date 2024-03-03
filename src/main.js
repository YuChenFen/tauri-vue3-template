import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createPinia } from 'pinia'
import 'virtual:svg-icons-register';
import svgIcon from "./assets/icons/svgIcon.vue";

const app = createApp(App)

const pinia = createPinia()

// 挂载全局组件
app.component("svg-icon", svgIcon)

app.use(pinia)  // 全局状态管理

app.mount("#app");

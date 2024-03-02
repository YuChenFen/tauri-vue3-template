import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

import 'virtual:svg-icons-register';
import svgIcon from "./assets/icons/svgIcon.vue";

const app = createApp(App)

// 挂载全局组件
app.component("svg-icon", svgIcon)

app.mount("#app");

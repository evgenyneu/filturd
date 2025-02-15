import { createApp } from "vue";
import "./styles/main.css";
import App from "./App.vue";
import FloatingVue from "floating-vue";

createApp(App).use(FloatingVue).mount("#app");

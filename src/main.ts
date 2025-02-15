import { createApp } from "vue";
import "./styles/main.css";
import App from "./App.vue";
import FloatingVue from "floating-vue";

let app = createApp(App);

app.use(FloatingVue, {
  distance: 10,
  themes: {
    tooltip: {
      delay: {
        show: 0,
        hide: 0,
      },
    },
  },
});

app.mount("#app");

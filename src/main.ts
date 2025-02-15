import { createApp } from "vue";
import "./styles/main.css";
import App from "./App.vue";
import FloatingVue from "floating-vue";

let app = createApp(App);

app.use(FloatingVue, {
  themes: {
    filturd: {
      $extend: "tooltip",
      delay: {
        show: 0,
        hide: 0,
      },
      distance: 10,
    },
  },
});

app.mount("#app");

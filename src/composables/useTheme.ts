import { ref, onMounted } from "vue";

export function useTheme() {
  // Initialize based on system preference, but only store 'light' or 'dark'
  const getSystemTheme = () =>
    window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";

  const theme = ref(localStorage.theme ?? getSystemTheme());

  const updateTheme = (newTheme: string) => {
    theme.value = newTheme;
    localStorage.theme = newTheme;

    if (newTheme === "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  };

  onMounted(() => {
    // Set initial theme
    updateTheme(theme.value);
  });

  return {
    theme,
    updateTheme,
  };
}

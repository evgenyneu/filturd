import { ref, onMounted, watch } from "vue";

export function useTheme() {
  const theme = ref(localStorage.theme ?? "system");

  const updateTheme = (newTheme: string) => {
    // Update theme ref and localStorage
    theme.value = newTheme;
    if (newTheme === "system") {
      localStorage.removeItem("theme");
    } else {
      localStorage.theme = newTheme;
    }

    // Update DOM
    if (
      newTheme === "dark" ||
      (newTheme === "system" &&
        window.matchMedia("(prefers-color-scheme: dark)").matches)
    ) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  };

  // Watch system theme changes when in system mode
  onMounted(() => {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaQuery.addEventListener("change", (e) => {
      if (theme.value === "system") {
        updateTheme("system");
      }
    });

    // Set initial theme
    updateTheme(theme.value);
  });

  return {
    theme,
    updateTheme,
  };
}

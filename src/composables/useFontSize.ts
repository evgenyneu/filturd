import { ref } from "vue";

export type FontSize = "normal" | "large" | "x-large";

const fontSize = ref<FontSize>("normal");

const fontSizeMap = {
  normal: "1rem",
  large: "1.25rem",
  "x-large": "1.5rem",
};

export function useFontSize() {
  const updateFontSize = (size: FontSize) => {
    fontSize.value = size;
    document.documentElement.style.setProperty(
      "--base-font-size",
      fontSizeMap[size]
    );
  };

  // Initialize on first load
  if (fontSize.value === "normal") {
    updateFontSize("normal");
  }

  return {
    fontSize,
    updateFontSize,
  };
}

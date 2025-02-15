import { ref, onMounted } from "vue";

export type FontSize = "normal" | "large" | "x-large";

const fontSize = ref<FontSize>(localStorage.fontSize ?? "normal");

const fontSizeMap = {
  normal: "1rem",
  large: "1.25rem",
  "x-large": "1.5rem",
};

export function useFontSize() {
  const updateFontSize = (size: FontSize) => {
    fontSize.value = size;
    localStorage.fontSize = size;

    document.documentElement.style.setProperty(
      "--base-font-size",
      fontSizeMap[size]
    );
  };

  const cycleFontSize = () => {
    const sizes = ["normal", "large", "x-large"] as const;
    const currentIndex = sizes.indexOf(fontSize.value);
    const nextSize = sizes[(currentIndex + 1) % sizes.length];
    updateFontSize(nextSize);
  };

  onMounted(() => {
    updateFontSize(fontSize.value);
  });

  return {
    fontSize,
    updateFontSize,
    cycleFontSize,
  };
}

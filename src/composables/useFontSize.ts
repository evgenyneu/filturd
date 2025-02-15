import { ref, onMounted } from "vue";

const fontSizeMultiplier = ref<number>(
  Number(localStorage.fontSizeMultiplier) || 1.0
);

export function useFontSize() {
  const updateFontSize = (multiplier: number) => {
    fontSizeMultiplier.value = multiplier;
    localStorage.fontSizeMultiplier = multiplier;

    document.documentElement.style.setProperty(
      "--base-font-size",
      `${fontSizeMultiplier.value}rem`
    );
  };

  const cycleFontSize = () => {
    const sizes: number[] = [1.0, 1.25, 1.5];
    const currentIndex = sizes.indexOf(fontSizeMultiplier.value);
    const nextSize = sizes[(currentIndex + 1) % sizes.length];
    updateFontSize(nextSize);
  };

  onMounted(() => {
    updateFontSize(fontSizeMultiplier.value);
  });

  return {
    fontSizeMultiplier,
    cycleFontSize,
  };
}

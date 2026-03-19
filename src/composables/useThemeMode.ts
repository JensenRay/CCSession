import { computed, ref, watch } from "vue";
import { darkTheme, useOsTheme } from "naive-ui";

export type ThemeMode = "light" | "dark";

const storageKey = "ccsession-theme-mode";

export function useThemeMode() {
  const osTheme = useOsTheme();
  const themeMode = ref<ThemeMode>(resolveInitialTheme(osTheme.value));

  const isDark = computed(() => themeMode.value === "dark");
  const naiveTheme = computed(() => (isDark.value ? darkTheme : null));

  watch(
    themeMode,
    (value) => {
      if (typeof window !== "undefined") {
        window.localStorage.setItem(storageKey, value);
      }
    },
    { immediate: true },
  );

  function setThemeMode(value: ThemeMode): void {
    themeMode.value = value;
  }

  function setDarkMode(value: boolean): void {
    setThemeMode(value ? "dark" : "light");
  }

  return {
    themeMode,
    isDark,
    naiveTheme,
    setThemeMode,
    setDarkMode,
  };
}

function resolveInitialTheme(osTheme: "light" | "dark" | null): ThemeMode {
  if (typeof window !== "undefined") {
    const storedTheme = window.localStorage.getItem(storageKey);
    if (storedTheme === "light" || storedTheme === "dark") {
      return storedTheme;
    }
  }

  return osTheme === "dark" ? "dark" : "light";
}

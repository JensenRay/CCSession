<script setup lang="ts">
import {
  NCard,
  NEl,
  NFlex,
  NRadioButton,
  NRadioGroup,
  NSpace,
  NText,
} from "naive-ui";
import type { ThemeMode } from "../composables/useThemeMode";

defineProps<{
  themeMode: ThemeMode;
}>();

defineEmits<{
  (event: "update:themeMode", value: ThemeMode): void;
}>();
</script>

<template>
  <n-flex vertical style="height: 100dvh; overflow: hidden;">
    <n-el style="flex-shrink: 0; border-bottom: 1px solid var(--n-border-color);">
      <n-flex justify="space-between" align="center" :wrap="true" style="padding: 16px;">
        <n-space vertical size="small">
          <n-text strong style="font-size: 20px;">
            Clear Codex Session
          </n-text>
          <n-text depth="3">
            Review local Codex sessions and remove the ones you no longer need.
          </n-text>
        </n-space>

        <n-radio-group
          :value="themeMode"
          size="small"
          @update:value="$emit('update:themeMode', $event)"
        >
          <n-radio-button value="light">
            Light
          </n-radio-button>
          <n-radio-button value="dark">
            Dark
          </n-radio-button>
        </n-radio-group>
      </n-flex>
    </n-el>

    <n-el style="flex: 1; min-height: 0; overflow: hidden; padding: 16px; box-sizing: border-box;">
      <n-card
        size="small"
        embedded
        style="max-width: 1200px; margin: 0 auto; width: 100%; height: 100%; min-height: 0;"
        content-style="height: 100%; display: flex; flex-direction: column; min-height: 0;"
      >
        <n-flex vertical size="large" style="flex: 1; min-height: 0; overflow: hidden;">
          <slot />
        </n-flex>
      </n-card>
    </n-el>
  </n-flex>
</template>

<script setup lang="ts">
import { NButton, NCard, NEmpty, NResult, NSpace, NSpin, NText } from "naive-ui";

defineProps<{
  state: "error" | "loading" | "empty";
  errorMessage?: string;
}>();

defineEmits<{
  (event: "retry"): void;
}>();
</script>

<template>
  <n-card size="large">
    <n-result
      v-if="state === 'error'"
      status="error"
      title="The session list could not be loaded"
      :description="errorMessage"
    >
      <template #footer>
        <n-button type="primary" @click="$emit('retry')">
          Retry
        </n-button>
      </template>
    </n-result>

    <n-space v-else-if="state === 'loading'" vertical align="center" size="large">
      <n-spin size="large" />
      <n-text depth="3">
        Scanning the local Codex session index from <code>~/.codex</code>.
      </n-text>
    </n-space>

    <n-empty v-else description="No sessions were returned.">
      <template #extra>
        <n-button @click="$emit('retry')">
          Refresh
        </n-button>
      </template>
    </n-empty>
  </n-card>
</template>

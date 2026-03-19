<script setup lang="ts">
import { NButton, NCard, NResult, NSpace, NSpin, NText } from "naive-ui";

defineProps<{
  state: "error" | "loading" | "missing";
  errorMessage?: string;
}>();

defineEmits<{
  (event: "retry"): void;
  (event: "back"): void;
}>();
</script>

<template>
  <n-card size="large">
    <n-result
      v-if="state === 'error'"
      status="error"
      title="The session detail could not be loaded"
      :description="errorMessage"
    >
      <template #footer>
        <n-space>
          <n-button type="primary" @click="$emit('retry')">
            Retry
          </n-button>
          <n-button secondary @click="$emit('back')">
            Back to List
          </n-button>
        </n-space>
      </template>
    </n-result>

    <n-space v-else-if="state === 'loading'" vertical align="center" size="large">
      <n-spin size="large" />
      <n-text depth="3">
        Waiting for the session index to finish loading before opening this record.
      </n-text>
    </n-space>

    <n-result
      v-else
      status="warning"
      title="This session is no longer available"
      description="It may have been deleted or the local index changed."
    >
      <template #footer>
        <n-space>
          <n-button secondary @click="$emit('back')">
            Back to List
          </n-button>
          <n-button @click="$emit('retry')">
            Refresh Index
          </n-button>
        </n-space>
      </template>
    </n-result>
  </n-card>
</template>

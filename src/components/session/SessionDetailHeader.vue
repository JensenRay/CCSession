<script setup lang="ts">
import { NButton, NFlex, NSpace } from "naive-ui";

defineProps<{
  selected: boolean;
  isDeletingCurrent: boolean;
  isDeleting: boolean;
  loading: boolean;
}>();

defineEmits<{
  (event: "back"): void;
  (event: "toggle-selection"): void;
  (event: "delete"): void;
  (event: "refresh"): void;
}>();
</script>

<template>
  <n-flex justify="space-between" align="center" :wrap="true">
    <n-space>
      <n-button secondary @click="$emit('back')">
        Back to List
      </n-button>
      <n-button
        :type="selected ? 'primary' : 'default'"
        :secondary="!selected"
        :disabled="isDeleting"
        :aria-pressed="selected"
        @click="$emit('toggle-selection')"
      >
        {{ selected ? "Selected" : "Select" }}
      </n-button>
    </n-space>

    <n-space>
      <n-button
        type="error"
        :loading="isDeletingCurrent"
        :disabled="isDeleting"
        @click="$emit('delete')"
      >
        Delete Session
      </n-button>
      <n-button :loading="loading" @click="$emit('refresh')">
        Refresh
      </n-button>
    </n-space>
  </n-flex>
</template>

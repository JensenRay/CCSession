<script setup lang="ts">
import { computed } from "vue";
import {
  NButton,
  NCard,
  NCheckbox,
  NEl,
  NFlex,
  NList,
  NScrollbar,
  NSpace,
  NText,
} from "naive-ui";
import type { SessionListItem } from "../../types";
import SessionListItemRow from "./SessionListItem.vue";

const props = defineProps<{
  sessions: SessionListItem[];
  selectedIds: string[];
  canBulkDelete: boolean;
  isDeleting: boolean;
  loading: boolean;
}>();

defineEmits<{
  (event: "refresh"): void;
  (event: "bulk-delete"): void;
  (event: "toggle-all"): void;
  (event: "toggle-session", sessionId: string): void;
  (event: "open-session", sessionId: string): void;
}>();

const allSelected = computed(
  () =>
    props.sessions.length > 0 &&
    props.selectedIds.length === props.sessions.length,
);

const partiallySelected = computed(
  () => props.selectedIds.length > 0 && !allSelected.value,
);
</script>

<template>
  <n-card
    size="large"
    style="height: 100%; min-height: 0;"
    content-style="height: 100%; display: flex; flex-direction: column; min-height: 0; overflow: hidden;"
  >
    <n-flex vertical style="flex: 1; min-height: 0; gap: 16px;">
      <n-flex justify="space-between" align="center" :wrap="true">
        <n-space vertical size="small">
          <n-text strong>
            Session List
          </n-text>
          <n-text depth="3">
            {{ selectedIds.length }} selected / {{ sessions.length }} total
          </n-text>
        </n-space>

        <n-space>
          <n-button type="error" :disabled="!canBulkDelete" @click="$emit('bulk-delete')">
            Delete Selected
          </n-button>
          <n-button :loading="loading" @click="$emit('refresh')">
            Refresh
          </n-button>
        </n-space>
      </n-flex>

      <n-checkbox
        :checked="allSelected"
        :indeterminate="partiallySelected"
        :disabled="isDeleting || !sessions.length"
        @update:checked="$emit('toggle-all')"
      >
        Select All
      </n-checkbox>

      <n-el style="flex: 1; min-height: 0; overflow: hidden;">
        <n-scrollbar style="height: 100%;">
          <n-list bordered hoverable clickable>
            <SessionListItemRow
              v-for="session in sessions"
              :key="session.id"
              :session="session"
              :selected="selectedIds.includes(session.id)"
              :disabled="isDeleting"
              @open="$emit('open-session', session.id)"
              @toggle="$emit('toggle-session', session.id)"
            />
          </n-list>
        </n-scrollbar>
      </n-el>
    </n-flex>
  </n-card>
</template>

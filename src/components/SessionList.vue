<script setup lang="ts">
import SessionRow from "./SessionRow.vue";
import type { SessionListItem } from "../types";

defineProps<{
  sessions: SessionListItem[];
  activeDeleteIds: string[];
  deleteDisabled?: boolean;
}>();

const emit = defineEmits<{
  (event: "request-delete", sessionId: string): void;
}>();
</script>

<template>
  <section class="session-list" aria-label="Codex sessions">
    <SessionRow
      v-for="session in sessions"
      :key="session.id"
      :session="session"
      :delete-disabled="deleteDisabled"
      :is-deleting="activeDeleteIds.includes(session.id)"
      @request-delete="emit('request-delete', $event)"
    />
  </section>
</template>

<style scoped>
.session-list {
  display: grid;
  gap: 1rem;
}
</style>

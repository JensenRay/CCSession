<script setup lang="ts">
import SessionRow from "./SessionRow.vue";
import type { SessionListItem } from "../types";

defineProps<{
  sessions: SessionListItem[];
}>();
</script>

<template>
  <section class="session-list" aria-label="Codex sessions">
    <SessionRow
      v-for="session in sessions"
      :key="session.id"
      :session="session"
    />
  </section>
</template>

<style scoped>
.session-list {
  display: grid;
  gap: 1rem;
}
</style>

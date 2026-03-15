<script setup lang="ts">
import { computed, ref, watchEffect } from "vue";
import SessionRow from "./SessionRow.vue";
import type { SessionListItem } from "../types";

const props = defineProps<{
  sessions: SessionListItem[];
  selectedIds: string[];
  activeSessionId: string;
  selectionDisabled?: boolean;
}>();

const emit = defineEmits<{
  (event: "toggle-all"): void;
  (event: "toggle-select", sessionId: string): void;
  (event: "open-session", sessionId: string): void;
}>();

const selectAllRef = ref<HTMLInputElement | null>(null);
const allSelected = computed(
  () =>
    props.sessions.length > 0 &&
    props.selectedIds.length === props.sessions.length,
);
const partiallySelected = computed(
  () => props.selectedIds.length > 0 && !allSelected.value,
);

watchEffect(() => {
  if (selectAllRef.value) {
    selectAllRef.value.indeterminate = partiallySelected.value;
  }
});
</script>

<template>
  <aside class="session-list" aria-label="Codex sessions">
    <header class="session-list__header">
      <div>
        <p class="session-list__eyebrow">Session List</p>
        <h2 class="session-list__title">Compact cards first</h2>
      </div>
      <p class="session-list__summary">
        {{ selectedIds.length }} selected / {{ sessions.length }} total
      </p>
    </header>

    <label class="session-list__select-all">
      <input
        ref="selectAllRef"
        type="checkbox"
        :checked="allSelected"
        :disabled="selectionDisabled || !sessions.length"
        @change="emit('toggle-all')"
      />
      <span>Select every visible session</span>
    </label>

    <div class="session-list__items">
      <SessionRow
        v-for="session in sessions"
        :key="session.id"
        :session="session"
        :selected="selectedIds.includes(session.id)"
        :is-active="activeSessionId === session.id"
        :selection-disabled="selectionDisabled"
        @toggle-select="emit('toggle-select', $event)"
        @open-session="emit('open-session', $event)"
      />
    </div>
  </aside>
</template>

<style scoped>
.session-list {
  position: sticky;
  top: 1rem;
  display: grid;
  gap: 1rem;
  padding: 1.15rem;
  border-radius: 1.55rem;
  border: 1px solid var(--border);
  background:
    linear-gradient(180deg, rgba(12, 29, 22, 0.96), rgba(8, 18, 15, 0.98)),
    var(--panel);
  box-shadow: 0 24px 60px rgba(1, 8, 6, 0.34);
}

.session-list__header {
  display: grid;
  gap: 0.5rem;
}

.session-list__eyebrow {
  margin: 0;
  color: var(--accent-soft);
  font-size: 0.74rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
}

.session-list__title {
  margin: 0;
  color: var(--heading);
  font-size: 1.15rem;
}

.session-list__summary {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.6;
}

.session-list__select-all {
  display: inline-flex;
  gap: 0.7rem;
  align-items: center;
  padding: 0.9rem 1rem;
  border-radius: 1rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(112, 193, 174, 0.12);
  color: var(--heading);
  font-weight: 700;
}

.session-list__items {
  display: grid;
  gap: 0.85rem;
  max-height: calc(100vh - 13rem);
  overflow: auto;
  padding-right: 0.2rem;
}

.session-list__items::-webkit-scrollbar {
  width: 8px;
}

.session-list__items::-webkit-scrollbar-thumb {
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.12);
}

@media (max-width: 980px) {
  .session-list {
    position: static;
  }

  .session-list__items {
    max-height: none;
  }
}
</style>

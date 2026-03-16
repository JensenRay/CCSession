<script setup lang="ts">
import { computed, ref, watchEffect } from "vue";
import type { SessionListItem } from "../types";
import SessionRow from "./SessionRow.vue";

const props = defineProps<{
  sessions: SessionListItem[];
  selectedIds: string[];
  selectionDisabled?: boolean;
  loading?: boolean;
  canBulkDelete?: boolean;
}>();

const emit = defineEmits<{
  (event: "toggle-all"): void;
  (event: "toggle-select", sessionId: string): void;
  (event: "open-session", sessionId: string): void;
  (event: "refresh"): void;
  (event: "request-batch-delete"): void;
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
      <div class="session-list__header-copy">
        <p class="session-list__eyebrow">Session List</p>
        <p class="session-list__summary">
          {{ selectedIds.length }} selected / {{ sessions.length }} total
        </p>
      </div>

      <div class="session-list__toolbar">
        <button class="ui-button ui-button--danger" type="button" :disabled="!canBulkDelete"
          @click="emit('request-batch-delete')">
          Delete Selected
        </button>
        <button class="ui-button" type="button" :disabled="loading" @click="emit('refresh')">
          {{ loading ? "Refreshing..." : "Refresh" }}
        </button>
      </div>
    </header>

    <label class="session-list__select-all">
      <input ref="selectAllRef" type="checkbox" :checked="allSelected" :disabled="selectionDisabled || !sessions.length"
        @change="emit('toggle-all')" />
      <span>Select ALL</span>
    </label>

    <div class="session-list__items">
      <SessionRow v-for="session in sessions" :key="session.id" :session="session"
        :selected="selectedIds.includes(session.id)" :selection-disabled="selectionDisabled"
        @toggle-select="emit('toggle-select', $event)" @open-session="emit('open-session', $event)" />
    </div>
  </aside>
</template>

<style scoped>
.session-list {
  position: sticky;
  top: 1rem;
  display: grid;
  grid-template-rows: auto auto minmax(0, 1fr);
  gap: 1rem;
  height: 100%;
  min-height: 0;
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
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 1rem;
  align-items: start;
}

.session-list__header-copy {
  min-width: 0;
  display: grid;
  gap: 0.35rem;
}

.session-list__eyebrow {
  margin: 0;
  color: var(--accent-soft);
  font-size: 0.74rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
}

.session-list__summary {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.6;
}

.session-list__toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  justify-content: flex-end;
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
  min-height: 0;
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
    height: auto;
  }

  .session-list__header {
    grid-template-columns: minmax(0, 1fr) auto;
  }

  .session-list__items {
    max-height: none;
  }
}

@media (max-width: 560px) {
  .session-list__toolbar {
    flex-wrap: nowrap;
  }

  .session-list__toolbar .ui-button {
    flex: 0 1 auto;
    min-width: 0;
    padding-inline: 0.95rem;
  }
}

@media (max-height: 860px) {
  .session-list {
    gap: 0.85rem;
    padding: 1rem;
  }

  .session-list__select-all {
    padding: 0.78rem 0.9rem;
  }
}
</style>

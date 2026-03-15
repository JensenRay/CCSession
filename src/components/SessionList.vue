<script setup lang="ts">
import { computed, ref, watchEffect } from "vue";
import SessionRow from "./SessionRow.vue";
import type { SessionListItem } from "../types";

const props = defineProps<{
  sessions: SessionListItem[];
  selectedIds: string[];
  activeDeleteIds: string[];
  selectionDisabled?: boolean;
  deleteDisabled?: boolean;
}>();

const emit = defineEmits<{
  (event: "toggle-all"): void;
  (event: "toggle-select", sessionId: string): void;
  (event: "request-delete", sessionId: string): void;
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
  <section class="session-list" aria-label="Codex sessions">
    <header class="session-list__toolbar">
      <label class="session-list__select-all">
        <input
          ref="selectAllRef"
          type="checkbox"
          :checked="allSelected"
          :disabled="selectionDisabled || !sessions.length"
          @change="emit('toggle-all')"
        />
        <span>Select all</span>
      </label>

      <p class="session-list__summary">
        {{ selectedIds.length }} selected / {{ sessions.length }} total
      </p>
    </header>

    <SessionRow
      v-for="session in sessions"
      :key="session.id"
      :session="session"
      :selected="selectedIds.includes(session.id)"
      :selection-disabled="selectionDisabled"
      :delete-disabled="deleteDisabled"
      :is-deleting="activeDeleteIds.includes(session.id)"
      @toggle-select="emit('toggle-select', $event)"
      @request-delete="emit('request-delete', $event)"
    />
  </section>
</template>

<style scoped>
.session-list {
  display: grid;
  gap: 1rem;
}

.session-list__toolbar {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: center;
  padding: 0.95rem 1rem;
  border-radius: 1rem;
  background: rgba(255, 255, 255, 0.56);
  border: 1px solid rgba(105, 74, 48, 0.12);
}

.session-list__select-all {
  display: inline-flex;
  gap: 0.65rem;
  align-items: center;
  color: var(--heading);
  font-weight: 700;
}

.session-list__summary {
  margin: 0;
  color: var(--text-muted);
}

@media (max-width: 640px) {
  .session-list__toolbar {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>

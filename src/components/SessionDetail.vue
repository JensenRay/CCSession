<script setup lang="ts">
import { computed } from "vue";
import type { SessionListItem } from "../types";
import {
  formatCount,
  formatFullTimestamp,
} from "../utils/sessionFormat";

const props = defineProps<{
  session: SessionListItem | null;
  loading?: boolean;
  selected?: boolean;
  selectionDisabled?: boolean;
  deleteDisabled?: boolean;
  isDeleting?: boolean;
  promptEntries: string[];
  promptEntriesLoading?: boolean;
  promptEntriesError?: string;
  promptEntriesWarnings?: string[];
}>();

const emit = defineEmits<{
  (event: "go-back"): void;
  (event: "refresh"): void;
  (event: "toggle-select", sessionId: string): void;
  (event: "request-delete", sessionId: string): void;
}>();
const updatedAtLabel = computed(() =>
  props.session ? formatFullTimestamp(props.session.updatedAt) : "",
);
const createdAtLabel = computed(() =>
  props.session ? formatFullTimestamp(props.session.createdAt) : "",
);
const metaItems = computed(() => {
  if (!props.session) {
    return [];
  }

  return [
    { label: "Session ID", value: props.session.id },
    { label: "Project", value: props.session.cwd },
    { label: "Created", value: createdAtLabel.value },
    { label: "Updated", value: updatedAtLabel.value },
    { label: "History entries", value: formatCount(props.session.historyCount) },
    { label: "Tokens used", value: formatCount(props.session.tokensUsed) },
    { label: "Source", value: props.session.source || "Unknown" },
    { label: "Provider", value: props.session.modelProvider || "Unknown" },
  ];
});
</script>

<template>
  <section v-if="session" class="session-detail">
    <div class="session-detail__toolbar">
      <div class="session-detail__toolbar-group">
        <button class="ui-button ui-button--ghost" type="button" @click="emit('go-back')">
          Back to List
        </button>
        <label class="session-detail__toggle">
          <input type="checkbox" :checked="selected" :disabled="selectionDisabled"
            @change="emit('toggle-select', session.id)" />
          <span>Select</span>
        </label>
      </div>

      <div class="session-detail__toolbar-group session-detail__toolbar-group--end">
        <button class="ui-button ui-button--danger session-detail__delete" type="button" :disabled="deleteDisabled"
          @click="emit('request-delete', session.id)">
          {{ isDeleting ? "Deleting..." : "Delete Session" }}
        </button>

        <button class="ui-button" type="button" :disabled="loading" @click="emit('refresh')">
          {{ loading ? "Refreshing..." : "Refresh" }}
        </button>
      </div>
    </div>

    <section class="session-detail__section session-detail__section--history">
      <div class="session-detail__section-header">
        <h3>Prompt History</h3>
        <span>{{ promptEntries.length }} prompts</span>
      </div>

      <p v-if="promptEntriesLoading" class="session-detail__inline-note">
        Loading the full prompt history...
      </p>
      <p v-else-if="promptEntriesError" class="session-detail__inline-note session-detail__inline-note--error">
        {{ promptEntriesError }}
      </p>
      <ul v-else-if="promptEntries.length" class="session-detail__preview-list">
        <li v-for="(item, index) in promptEntries" :key="`${session.id}-${index}`"
          class="session-detail__preview-item">
          {{ item }}
        </li>
      </ul>
      <p v-else class="session-detail__empty">
        No prompt entries were returned for this session.
      </p>
      <ul v-if="promptEntriesWarnings?.length" class="session-detail__warning-list">
        <li v-for="warning in promptEntriesWarnings" :key="warning">{{ warning }}</li>
      </ul>
    </section>

    <section class="session-detail__section session-detail__section--metadata">
      <div class="session-detail__section-header">
        <h3>Metadata</h3>
      </div>

      <dl class="session-detail__meta-grid">
        <div v-for="item in metaItems" :key="item.label" class="session-detail__meta-item">
          <dt>{{ item.label }}</dt>
          <dd>{{ item.value }}</dd>
        </div>
      </dl>
    </section>
  </section>
</template>

<style scoped>
.session-detail {
  --history-panel-share: 56%;
  display: grid;
  grid-template-rows: auto minmax(0, var(--history-panel-share)) minmax(0, 1fr);
  gap: 0.75rem;
  height: 100%;
  min-height: 0;
  padding: 0.9rem 1.15rem 1.1rem;
  border-radius: 1.55rem;
  border: 1px solid var(--border);
  background:
    linear-gradient(180deg, rgba(13, 31, 24, 0.96), rgba(8, 18, 15, 0.98)),
    var(--panel);
  box-shadow: 0 26px 70px rgba(1, 8, 6, 0.34);
}

.session-detail__toolbar {
  display: flex;
  justify-content: space-between;
  gap: 0.85rem;
  align-items: center;
}

.session-detail__toolbar-group {
  display: flex;
  flex-wrap: wrap;
  gap: 0.65rem;
  align-items: center;
}

.session-detail__toolbar-group--end {
  justify-content: flex-end;
}

.session-detail__toggle {
  display: inline-flex;
  gap: 0.65rem;
  align-items: center;
  min-height: 2.9rem;
  padding: 0 1rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(112, 193, 174, 0.14);
  color: var(--heading);
  font-weight: 700;
}

.session-detail__delete:disabled {
  cursor: wait;
}

.session-detail__section,
.session-detail__meta-item {
  border-radius: 1.1rem;
  border: 1px solid rgba(112, 193, 174, 0.12);
  background: rgba(255, 255, 255, 0.04);
}

.session-detail__section {
  display: grid;
  gap: 0.85rem;
  min-height: 0;
  padding: 0.9rem;
}

.session-detail__section--history {
  display: flex;
  flex-direction: column;
}

.session-detail__section--metadata {
  grid-template-rows: auto minmax(0, 1fr);
  overflow: hidden;
}

.session-detail__section-header {
  display: flex;
  justify-content: space-between;
  gap: 0.75rem;
  align-items: center;
}

.session-detail__section-header h3,
.session-detail__section-header span {
  margin: 0;
}

.session-detail__section-header h3 {
  color: var(--heading);
  font-size: 1rem;
}

.session-detail__section-header span {
  color: var(--text-muted);
  font-size: 0.82rem;
}

.session-detail__preview-list {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  gap: 0.7rem;
  margin: 0;
  padding: 0;
  list-style: none;
  min-height: 0;
  overflow: auto;
  padding-right: 0.25rem;
}

.session-detail__preview-list::-webkit-scrollbar {
  width: 8px;
}

.session-detail__preview-list::-webkit-scrollbar-thumb {
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.12);
}

.session-detail__preview-item,
.session-detail__empty {
  margin: 0;
  padding: 0.85rem 0.95rem;
  border-radius: 0.95rem;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(112, 193, 174, 0.1);
  color: var(--text-soft);
  line-height: 1.7;
}

.session-detail__inline-note {
  margin: 0;
  color: var(--text-muted);
  line-height: 1.6;
}

.session-detail__inline-note--error {
  color: var(--danger);
}

.session-detail__warning-list {
  margin: 0;
  padding-left: 1.1rem;
  color: var(--text-muted);
}

.session-detail__meta-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.75rem;
  margin: 0;
  min-height: 0;
  overflow: auto;
  padding-right: 0.2rem;
}

.session-detail__meta-grid::-webkit-scrollbar {
  width: 8px;
}

.session-detail__meta-grid::-webkit-scrollbar-thumb {
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.12);
}

.session-detail__meta-item {
  padding: 0.9rem 1rem;
}

.session-detail__meta-item dt {
  color: var(--text-muted);
  font-size: 0.76rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.session-detail__meta-item dd {
  margin: 0.45rem 0 0;
  color: var(--heading);
  line-height: 1.65;
  word-break: break-word;
}

@media (max-height: 860px) {
  .session-detail {
    --history-panel-share: 52%;
    gap: 0.65rem;
    padding: 0.75rem 0.95rem 0.95rem;
  }

  .session-detail__section {
    gap: 0.7rem;
    padding: 0.82rem;
  }
}

@media (max-width: 760px) {
  .session-detail {
    padding: 1rem;
  }

  .session-detail__toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .session-detail__toolbar-group {
    width: 100%;
  }

  .session-detail__toolbar-group--end {
    justify-content: flex-end;
  }

  .session-detail__toolbar-group :deep(.ui-button) {
    flex: 1 1 0;
  }

  .session-detail__meta-grid {
    grid-template-columns: minmax(0, 1fr);
  }

  .session-detail__section-header {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>

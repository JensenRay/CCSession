<script setup lang="ts">
import { computed } from "vue";
import type { SessionListItem } from "../types";

const props = defineProps<{
  session: SessionListItem;
  deleteDisabled?: boolean;
  isDeleting?: boolean;
}>();

const emit = defineEmits<{
  (event: "request-delete", sessionId: string): void;
}>();

const updatedAtLabel = computed(() => formatTimestamp(props.session.updatedAt));
const createdAtLabel = computed(() => formatTimestamp(props.session.createdAt));
const title = computed(() => props.session.title.trim() || props.session.id);
const summary = computed(
  () => props.session.summary.trim() || "No summary available.",
);

function formatTimestamp(timestamp: number): string {
  return new Intl.DateTimeFormat("zh-CN", {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(timestamp * 1000);
}

function formatCount(value: number): string {
  return new Intl.NumberFormat("en-US").format(value);
}
</script>

<template>
  <article class="session-row">
    <header class="session-row__header">
      <div class="session-row__title-group">
        <p class="session-row__eyebrow">Session</p>
        <h3 class="session-row__title">{{ title }}</h3>
      </div>
      <div class="session-row__header-actions">
        <div class="session-row__badges">
          <span v-if="session.archived" class="session-row__badge">Archived</span>
          <span class="session-row__status" :data-available="session.hasRollout">
            {{ session.hasRollout ? "Rollout ready" : "Rollout missing" }}
          </span>
          <span class="session-row__status" :data-available="session.hasSnapshot">
            {{ session.hasSnapshot ? "Snapshot ready" : "Snapshot missing" }}
          </span>
        </div>

        <button
          class="session-row__delete-button"
          type="button"
          :disabled="deleteDisabled"
          @click="emit('request-delete', session.id)"
        >
          {{ isDeleting ? "Deleting..." : "Delete" }}
        </button>
      </div>
    </header>

    <p class="session-row__summary">{{ summary }}</p>

    <ul v-if="session.contentPreview.length" class="session-row__preview-list">
      <li v-for="item in session.contentPreview" :key="item" class="session-row__preview-item">
        {{ item }}
      </li>
    </ul>
    <p v-else class="session-row__preview-empty">No preview messages were returned.</p>

    <dl class="session-row__meta-grid">
      <div class="session-row__meta-item">
        <dt>ID</dt>
        <dd>{{ session.id }}</dd>
      </div>
      <div class="session-row__meta-item">
        <dt>Project</dt>
        <dd>{{ session.cwd }}</dd>
      </div>
      <div class="session-row__meta-item">
        <dt>Updated</dt>
        <dd>{{ updatedAtLabel }}</dd>
      </div>
      <div class="session-row__meta-item">
        <dt>Created</dt>
        <dd>{{ createdAtLabel }}</dd>
      </div>
      <div class="session-row__meta-item">
        <dt>Provider</dt>
        <dd>{{ session.modelProvider || "Unknown" }}</dd>
      </div>
      <div class="session-row__meta-item">
        <dt>Source</dt>
        <dd>{{ session.source || "Unknown" }}</dd>
      </div>
      <div class="session-row__meta-item">
        <dt>Tokens</dt>
        <dd>{{ formatCount(session.tokensUsed) }}</dd>
      </div>
      <div class="session-row__meta-item">
        <dt>History</dt>
        <dd>{{ formatCount(session.historyCount) }}</dd>
      </div>
      <div class="session-row__meta-item">
        <dt>Structured logs</dt>
        <dd>{{ formatCount(session.structuredLogCount) }}</dd>
      </div>
    </dl>
  </article>
</template>

<style scoped>
.session-row {
  display: grid;
  gap: 1rem;
  padding: 1.25rem;
  border: 1px solid rgba(105, 74, 48, 0.12);
  border-radius: 1rem;
  background:
    linear-gradient(180deg, rgba(255, 252, 247, 0.98), rgba(247, 240, 232, 0.94)),
    var(--panel);
  box-shadow: 0 14px 40px rgba(61, 45, 31, 0.08);
}

.session-row__header {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: flex-start;
}

.session-row__title-group {
  display: grid;
  gap: 0.35rem;
}

.session-row__eyebrow {
  margin: 0;
  font-size: 0.72rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: var(--accent);
}

.session-row__title {
  margin: 0;
  color: var(--heading);
  font-size: 1.15rem;
  line-height: 1.4;
  word-break: break-word;
}

.session-row__badges {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  justify-content: flex-end;
}

.session-row__header-actions {
  display: grid;
  justify-items: end;
  gap: 0.75rem;
}

.session-row__delete-button {
  min-height: 2.5rem;
  padding: 0 0.95rem;
  border: none;
  border-radius: 999px;
  background: rgba(139, 61, 39, 0.1);
  color: var(--danger);
  font-weight: 700;
}

.session-row__delete-button:disabled {
  opacity: 0.64;
  cursor: wait;
}

.session-row__badge,
.session-row__status {
  display: inline-flex;
  align-items: center;
  min-height: 2rem;
  padding: 0 0.75rem;
  border-radius: 999px;
  font-size: 0.8rem;
  font-weight: 600;
}

.session-row__badge {
  background: rgba(159, 112, 0, 0.14);
  color: #805300;
}

.session-row__status {
  background: rgba(67, 98, 74, 0.1);
  color: #315d3c;
}

.session-row__status[data-available="false"] {
  background: rgba(132, 56, 37, 0.12);
  color: #8c3b28;
}

.session-row__summary {
  margin: 0;
  color: var(--text);
  font-size: 0.98rem;
  line-height: 1.7;
}

.session-row__preview-list {
  display: grid;
  gap: 0.65rem;
  margin: 0;
  padding: 0;
  list-style: none;
}

.session-row__preview-item,
.session-row__preview-empty {
  margin: 0;
  padding: 0.85rem 0.95rem;
  border-radius: 0.85rem;
  background: rgba(255, 255, 255, 0.72);
  border: 1px solid rgba(105, 74, 48, 0.12);
  color: var(--text-soft);
  line-height: 1.6;
}

.session-row__meta-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 0.9rem;
  margin: 0;
}

.session-row__meta-item {
  min-width: 0;
  padding: 0.95rem;
  border-radius: 0.85rem;
  background: rgba(121, 92, 62, 0.06);
}

.session-row__meta-item dt {
  margin: 0 0 0.4rem;
  font-size: 0.78rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--text-muted);
}

.session-row__meta-item dd {
  margin: 0;
  color: var(--heading);
  line-height: 1.6;
  word-break: break-word;
}

@media (max-width: 680px) {
  .session-row {
    padding: 1rem;
  }

  .session-row__header {
    flex-direction: column;
  }

  .session-row__badges {
    justify-content: flex-start;
  }

  .session-row__header-actions {
    justify-items: start;
  }
}
</style>

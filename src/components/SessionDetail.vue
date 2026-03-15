<script setup lang="ts">
import { computed } from "vue";
import type { SessionListItem } from "../types";

const props = defineProps<{
  session: SessionListItem | null;
  selected?: boolean;
  selectionDisabled?: boolean;
  deleteDisabled?: boolean;
  isDeleting?: boolean;
}>();

const emit = defineEmits<{
  (event: "toggle-select", sessionId: string): void;
  (event: "request-delete", sessionId: string): void;
}>();

const title = computed(
  () => props.session?.title.trim() || props.session?.id || "No session selected",
);
const summary = computed(
  () => props.session?.summary.trim() || "No summary available.",
);
const updatedAtLabel = computed(() =>
  props.session ? formatTimestamp(props.session.updatedAt) : "",
);
const createdAtLabel = computed(() =>
  props.session ? formatTimestamp(props.session.createdAt) : "",
);
const detailBadges = computed(() => {
  if (!props.session) {
    return [];
  }

  return [
    {
      label: props.session.archived ? "Archived" : "Live session",
      tone: props.session.archived ? "muted" : "neutral",
    },
    {
      label: props.session.hasRollout ? "Rollout ready" : "Rollout missing",
      tone: props.session.hasRollout ? "neutral" : "warning",
    },
    {
      label: props.session.hasSnapshot ? "Snapshot ready" : "Snapshot missing",
      tone: props.session.hasSnapshot ? "neutral" : "warning",
    },
    {
      label: props.session.modelProvider || "Unknown provider",
      tone: "accent",
    },
  ];
});
const statCards = computed(() => {
  if (!props.session) {
    return [];
  }

  return [
    { label: "History entries", value: formatCount(props.session.historyCount) },
    { label: "Structured logs", value: formatCount(props.session.structuredLogCount) },
    { label: "Tokens used", value: formatCount(props.session.tokensUsed) },
    { label: "Updated", value: updatedAtLabel.value },
  ];
});
const metaItems = computed(() => {
  if (!props.session) {
    return [];
  }

  return [
    { label: "Session ID", value: props.session.id },
    { label: "Project", value: props.session.cwd },
    { label: "Created", value: createdAtLabel.value },
    { label: "Updated", value: updatedAtLabel.value },
    { label: "Source", value: props.session.source || "Unknown" },
    { label: "Provider", value: props.session.modelProvider || "Unknown" },
  ];
});

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
  <section v-if="session" class="session-detail">
    <header class="session-detail__header">
      <div class="session-detail__header-copy">
        <p class="session-detail__eyebrow">Session Detail</p>
        <h2 class="session-detail__title">{{ title }}</h2>
        <p class="session-detail__summary">{{ summary }}</p>
      </div>

      <div class="session-detail__actions">
        <label class="session-detail__toggle">
          <input
            type="checkbox"
            :checked="selected"
            :disabled="selectionDisabled"
            @change="emit('toggle-select', session.id)"
          />
          <span>Include in batch delete</span>
        </label>

        <button
          class="session-detail__delete"
          type="button"
          :disabled="deleteDisabled"
          @click="emit('request-delete', session.id)"
        >
          {{ isDeleting ? "Deleting..." : "Delete Session" }}
        </button>
      </div>
    </header>

    <div class="session-detail__badge-row">
      <span
        v-for="badge in detailBadges"
        :key="badge.label"
        class="session-detail__badge"
        :data-tone="badge.tone"
      >
        {{ badge.label }}
      </span>
    </div>

    <section class="session-detail__stats">
      <article
        v-for="stat in statCards"
        :key="stat.label"
        class="session-detail__stat-card"
      >
        <span class="session-detail__stat-label">{{ stat.label }}</span>
        <strong class="session-detail__stat-value">{{ stat.value }}</strong>
      </article>
    </section>

    <section class="session-detail__section">
      <div class="session-detail__section-header">
        <h3>Recent prompt preview</h3>
        <span>{{ session.contentPreview.length }} entries</span>
      </div>

      <ul v-if="session.contentPreview.length" class="session-detail__preview-list">
        <li
          v-for="item in session.contentPreview"
          :key="item"
          class="session-detail__preview-item"
        >
          {{ item }}
        </li>
      </ul>
      <p v-else class="session-detail__empty">
        No preview lines were returned for this session.
      </p>
    </section>

    <section class="session-detail__section">
      <div class="session-detail__section-header">
        <h3>Metadata</h3>
        <span>Expanded view</span>
      </div>

      <dl class="session-detail__meta-grid">
        <div
          v-for="item in metaItems"
          :key="item.label"
          class="session-detail__meta-item"
        >
          <dt>{{ item.label }}</dt>
          <dd>{{ item.value }}</dd>
        </div>
      </dl>
    </section>
  </section>
</template>

<style scoped>
.session-detail {
  display: grid;
  gap: 1rem;
  padding: 1.25rem;
  border-radius: 1.55rem;
  border: 1px solid var(--border);
  background:
    linear-gradient(180deg, rgba(13, 31, 24, 0.96), rgba(8, 18, 15, 0.98)),
    var(--panel);
  box-shadow: 0 26px 70px rgba(1, 8, 6, 0.34);
}

.session-detail__header {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: flex-start;
}

.session-detail__header-copy {
  display: grid;
  gap: 0.55rem;
}

.session-detail__eyebrow {
  margin: 0;
  color: var(--accent-soft);
  font-size: 0.78rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
}

.session-detail__title {
  margin: 0;
  color: var(--heading);
  font-size: clamp(1.5rem, 3vw, 2.35rem);
  line-height: 1.08;
}

.session-detail__summary {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.75;
}

.session-detail__actions {
  display: grid;
  gap: 0.75rem;
  min-width: 240px;
}

.session-detail__toggle {
  display: inline-flex;
  gap: 0.65rem;
  align-items: center;
  padding: 0.9rem 1rem;
  border-radius: 1rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(112, 193, 174, 0.14);
  color: var(--heading);
  font-weight: 700;
}

.session-detail__delete {
  min-height: 2.9rem;
  padding: 0 1rem;
  border: none;
  border-radius: 999px;
  background: linear-gradient(135deg, var(--accent), #ffb114);
  color: #05110d;
  font-weight: 800;
}

.session-detail__delete:disabled {
  opacity: 0.45;
  cursor: wait;
}

.session-detail__badge-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.65rem;
}

.session-detail__badge {
  display: inline-flex;
  align-items: center;
  min-height: 2rem;
  padding: 0 0.78rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.06);
  color: var(--heading);
  font-size: 0.82rem;
  font-weight: 700;
}

.session-detail__badge[data-tone="accent"] {
  background: rgba(55, 193, 218, 0.14);
  color: var(--accent-soft);
}

.session-detail__badge[data-tone="warning"] {
  background: rgba(255, 201, 71, 0.16);
  color: var(--accent);
}

.session-detail__badge[data-tone="muted"] {
  background: rgba(255, 255, 255, 0.07);
  color: var(--text-muted);
}

.session-detail__stats {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 0.8rem;
}

.session-detail__stat-card,
.session-detail__section,
.session-detail__meta-item {
  border-radius: 1.1rem;
  border: 1px solid rgba(112, 193, 174, 0.12);
  background: rgba(255, 255, 255, 0.04);
}

.session-detail__stat-card {
  display: grid;
  gap: 0.45rem;
  padding: 1rem;
}

.session-detail__stat-label {
  color: var(--text-muted);
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.session-detail__stat-value {
  color: var(--heading);
  font-size: 1.02rem;
  line-height: 1.4;
}

.session-detail__section {
  display: grid;
  gap: 1rem;
  padding: 1rem;
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
  display: grid;
  gap: 0.75rem;
  margin: 0;
  padding: 0;
  list-style: none;
}

.session-detail__preview-item,
.session-detail__empty {
  margin: 0;
  padding: 0.9rem 1rem;
  border-radius: 1rem;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(112, 193, 174, 0.1);
  color: var(--text-soft);
  line-height: 1.7;
}

.session-detail__meta-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.8rem;
  margin: 0;
}

.session-detail__meta-item {
  padding: 0.95rem;
}

.session-detail__meta-item dt {
  margin: 0 0 0.4rem;
  color: var(--text-muted);
  font-size: 0.76rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
}

.session-detail__meta-item dd {
  margin: 0;
  color: var(--heading);
  line-height: 1.7;
  word-break: break-word;
}

@media (max-width: 1120px) {
  .session-detail__stats {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 760px) {
  .session-detail {
    padding: 1rem;
    border-radius: 1.2rem;
  }

  .session-detail__header {
    flex-direction: column;
  }

  .session-detail__actions {
    width: 100%;
    min-width: 0;
  }

  .session-detail__stats,
  .session-detail__meta-grid {
    grid-template-columns: minmax(0, 1fr);
  }

  .session-detail__section-header {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>

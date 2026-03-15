<script setup lang="ts">
import { computed } from "vue";
import type { SessionListItem } from "../types";

const props = defineProps<{
  session: SessionListItem;
  selected?: boolean;
  isActive?: boolean;
  selectionDisabled?: boolean;
}>();

const emit = defineEmits<{
  (event: "toggle-select", sessionId: string): void;
  (event: "open-session", sessionId: string): void;
}>();

const updatedAtLabel = computed(() => formatTimestamp(props.session.updatedAt));
const title = computed(() => props.session.title.trim() || props.session.id);
const summary = computed(
  () => props.session.summary.trim() || "No summary available.",
);
const previewLine = computed(
  () =>
    props.session.contentPreview[props.session.contentPreview.length - 1]?.trim() ||
    "",
);
const condensedPath = computed(() => compactPath(props.session.cwd));
const stateLabel = computed(() => {
  if (!props.session.hasRollout || !props.session.hasSnapshot) {
    return "Needs review";
  }

  if (props.session.archived) {
    return "Archived";
  }

  return "Ready";
});

function formatTimestamp(timestamp: number): string {
  return new Intl.DateTimeFormat("zh-CN", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(timestamp * 1000);
}

function formatCount(value: number): string {
  return new Intl.NumberFormat("en-US", {
    notation: value > 999 ? "compact" : "standard",
  }).format(value);
}

function compactPath(path: string): string {
  const normalized = path.replace(/\\/g, "/");
  const segments = normalized.split("/").filter(Boolean);

  if (segments.length <= 2) {
    return path;
  }

  return `…/${segments.slice(-2).join("/")}`;
}
</script>

<template>
  <article
    class="session-card"
    :data-active="isActive"
    :data-selected="selected"
  >
    <div class="session-card__top">
      <label class="session-card__checkbox">
        <input
          type="checkbox"
          :checked="selected"
          :disabled="selectionDisabled"
          @change="emit('toggle-select', session.id)"
        />
        <span>Batch</span>
      </label>

      <div class="session-card__status">
        <span class="session-card__status-dot"></span>
        <span>{{ stateLabel }}</span>
      </div>
    </div>

    <button
      class="session-card__surface"
      type="button"
      :aria-pressed="isActive"
      @click="emit('open-session', session.id)"
    >
      <div class="session-card__title-row">
        <h3 class="session-card__title">{{ title }}</h3>
        <span class="session-card__updated">{{ updatedAtLabel }}</span>
      </div>

      <p class="session-card__summary">{{ summary }}</p>
      <p v-if="previewLine" class="session-card__preview">{{ previewLine }}</p>

      <div class="session-card__footer">
        <span class="session-card__path">{{ condensedPath }}</span>
        <div class="session-card__metrics">
          <span>{{ formatCount(session.historyCount) }} history</span>
          <span>{{ formatCount(session.tokensUsed) }} tok</span>
        </div>
      </div>
    </button>
  </article>
</template>

<style scoped>
.session-card {
  display: grid;
  gap: 0.85rem;
  padding: 0.95rem;
  border-radius: 1.2rem;
  border: 1px solid rgba(112, 193, 174, 0.14);
  background: rgba(255, 255, 255, 0.04);
  transition:
    transform 150ms ease,
    border-color 150ms ease,
    box-shadow 150ms ease,
    background 150ms ease;
}

.session-card:hover {
  transform: translateY(-1px);
  border-color: rgba(55, 193, 218, 0.26);
  box-shadow: 0 16px 30px rgba(1, 8, 6, 0.2);
}

.session-card[data-active="true"] {
  border-color: rgba(255, 201, 71, 0.42);
  background: linear-gradient(180deg, rgba(37, 47, 18, 0.46), rgba(17, 34, 28, 0.72));
  box-shadow: 0 18px 36px rgba(255, 201, 71, 0.08);
}

.session-card[data-selected="true"] {
  background: linear-gradient(180deg, rgba(13, 41, 36, 0.84), rgba(7, 25, 21, 0.94));
}

.session-card__top,
.session-card__title-row,
.session-card__footer {
  display: flex;
  justify-content: space-between;
  gap: 0.75rem;
  align-items: center;
}

.session-card__checkbox {
  display: inline-flex;
  gap: 0.55rem;
  align-items: center;
  color: var(--text-muted);
  font-weight: 700;
}

.session-card__status {
  display: inline-flex;
  gap: 0.45rem;
  align-items: center;
  color: var(--text-soft);
  font-size: 0.82rem;
}

.session-card__status-dot {
  width: 0.55rem;
  height: 0.55rem;
  border-radius: 999px;
  background: var(--accent);
  box-shadow: 0 0 0 0.22rem rgba(255, 201, 71, 0.12);
}

.session-card__surface {
  display: grid;
  gap: 0.75rem;
  width: 100%;
  padding: 0;
  border: none;
  background: transparent;
  text-align: left;
  color: inherit;
}

.session-card__title {
  margin: 0;
  color: var(--heading);
  font-size: 1rem;
  line-height: 1.45;
}

.session-card__updated,
.session-card__path {
  color: var(--text-muted);
  font-size: 0.8rem;
  white-space: nowrap;
}

.session-card__summary,
.session-card__preview {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.65;
  display: -webkit-box;
  overflow: hidden;
  -webkit-box-orient: vertical;
}

.session-card__summary {
  -webkit-line-clamp: 2;
}

.session-card__preview {
  padding: 0.75rem 0.8rem;
  border-radius: 0.95rem;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(112, 193, 174, 0.1);
  -webkit-line-clamp: 2;
}

.session-card__footer {
  align-items: flex-end;
}

.session-card__path {
  max-width: 52%;
  overflow: hidden;
  text-overflow: ellipsis;
}

.session-card__metrics {
  display: flex;
  flex-wrap: wrap;
  gap: 0.55rem;
  justify-content: flex-end;
  color: var(--heading);
  font-size: 0.8rem;
  font-weight: 700;
}

@media (max-width: 720px) {
  .session-card__title-row,
  .session-card__footer {
    flex-direction: column;
    align-items: flex-start;
  }

  .session-card__path {
    max-width: 100%;
  }

  .session-card__metrics {
    justify-content: flex-start;
  }
}
</style>

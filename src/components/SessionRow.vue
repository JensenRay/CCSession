<script setup lang="ts">
import { computed } from "vue";
import type { SessionListItem } from "../types";
import {
  compactPath,
  formatCount,
  getSessionSummary,
} from "../utils/sessionFormat";

const props = defineProps<{
  session: SessionListItem;
  selected?: boolean;
  selectionDisabled?: boolean;
}>();

const emit = defineEmits<{
  (event: "toggle-select", sessionId: string): void;
  (event: "open-session", sessionId: string): void;
}>();

const summary = computed(() => getSessionSummary(props.session));
const condensedPath = computed(() => compactPath(props.session.cwd));
</script>

<template>
  <article class="session-card" :data-selected="selected">
    <label class="session-card__checkbox">
      <input type="checkbox" :checked="selected" :disabled="selectionDisabled"
        @change="emit('toggle-select', session.id)" />
      <span>Batch</span>
    </label>

    <button class="session-card__surface" type="button" :aria-label="`Open session ${summary}`"
      @click="emit('open-session', session.id)">
      <p class="session-card__summary">{{ summary }}</p>

      <div class="session-card__footer">
        <span class="session-card__path">{{ condensedPath }}</span>
        <div class="session-card__metrics">
          <span>{{ formatCount(session.historyCount, true) }} history</span>
          <span>{{ formatCount(session.tokensUsed, true) }} tok</span>
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

.session-card[data-selected="true"] {
  background: linear-gradient(180deg, rgba(13, 41, 36, 0.84), rgba(7, 25, 21, 0.94));
}

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

.session-card__path {
  color: var(--text-muted);
  font-size: 0.8rem;
  white-space: nowrap;
}

.session-card__summary {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.65;
  line-clamp: 2;
  display: -webkit-box;
  overflow: hidden;
  -webkit-box-orient: vertical;
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

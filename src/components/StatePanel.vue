<script setup lang="ts">
import { useSlots } from "vue";

withDefaults(
  defineProps<{
    eyebrow: string;
    title: string;
    tone?: "default" | "error";
  }>(),
  {
    tone: "default",
  },
);

const slots = useSlots();
</script>

<template>
  <section class="state-panel" :data-tone="tone">
    <p class="state-panel__eyebrow">{{ eyebrow }}</p>
    <h2 class="state-panel__title">{{ title }}</h2>
    <div class="state-panel__message">
      <slot />
    </div>
    <div v-if="slots.actions" class="state-panel__actions">
      <slot name="actions" />
    </div>
  </section>
</template>

<style scoped>
.state-panel {
  display: grid;
  gap: 0.8rem;
  padding: 1.25rem;
  border-radius: 1.55rem;
  border: 1px solid var(--border);
  background:
    linear-gradient(180deg, rgba(13, 31, 24, 0.96), rgba(8, 18, 15, 0.98)),
    var(--panel);
  box-shadow: 0 26px 70px rgba(1, 8, 6, 0.34);
}

.state-panel[data-tone="error"] {
  border-color: rgba(255, 123, 92, 0.28);
  background:
    linear-gradient(180deg, rgba(46, 18, 14, 0.95), rgba(24, 10, 8, 0.96)),
    var(--panel);
}

.state-panel__eyebrow {
  margin: 0;
  color: var(--accent-soft);
  font-size: 0.78rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
}

.state-panel__title {
  margin: 0;
  color: var(--heading);
  font-size: clamp(1.5rem, 3vw, 2.1rem);
}

.state-panel__message {
  color: var(--text-soft);
  line-height: 1.75;
}

.state-panel__message :deep(code) {
  padding: 0.1rem 0.42rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.08);
  color: var(--accent);
}

.state-panel__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

@media (max-width: 720px) {
  .state-panel {
    border-radius: 1.2rem;
  }

  .state-panel__actions :deep(.ui-button) {
    width: 100%;
  }
}
</style>

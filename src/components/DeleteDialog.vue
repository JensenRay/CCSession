<script setup lang="ts">
type DeleteDialogItem = {
  id: string;
  title: string;
  summary: string;
};

defineProps<{
  open: boolean;
  title: string;
  description: string;
  items: DeleteDialogItem[];
  confirmLabel: string;
  busy: boolean;
}>();

const emit = defineEmits<{
  (event: "close"): void;
  (event: "confirm"): void;
}>();
</script>

<template>
  <div v-if="open" class="dialog-backdrop" @click.self="emit('close')">
    <section class="dialog-panel" role="dialog" aria-modal="true" :aria-label="title">
      <header class="dialog-panel__header">
        <div>
          <p class="dialog-panel__eyebrow">Delete Confirmation</p>
          <h2 class="dialog-panel__title">{{ title }}</h2>
        </div>
        <button class="dialog-panel__close" type="button" :disabled="busy" @click="emit('close')">
          Close
        </button>
      </header>

      <p class="dialog-panel__description">{{ description }}</p>

      <section class="dialog-panel__warning">
        <p class="dialog-panel__warning-title">Before deleting</p>
        <ul class="dialog-panel__warning-list">
          <li>请先退出 Codex，避免 SQLite 和本地文件仍在被写入。</li>
          <li>前端只会调用 `delete_sessions`，不会在本地额外推断删除行为。</li>
        </ul>
      </section>

      <ul class="dialog-panel__items">
        <li v-for="item in items" :key="item.id" class="dialog-panel__item">
          <h3>{{ item.title }}</h3>
          <p>{{ item.summary }}</p>
          <code>{{ item.id }}</code>
        </li>
      </ul>

      <footer class="dialog-panel__footer">
        <button class="dialog-panel__secondary" type="button" :disabled="busy" @click="emit('close')">
          Cancel
        </button>
        <button class="dialog-panel__primary" type="button" :disabled="busy" @click="emit('confirm')">
          {{ busy ? "Deleting..." : confirmLabel }}
        </button>
      </footer>
    </section>
  </div>
</template>

<style scoped>
.dialog-backdrop {
  position: fixed;
  inset: 0;
  display: grid;
  place-items: center;
  padding: 1rem;
  background: rgba(53, 35, 22, 0.36);
  backdrop-filter: blur(10px);
  z-index: 20;
}

.dialog-panel {
  width: min(640px, 100%);
  display: grid;
  gap: 1rem;
  padding: 1.35rem;
  border-radius: 1.3rem;
  border: 1px solid rgba(105, 74, 48, 0.12);
  background: rgba(255, 251, 245, 0.98);
  box-shadow: 0 28px 80px rgba(43, 28, 18, 0.24);
}

.dialog-panel__header,
.dialog-panel__footer {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: center;
}

.dialog-panel__eyebrow {
  margin: 0 0 0.35rem;
  color: var(--accent);
  font-size: 0.76rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.dialog-panel__title {
  margin: 0;
  color: var(--heading);
}

.dialog-panel__close,
.dialog-panel__secondary,
.dialog-panel__primary {
  min-height: 2.75rem;
  padding: 0 1rem;
  border-radius: 999px;
  font-weight: 700;
  border: none;
}

.dialog-panel__close,
.dialog-panel__secondary {
  background: rgba(105, 74, 48, 0.08);
  color: var(--heading);
}

.dialog-panel__primary {
  background: linear-gradient(135deg, var(--danger), #a3472f);
  color: #fff;
}

.dialog-panel__description {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.7;
}

.dialog-panel__warning {
  padding: 0.95rem 1rem;
  border-radius: 1rem;
  background: rgba(255, 244, 226, 0.88);
  border: 1px solid rgba(159, 112, 0, 0.18);
}

.dialog-panel__warning-title {
  margin: 0 0 0.65rem;
  color: #805300;
  font-weight: 700;
}

.dialog-panel__warning-list {
  margin: 0;
  padding-left: 1.1rem;
  color: var(--text-soft);
  line-height: 1.6;
}

.dialog-panel__items {
  display: grid;
  gap: 0.75rem;
  margin: 0;
  padding: 0;
  list-style: none;
}

.dialog-panel__item {
  padding: 0.9rem 1rem;
  border-radius: 1rem;
  background: rgba(255, 255, 255, 0.74);
  border: 1px solid rgba(105, 74, 48, 0.1);
}

.dialog-panel__item h3,
.dialog-panel__item p,
.dialog-panel__item code {
  margin: 0;
}

.dialog-panel__item h3 {
  color: var(--heading);
  font-size: 1rem;
}

.dialog-panel__item p {
  margin-top: 0.45rem;
  color: var(--text-soft);
  line-height: 1.6;
}

.dialog-panel__item code {
  display: inline-block;
  margin-top: 0.65rem;
  padding: 0.18rem 0.45rem;
  border-radius: 999px;
  background: rgba(105, 74, 48, 0.08);
  color: var(--text-muted);
}

@media (max-width: 640px) {
  .dialog-panel {
    padding: 1rem;
    border-radius: 1rem;
  }

  .dialog-panel__header,
  .dialog-panel__footer {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>

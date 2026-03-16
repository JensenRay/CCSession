<script setup lang="ts">
import { onMounted } from "vue";
import { RouterView } from "vue-router";
import DeleteDialog from "./components/DeleteDialog.vue";
import { useSessionStore } from "./composables/useSessionStore";

const store = useSessionStore();

onMounted(() => {
  void store.ensureSessionsLoaded();
});
</script>

<template>
  <main class="app-shell">
    <section v-if="store.actionNotice" class="notice-panel" :class="`notice-panel--${store.actionNotice.tone}`">
      <h2 class="notice-panel__title">{{ store.actionNotice.title }}</h2>
      <p class="notice-panel__message">{{ store.actionNotice.message }}</p>
      <ul v-if="store.actionNotice.details.length" class="notice-panel__list">
        <li v-for="detail in store.actionNotice.details" :key="detail">
          {{ detail }}
        </li>
      </ul>
      <ul v-if="store.summaryReports.length" class="notice-panel__report-list">
        <li v-for="report in store.summaryReports" :key="report.sessionId" class="notice-panel__report-item">
          <strong>{{ report.sessionId }}</strong>
          <span>{{ report.status }}</span>
          <p v-if="report.error">{{ report.error }}</p>
          <p v-else-if="report.warnings.length">
            {{ report.warnings.join(" / ") }}
          </p>
        </li>
      </ul>
    </section>

    <section v-if="store.warnings.length" class="notice-panel notice-panel--warning">
      <h2 class="notice-panel__title">Warnings</h2>
      <ul class="notice-panel__list">
        <li v-for="warning in store.warnings" :key="warning">{{ warning }}</li>
      </ul>
    </section>

    <RouterView />

    <DeleteDialog :open="Boolean(store.pendingDeleteIds.length)" :title="store.deleteDialogTitle"
      :description="store.deleteDialogDescription" :items="store.deleteDialogItems"
      :confirm-label="store.deleteDialogConfirmLabel" :busy="store.isDeleting" @close="store.closeDeleteDialog"
      @confirm="store.confirmDelete" />
  </main>
</template>

<style scoped>
.app-shell {
  width: min(1400px, calc(100vw - 2rem));
  margin: 0 auto;
  padding: 1.4rem 0 2.8rem;
  display: grid;
  gap: 1rem;
}

.notice-panel {
  padding: 1.2rem;
  border: 1px solid var(--border);
  border-radius: 1.6rem;
  background:
    linear-gradient(180deg, rgba(15, 35, 28, 0.94), rgba(10, 24, 19, 0.96)),
    var(--panel);
  box-shadow: 0 28px 70px rgba(1, 8, 6, 0.34);
}

.notice-panel--warning {
  border-color: rgba(255, 201, 71, 0.22);
  background:
    linear-gradient(180deg, rgba(49, 41, 8, 0.95), rgba(22, 18, 6, 0.96)),
    var(--panel);
}

.notice-panel--success {
  border-color: rgba(112, 193, 174, 0.24);
}

.notice-panel--error {
  border-color: rgba(255, 123, 92, 0.28);
  background:
    linear-gradient(180deg, rgba(46, 18, 14, 0.95), rgba(24, 10, 8, 0.96)),
    var(--panel);
}

.notice-panel__title {
  margin: 0;
  color: var(--heading);
}

.notice-panel__message {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.72;
}

.notice-panel__list {
  margin: 0.85rem 0 0;
  padding-left: 1.1rem;
  color: var(--text-soft);
}

.notice-panel__report-list {
  display: grid;
  gap: 0.75rem;
  margin: 1rem 0 0;
  padding: 0;
  list-style: none;
}

.notice-panel__report-item {
  padding: 0.85rem 0.95rem;
  border-radius: 0.95rem;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(112, 193, 174, 0.12);
}

.notice-panel__report-item strong,
.notice-panel__report-item span,
.notice-panel__report-item p {
  display: block;
}

.notice-panel__report-item strong {
  color: var(--heading);
}

.notice-panel__report-item span {
  margin-top: 0.3rem;
  color: var(--accent);
  text-transform: capitalize;
}

.notice-panel__report-item p {
  margin: 0.45rem 0 0;
  color: var(--text-soft);
}

@media (max-width: 720px) {
  .app-shell {
    width: min(100vw - 1rem, 100%);
    padding-top: 0.6rem;
  }

  .notice-panel {
    border-radius: 1.2rem;
  }
}
</style>

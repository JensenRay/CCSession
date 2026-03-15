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
    <header class="app-header">
      <section class="app-header__brand">
        <p class="app-header__eyebrow">Local Codex Session Cleaner</p>
        <div class="app-header__title-row">
          <div class="app-header__mark" aria-hidden="true">
            <span></span>
            <span></span>
          </div>
          <div>
            <h1 class="app-header__title">CCSession</h1>
            <p class="app-header__description">
              Start with a compact session list. Open one card only when you
              want to move into the dedicated detail page.
            </p>
          </div>
        </div>
        <p class="app-header__meta">
          {{
            store.sessionsData?.codexRoot
              ? `Source: ${store.sessionsData.codexRoot}`
              : "Waiting for list_sessions to scan ~/.codex."
          }}
        </p>
      </section>

      <section class="app-header__stats">
        <article class="app-header__stat-card">
          <span class="app-header__stat-label">Sessions</span>
          <strong class="app-header__stat-value">{{ store.total }}</strong>
        </article>
        <article class="app-header__stat-card">
          <span class="app-header__stat-label">Queued</span>
          <strong class="app-header__stat-value">{{ store.selectedCount }}</strong>
        </article>
        <article class="app-header__stat-card">
          <span class="app-header__stat-label">Scanned At</span>
          <strong class="app-header__stat-value app-header__stat-value--small">
            {{ store.scannedAtLabel }}
          </strong>
        </article>
      </section>

      <section class="app-header__actions">
        <div class="app-header__focus-card">
          <p class="app-header__focus-label">Batch Delete Queue</p>
          <strong class="app-header__focus-title">
            {{ store.selectedSummaryLabel }}
          </strong>
          <p class="app-header__focus-meta">
            Select from the list page, then delete in one batch when you are
            ready.
          </p>
        </div>

        <div class="app-header__button-row">
          <button
            class="app-button app-button--danger"
            type="button"
            :disabled="!store.canBulkDelete"
            @click="store.openBatchDeleteDialog"
          >
            Delete Selected
          </button>
          <button
            class="app-button"
            type="button"
            :disabled="store.loading"
            @click="store.refreshSessions"
          >
            {{ store.loading ? "Refreshing..." : "Refresh" }}
          </button>
        </div>
      </section>
    </header>

    <section
      v-if="store.actionNotice"
      class="notice-panel"
      :class="`notice-panel--${store.actionNotice.tone}`"
    >
      <h2 class="notice-panel__title">{{ store.actionNotice.title }}</h2>
      <p class="notice-panel__message">{{ store.actionNotice.message }}</p>
      <ul v-if="store.actionNotice.details.length" class="notice-panel__list">
        <li v-for="detail in store.actionNotice.details" :key="detail">
          {{ detail }}
        </li>
      </ul>
      <ul v-if="store.summaryReports.length" class="notice-panel__report-list">
        <li
          v-for="report in store.summaryReports"
          :key="report.sessionId"
          class="notice-panel__report-item"
        >
          <strong>{{ report.sessionId }}</strong>
          <span>{{ report.status }}</span>
          <p v-if="report.error">{{ report.error }}</p>
          <p v-else-if="report.warnings.length">
            {{ report.warnings.join(" / ") }}
          </p>
        </li>
      </ul>
    </section>

    <section
      v-if="store.warnings.length"
      class="notice-panel notice-panel--warning"
    >
      <h2 class="notice-panel__title">Warnings</h2>
      <ul class="notice-panel__list">
        <li v-for="warning in store.warnings" :key="warning">{{ warning }}</li>
      </ul>
    </section>

    <RouterView />

    <DeleteDialog
      :open="Boolean(store.pendingDeleteIds.length)"
      :title="store.deleteDialogTitle"
      :description="store.deleteDialogDescription"
      :items="store.deleteDialogItems"
      :confirm-label="store.deleteDialogConfirmLabel"
      :busy="store.isDeleting"
      @close="store.closeDeleteDialog"
      @confirm="store.confirmDelete"
    />
  </main>
</template>

<style scoped>
.app-shell {
  width: min(1400px, calc(100vw - 2rem));
  margin: 0 auto;
  padding: 1.4rem 0 2.8rem;
}

.app-header {
  display: grid;
  grid-template-columns: minmax(0, 1.5fr) minmax(220px, 0.7fr) minmax(280px, 0.95fr);
  gap: 1rem;
  margin-bottom: 1rem;
}

.app-header__brand,
.app-header__stats,
.app-header__actions,
.notice-panel {
  border: 1px solid var(--border);
  border-radius: 1.6rem;
  background:
    linear-gradient(180deg, rgba(15, 35, 28, 0.94), rgba(10, 24, 19, 0.96)),
    var(--panel);
  box-shadow: 0 28px 70px rgba(1, 8, 6, 0.34);
}

.app-header__brand,
.app-header__actions {
  padding: 1.35rem;
}

.app-header__brand {
  display: grid;
  gap: 1rem;
}

.app-header__eyebrow {
  margin: 0;
  color: var(--accent-soft);
  font-size: 0.78rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
}

.app-header__title-row {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr);
  gap: 1rem;
  align-items: center;
}

.app-header__mark {
  position: relative;
  width: 4.25rem;
  aspect-ratio: 1;
  border-radius: 1.3rem;
  background: linear-gradient(145deg, rgba(12, 44, 36, 0.95), rgba(58, 58, 16, 0.9));
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.08);
}

.app-header__mark span {
  position: absolute;
  inset: 0;
  margin: auto;
  width: 2.35rem;
  height: 2.35rem;
  border-radius: 999px;
  border: 0.78rem solid transparent;
}

.app-header__mark span:first-child {
  top: -0.62rem;
  left: 0.68rem;
  border-color: var(--accent) var(--accent) transparent transparent;
  transform: rotate(46deg);
}

.app-header__mark span:last-child {
  bottom: -0.62rem;
  right: 0.68rem;
  border-color: transparent transparent var(--accent-soft) var(--accent-soft);
  transform: rotate(46deg);
}

.app-header__title {
  margin: 0;
  color: var(--heading);
  font-size: clamp(2rem, 4vw, 3.6rem);
  line-height: 0.95;
}

.app-header__description,
.app-header__meta,
.notice-panel__message {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.72;
}

.app-header__meta {
  padding: 0.85rem 1rem;
  border-radius: 1rem;
  background: rgba(112, 193, 174, 0.08);
  border: 1px solid rgba(112, 193, 174, 0.14);
}

.app-header__stats {
  display: grid;
  gap: 0.75rem;
  padding: 1rem;
}

.app-header__stat-card {
  display: grid;
  gap: 0.45rem;
  padding: 1rem;
  border-radius: 1.15rem;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(112, 193, 174, 0.12);
}

.app-header__stat-label {
  color: var(--text-muted);
  font-size: 0.78rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.app-header__stat-value {
  color: var(--heading);
  font-size: 2rem;
  line-height: 1.05;
}

.app-header__stat-value--small {
  font-size: 1rem;
}

.app-header__actions {
  display: grid;
  gap: 1rem;
}

.app-header__focus-card {
  display: grid;
  gap: 0.35rem;
  padding: 1rem;
  border-radius: 1rem;
  background: linear-gradient(135deg, rgba(255, 201, 71, 0.12), rgba(55, 193, 218, 0.08));
  border: 1px solid rgba(255, 201, 71, 0.18);
}

.app-header__focus-label {
  margin: 0;
  color: var(--text-muted);
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.app-header__focus-title {
  color: var(--heading);
  font-size: 1.05rem;
  line-height: 1.5;
}

.app-header__focus-meta {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.6;
}

.app-header__button-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.app-button {
  min-height: 2.9rem;
  padding: 0 1.1rem;
  border: none;
  border-radius: 999px;
  background: linear-gradient(135deg, var(--accent-soft), #0b93ab);
  color: #05110d;
  font-weight: 800;
  transition:
    transform 150ms ease,
    box-shadow 150ms ease,
    opacity 150ms ease;
}

.app-button:hover {
  transform: translateY(-1px);
  box-shadow: 0 16px 32px rgba(55, 193, 218, 0.2);
}

.app-button:disabled {
  opacity: 0.45;
  cursor: wait;
  transform: none;
  box-shadow: none;
}

.app-button--danger {
  background: linear-gradient(135deg, var(--accent), #ffb114);
}

.notice-panel {
  margin-bottom: 1rem;
  padding: 1.2rem;
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

@media (max-width: 1180px) {
  .app-header {
    grid-template-columns: minmax(0, 1fr);
  }
}

@media (max-width: 720px) {
  .app-shell {
    width: min(100vw - 1rem, 100%);
    padding-top: 0.6rem;
  }

  .app-header__brand,
  .app-header__stats,
  .app-header__actions,
  .notice-panel {
    border-radius: 1.2rem;
  }

  .app-header__title-row {
    grid-template-columns: minmax(0, 1fr);
  }

  .app-header__button-row {
    flex-direction: column;
  }

  .app-button {
    width: 100%;
  }
}
</style>

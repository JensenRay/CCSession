<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import DeleteDialog from "./components/DeleteDialog.vue";
import SessionList from "./components/SessionList.vue";
import {
  SessionCommandError,
  deleteSessions,
  listSessions,
} from "./api/sessions";
import type {
  DeleteSessionsData,
  ListSessionsData,
  SessionDeleteReport,
  SessionListItem,
} from "./types";

type NoticeTone = "success" | "warning" | "error";

type ActionNotice = {
  tone: NoticeTone;
  title: string;
  message: string;
  details: string[];
  reports: SessionDeleteReport[];
};

const sessionsData = ref<ListSessionsData | null>(null);
const loading = ref(false);
const errorMessage = ref("");
const activeDeleteIds = ref<string[]>([]);
const pendingDeleteIds = ref<string[]>([]);
const selectedIds = ref<string[]>([]);
const actionNotice = ref<ActionNotice | null>(null);

const sessions = computed(() => sessionsData.value?.sessions ?? []);
const total = computed(() => sessionsData.value?.total ?? 0);
const warnings = computed(() => sessionsData.value?.warnings ?? []);
const selectedCount = computed(() => selectedIds.value.length);
const allSelected = computed(
  () =>
    sessions.value.length > 0 &&
    selectedIds.value.length === sessions.value.length,
);
const pendingDeleteSessions = computed<SessionListItem[]>(() =>
  pendingDeleteIds.value
    .map((sessionId) =>
      sessions.value.find((session) => session.id === sessionId),
    )
    .filter((session): session is SessionListItem => Boolean(session)),
);
const deleteDialogItems = computed(() => {
  return pendingDeleteSessions.value.map((session) => ({
    id: session.id,
    title: session.title || session.id,
    summary: session.summary || "No summary available.",
  }));
});
const deleteDialogTitle = computed(() =>
  pendingDeleteSessions.value.length > 1
    ? `Delete ${pendingDeleteSessions.value.length} sessions?`
    : "Delete this session?",
);
const deleteDialogDescription = computed(() =>
  pendingDeleteSessions.value.length > 1
    ? "This batch delete request follows the backend contract and will ask the backend to remove each selected session plus its related history, logs, rollout file, and shell snapshot."
    : "This operation follows the backend deletion contract and may remove thread state, history rows, logs, rollout files, and shell snapshots for the selected session.",
);
const deleteDialogConfirmLabel = computed(() =>
  pendingDeleteSessions.value.length > 1 ? "Delete Selected" : "Delete Session",
);
const canBulkDelete = computed(
  () =>
    selectedIds.value.length > 0 &&
    !loading.value &&
    activeDeleteIds.value.length === 0,
);
const isDeleting = computed(() => activeDeleteIds.value.length > 0);
const summaryReports = computed(
  () =>
    actionNotice.value?.reports.filter(
      (report) =>
        report.status !== "deleted" ||
        report.warnings.length > 0 ||
        Boolean(report.error),
    ) ?? [],
);
const selectedSummaryLabel = computed(() => {
  if (!selectedIds.value.length) {
    return "No sessions selected";
  }

  return `${selectedIds.value.length} selected`;
});
const scannedAtLabel = computed(() => {
  if (!sessionsData.value?.scannedAt) {
    return "Not scanned yet";
  }

  return new Intl.DateTimeFormat("zh-CN", {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(sessionsData.value.scannedAt * 1000);
});

async function refreshSessions(): Promise<void> {
  loading.value = true;
  errorMessage.value = "";

  try {
    const nextData = await listSessions();
    const sessionIds = new Set(nextData.sessions.map((session) => session.id));

    sessionsData.value = nextData;
    selectedIds.value = selectedIds.value.filter((sessionId) =>
      sessionIds.has(sessionId),
    );
  } catch (error) {
    const commandError =
      error instanceof SessionCommandError
        ? error
        : new SessionCommandError(
            "command_rejected",
            "The session list could not be loaded.",
          );

    errorMessage.value = [
      commandError.message,
      ...commandError.details,
    ].join("\n");
  } finally {
    loading.value = false;
  }
}

function toggleSessionSelection(sessionId: string): void {
  const nextSelection = new Set(selectedIds.value);

  if (nextSelection.has(sessionId)) {
    nextSelection.delete(sessionId);
  } else {
    nextSelection.add(sessionId);
  }

  selectedIds.value = sessions.value
    .map((session) => session.id)
    .filter((id) => nextSelection.has(id));
}

function toggleAllSelection(): void {
  selectedIds.value = allSelected.value
    ? []
    : sessions.value.map((session) => session.id);
}

function openDeleteDialog(sessionIds: string[]): void {
  if (!sessionIds.length) {
    return;
  }

  pendingDeleteIds.value = sessionIds;
}

function openSingleDeleteDialog(sessionId: string): void {
  openDeleteDialog([sessionId]);
}

function openBatchDeleteDialog(): void {
  openDeleteDialog(selectedIds.value);
}

function closeDeleteDialog(): void {
  if (activeDeleteIds.value.length) {
    return;
  }

  pendingDeleteIds.value = [];
}

function createDeleteNotice(result: DeleteSessionsData): ActionNotice {
  const tone: NoticeTone =
    result.failedCount > 0
      ? "error"
      : result.partialFailureCount > 0 || result.notFoundCount > 0
        ? "warning"
        : "success";
  const title =
    result.requestedCount > 1
      ? "Batch delete finished"
      : tone === "success"
        ? "Session deleted"
        : tone === "warning"
          ? "Session deletion needs review"
          : "Session deletion failed";

  return {
    tone,
    title,
    message: [
      `Requested ${result.requestedCount}.`,
      `Deleted ${result.deletedCount}.`,
      `Partial ${result.partialFailureCount}.`,
      `Failed ${result.failedCount}.`,
      `Missing ${result.notFoundCount}.`,
    ].join(" "),
    details: result.warnings,
    reports: result.reports,
  };
}

async function confirmDelete(): Promise<void> {
  if (!pendingDeleteIds.value.length) {
    return;
  }

  activeDeleteIds.value = [...pendingDeleteIds.value];
  actionNotice.value = null;

  try {
    const result = await deleteSessions({
      sessionIds: pendingDeleteIds.value,
      requireCodexStopped: true,
    });

    actionNotice.value = createDeleteNotice(result);
    pendingDeleteIds.value = [];
    selectedIds.value = [];
    await refreshSessions();
  } catch (error) {
    const commandError =
      error instanceof SessionCommandError
        ? error
        : new SessionCommandError(
            "command_rejected",
            "The delete request was rejected.",
          );

    actionNotice.value = {
      tone: "error",
      title: "Delete request failed",
      message: commandError.message,
      details: commandError.details,
      reports: [],
    };
  } finally {
    activeDeleteIds.value = [];
  }
}

onMounted(() => {
  void refreshSessions();
});
</script>

<template>
  <main class="app-shell">
    <section class="hero">
      <div class="hero__content">
        <p class="hero__eyebrow">Local Codex Session Cleaner</p>
        <h1 class="hero__title">CCSession</h1>
        <p class="hero__description">
          Read the local session index from <code>~/.codex</code>, inspect the
          latest activity, and prepare for safe cleanup.
        </p>
      </div>

      <div class="hero__stats">
        <div class="hero__stat-card">
          <span class="hero__stat-label">Sessions</span>
          <strong class="hero__stat-value">{{ total }}</strong>
        </div>
        <div class="hero__stat-card">
          <span class="hero__stat-label">Scanned At</span>
          <strong class="hero__stat-value hero__stat-value--small">
            {{ scannedAtLabel }}
          </strong>
        </div>
      </div>
    </section>

    <section class="toolbar">
      <div class="toolbar__meta">
        <p class="toolbar__title">Session Index</p>
        <p class="toolbar__subtitle">
          {{
            sessionsData?.codexRoot
              ? `Source: ${sessionsData.codexRoot}`
              : "The frontend expects list_sessions to follow the implementation plan contract."
          }}
        </p>
      </div>

      <div class="toolbar__actions">
        <span class="toolbar__selection">{{ selectedSummaryLabel }}</span>
        <button
          class="toolbar__button toolbar__button--danger"
          type="button"
          :disabled="!canBulkDelete"
          @click="openBatchDeleteDialog"
        >
          Delete Selected
        </button>
        <button class="toolbar__button" type="button" :disabled="loading" @click="refreshSessions">
          {{ loading ? "Refreshing..." : "Refresh" }}
        </button>
      </div>
    </section>

    <section
      v-if="actionNotice"
      class="notice-panel"
      :class="`notice-panel--${actionNotice.tone}`"
    >
      <h2 class="notice-panel__title">{{ actionNotice.title }}</h2>
      <p class="notice-panel__message">{{ actionNotice.message }}</p>
      <ul v-if="actionNotice.details.length" class="notice-panel__list">
        <li v-for="detail in actionNotice.details" :key="detail">{{ detail }}</li>
      </ul>
      <ul v-if="summaryReports.length" class="notice-panel__report-list">
        <li v-for="report in summaryReports" :key="report.sessionId" class="notice-panel__report-item">
          <strong>{{ report.sessionId }}</strong>
          <span>{{ report.status }}</span>
          <p v-if="report.error">{{ report.error }}</p>
          <p v-else-if="report.warnings.length">{{ report.warnings.join(" / ") }}</p>
        </li>
      </ul>
    </section>

    <section v-if="warnings.length" class="notice-panel notice-panel--warning">
      <h2 class="notice-panel__title">Warnings</h2>
      <ul class="notice-panel__list">
        <li v-for="warning in warnings" :key="warning">{{ warning }}</li>
      </ul>
    </section>

    <section v-if="errorMessage" class="notice-panel notice-panel--error">
      <h2 class="notice-panel__title">Load Failed</h2>
      <p class="notice-panel__message">{{ errorMessage }}</p>
      <button class="notice-panel__action" type="button" @click="refreshSessions">
        Retry
      </button>
    </section>

    <section v-else-if="loading" class="state-panel">
      <p class="state-panel__eyebrow">Loading</p>
      <h2 class="state-panel__title">Reading the local Codex session index</h2>
      <p class="state-panel__message">
        Waiting for <code>list_sessions</code> to return session cards,
        previews, and counts.
      </p>
    </section>

    <section v-else-if="!sessions.length" class="state-panel">
      <p class="state-panel__eyebrow">Empty</p>
      <h2 class="state-panel__title">No sessions were returned</h2>
      <p class="state-panel__message">
        Check whether the backend already exposes <code>list_sessions</code> and
        whether the local Codex data can be scanned.
      </p>
    </section>

    <SessionList
      v-else
      :sessions="sessions"
      :selected-ids="selectedIds"
      :active-delete-ids="activeDeleteIds"
      :selection-disabled="isDeleting"
      :delete-disabled="isDeleting"
      @toggle-all="toggleAllSelection"
      @toggle-select="toggleSessionSelection"
      @request-delete="openSingleDeleteDialog"
    />

    <DeleteDialog
      :open="Boolean(pendingDeleteIds.length)"
      :title="deleteDialogTitle"
      :description="deleteDialogDescription"
      :items="deleteDialogItems"
      :confirm-label="deleteDialogConfirmLabel"
      :busy="isDeleting"
      @close="closeDeleteDialog"
      @confirm="confirmDelete"
    />
  </main>
</template>

<style scoped>
.app-shell {
  width: min(1180px, calc(100vw - 2rem));
  margin: 0 auto;
  padding: 2rem 0 3rem;
}

.hero {
  display: grid;
  grid-template-columns: minmax(0, 1.7fr) minmax(260px, 0.95fr);
  gap: 1.25rem;
  padding: 1.5rem;
  border: 1px solid var(--border);
  border-radius: 1.5rem;
  background:
    linear-gradient(135deg, rgba(255, 251, 245, 0.98), rgba(239, 223, 204, 0.88)),
    var(--panel);
  box-shadow: 0 18px 50px rgba(88, 63, 43, 0.1);
}

.hero__content {
  display: grid;
  gap: 0.8rem;
}

.hero__eyebrow {
  margin: 0;
  color: var(--accent);
  font-size: 0.78rem;
  font-weight: 700;
  letter-spacing: 0.18em;
  text-transform: uppercase;
}

.hero__title {
  margin: 0;
  color: var(--heading);
  font-size: clamp(2.3rem, 5vw, 4.1rem);
  line-height: 0.98;
}

.hero__description {
  max-width: 44rem;
  margin: 0;
  color: var(--text-soft);
  font-size: 1.02rem;
  line-height: 1.8;
}

.hero__description code {
  padding: 0.1rem 0.45rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.72);
  color: var(--accent-strong);
}

.hero__stats {
  display: grid;
  gap: 0.85rem;
  align-content: stretch;
}

.hero__stat-card {
  display: grid;
  gap: 0.55rem;
  align-content: space-between;
  padding: 1.1rem;
  border-radius: 1.15rem;
  background: rgba(255, 255, 255, 0.72);
  border: 1px solid rgba(105, 74, 48, 0.12);
}

.hero__stat-label {
  color: var(--text-muted);
  font-size: 0.82rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.hero__stat-value {
  color: var(--heading);
  font-size: 2rem;
  line-height: 1.1;
}

.hero__stat-value--small {
  font-size: 1.05rem;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: center;
  margin: 1.35rem 0 1rem;
  padding: 1rem 1.15rem;
  border-radius: 1.1rem;
  background: rgba(255, 255, 255, 0.56);
  border: 1px solid var(--border);
}

.toolbar__meta {
  display: grid;
  gap: 0.35rem;
}

.toolbar__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  justify-content: flex-end;
  align-items: center;
}

.toolbar__title {
  margin: 0;
  color: var(--heading);
  font-size: 1rem;
  font-weight: 700;
}

.toolbar__subtitle {
  margin: 0;
  color: var(--text-muted);
  line-height: 1.6;
}

.toolbar__selection {
  display: inline-flex;
  align-items: center;
  min-height: 2.6rem;
  padding: 0 0.9rem;
  border-radius: 999px;
  background: rgba(105, 74, 48, 0.08);
  color: var(--heading);
  font-weight: 700;
}

.toolbar__button,
.notice-panel__action {
  min-height: 2.9rem;
  padding: 0 1rem;
  border: none;
  border-radius: 999px;
  background: linear-gradient(135deg, var(--accent), var(--accent-strong));
  color: #fff;
  font-weight: 700;
  transition:
    transform 150ms ease,
    box-shadow 150ms ease,
    opacity 150ms ease;
}

.toolbar__button:hover,
.notice-panel__action:hover {
  transform: translateY(-1px);
  box-shadow: 0 14px 28px rgba(138, 66, 24, 0.18);
}

.toolbar__button:disabled {
  opacity: 0.66;
  cursor: wait;
  transform: none;
  box-shadow: none;
}

.toolbar__button--danger {
  background: linear-gradient(135deg, var(--danger), #a3472f);
}

.notice-panel,
.state-panel {
  margin-bottom: 1rem;
  padding: 1.2rem;
  border-radius: 1.1rem;
  border: 1px solid var(--border);
}

.notice-panel {
  background: rgba(255, 255, 255, 0.7);
}

.notice-panel--warning {
  border-color: rgba(159, 112, 0, 0.22);
  background: rgba(255, 247, 226, 0.9);
}

.notice-panel--success {
  border-color: rgba(49, 93, 60, 0.16);
  background: rgba(235, 248, 238, 0.88);
}

.notice-panel--error {
  border-color: rgba(139, 61, 39, 0.22);
  background: rgba(255, 239, 234, 0.9);
}

.notice-panel__title,
.state-panel__title {
  margin: 0;
  color: var(--heading);
}

.notice-panel__list {
  margin: 0.8rem 0 0;
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
  background: rgba(255, 255, 255, 0.62);
  border: 1px solid rgba(105, 74, 48, 0.1);
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
  margin-top: 0.35rem;
  color: var(--accent-strong);
  text-transform: capitalize;
}

.notice-panel__report-item p {
  margin: 0.45rem 0 0;
  color: var(--text-soft);
  line-height: 1.6;
}

.notice-panel__message,
.state-panel__message {
  margin: 0.75rem 0 0;
  color: var(--text-soft);
  line-height: 1.7;
  white-space: pre-line;
}

.notice-panel__action {
  margin-top: 1rem;
}

.state-panel {
  background: rgba(255, 255, 255, 0.64);
}

.state-panel__eyebrow {
  margin: 0 0 0.35rem;
  color: var(--accent);
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.state-panel__message code {
  padding: 0.1rem 0.35rem;
  border-radius: 0.45rem;
  background: rgba(255, 255, 255, 0.72);
}

@media (max-width: 900px) {
  .hero {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 640px) {
  .app-shell {
    width: min(100vw - 1rem, 100%);
    padding-top: 1rem;
  }

  .hero,
  .toolbar,
  .notice-panel,
  .state-panel {
    border-radius: 1rem;
  }

  .toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .toolbar__actions {
    justify-content: stretch;
  }
}
</style>

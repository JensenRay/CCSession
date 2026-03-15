import { computed, reactive, ref } from "vue";
import {
  SessionCommandError,
  deleteSessions,
  listSessions,
} from "../api/sessions";
import type {
  DeleteSessionsData,
  ListSessionsData,
  SessionDeleteReport,
  SessionListItem,
} from "../types";

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
const hasLoaded = ref(false);
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
const deleteDialogItems = computed(() =>
  pendingDeleteSessions.value.map((session) => ({
    id: session.id,
    title: session.title || session.id,
    summary: session.summary || "No summary available.",
  })),
);
const deleteDialogTitle = computed(() =>
  pendingDeleteSessions.value.length > 1
    ? `Delete ${pendingDeleteSessions.value.length} sessions?`
    : "Delete this session?",
);
const deleteDialogDescription = computed(() =>
  pendingDeleteSessions.value.length > 1
    ? "Each selected session will be removed from the thread index together with related history rows, structured logs, rollout files, and shell snapshots."
    : "This removes the selected session from the thread index and also asks the backend to clean its related history, logs, rollout file, and shell snapshot.",
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
    return "No sessions queued for batch delete";
  }

  return `${selectedIds.value.length} sessions queued`;
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

async function ensureSessionsLoaded(force = false): Promise<void> {
  if (force || (!hasLoaded.value && !loading.value)) {
    await refreshSessions();
  }
}

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

    errorMessage.value = [commandError.message, ...commandError.details].join(
      "\n",
    );
  } finally {
    hasLoaded.value = true;
    loading.value = false;
  }
}

function getSessionById(sessionId: string): SessionListItem | null {
  return sessions.value.find((session) => session.id === sessionId) ?? null;
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

const sessionStore = reactive({
  sessionsData,
  sessions,
  total,
  warnings,
  loading,
  errorMessage,
  selectedIds,
  selectedCount,
  activeDeleteIds,
  pendingDeleteIds,
  actionNotice,
  summaryReports,
  scannedAtLabel,
  selectedSummaryLabel,
  deleteDialogItems,
  deleteDialogTitle,
  deleteDialogDescription,
  deleteDialogConfirmLabel,
  canBulkDelete,
  isDeleting,
  ensureSessionsLoaded,
  refreshSessions,
  getSessionById,
  toggleSessionSelection,
  toggleAllSelection,
  openDeleteDialog,
  openBatchDeleteDialog,
  closeDeleteDialog,
  confirmDelete,
});

export function useSessionStore() {
  return sessionStore;
}

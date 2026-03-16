import { computed, ref } from "vue";
import {
  deleteSessions,
  toSessionCommandError,
} from "../../api/sessions";
import { getCompactSessionSummary } from "../../utils/sessionFormat";
import type {
  DeleteSessionsData,
  SessionDeleteReport,
  SessionListItem,
} from "../../types";
import type { SessionIndexState } from "./useSessionIndex";

type NoticeTone = "success" | "warning" | "error";

type ActionNotice = {
  tone: NoticeTone;
  title: string;
  message: string;
  details: string[];
  reports: SessionDeleteReport[];
};

type DeleteDialogItem = {
  id: string;
  summary: string;
};

export function createSessionDeleteState(indexState: SessionIndexState) {
  const activeDeleteIds = ref<string[]>([]);
  const pendingDeleteIds = ref<string[]>([]);
  const actionNotice = ref<ActionNotice | null>(null);

  const pendingDeleteSessions = computed<SessionListItem[]>(() =>
    pendingDeleteIds.value
      .map((sessionId) =>
        indexState.sessions.value.find((session) => session.id === sessionId),
      )
      .filter((session): session is SessionListItem => Boolean(session)),
  );

  const deleteDialogItems = computed<DeleteDialogItem[]>(() =>
    pendingDeleteSessions.value.map((session) => ({
      id: session.id,
      summary: getCompactSessionSummary(session, 140),
    })),
  );

  const deleteDialogTitle = computed(() =>
    pendingDeleteSessions.value.length > 1
      ? `Delete ${pendingDeleteSessions.value.length} sessions?`
      : "Delete this session?",
  );

  const deleteDialogDescription = computed(() =>
    pendingDeleteSessions.value.length > 1
      ? "Each selected session will be removed from the thread index together with related history rows, structured logs, and shell snapshots. The rollout JSONL file will be moved to Trash so it can still be recovered."
      : "This removes the selected session from the thread index and cleans its related history, logs, and shell snapshot. The rollout JSONL file will be moved to Trash so it can still be recovered.",
  );

  const deleteDialogConfirmLabel = computed(() =>
    pendingDeleteSessions.value.length > 1 ? "Delete Selected" : "Delete Session",
  );

  const canBulkDelete = computed(
    () =>
      indexState.selectedIds.value.length > 0 &&
      !indexState.loading.value &&
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

  function openDeleteDialog(sessionIds: string[]): void {
    const normalizedSessionIds = dedupeSessionIds(sessionIds);

    if (!normalizedSessionIds.length) {
      return;
    }

    pendingDeleteIds.value = normalizedSessionIds;
  }

  function openBatchDeleteDialog(): void {
    openDeleteDialog(indexState.selectedIds.value);
  }

  function closeDeleteDialog(): void {
    if (activeDeleteIds.value.length) {
      return;
    }

    pendingDeleteIds.value = [];
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
      indexState.clearSelection();
      await indexState.refreshSessions();
    } catch (error) {
      const commandError = toSessionCommandError(
        error,
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

  return {
    activeDeleteIds,
    pendingDeleteIds,
    actionNotice,
    summaryReports,
    deleteDialogItems,
    deleteDialogTitle,
    deleteDialogDescription,
    deleteDialogConfirmLabel,
    canBulkDelete,
    isDeleting,
    openDeleteDialog,
    openBatchDeleteDialog,
    closeDeleteDialog,
    confirmDelete,
  };
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
      ? tone === "success"
        ? "Batch delete finished"
        : tone === "warning"
          ? "Batch delete finished with warnings"
          : "Batch delete failed"
      : tone === "success"
        ? "Session deleted"
        : tone === "warning"
          ? "Session deleted with warnings"
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

function dedupeSessionIds(sessionIds: string[]): string[] {
  const seen = new Set<string>();
  const dedupedIds: string[] = [];

  for (const sessionId of sessionIds) {
    if (seen.has(sessionId)) {
      continue;
    }

    seen.add(sessionId);
    dedupedIds.push(sessionId);
  }

  return dedupedIds;
}

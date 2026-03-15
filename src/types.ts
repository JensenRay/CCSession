export type ApiErrorCode =
  | "codex_root_not_found"
  | "state_db_open_failed"
  | "state_db_query_failed"
  | "logs_db_open_failed"
  | "history_file_read_failed"
  | "history_file_write_failed"
  | "codex_running_detected"
  | "invalid_input"
  | "delete_execution_failed";

export type ApiError = {
  code: ApiErrorCode;
  message: string;
  details: string[];
};

export type ApiResponse<T> =
  | {
      ok: true;
      data: T;
    }
  | {
      ok: false;
      error: ApiError;
    };

export type ListSessionsRequest = {
  includeArchived: boolean;
};

export type SessionListItem = {
  id: string;
  title: string;
  summary: string;
  contentPreview: string[];
  cwd: string;
  createdAt: number;
  updatedAt: number;
  tokensUsed: number;
  archived: boolean;
  source: string;
  modelProvider: string;
  hasRollout: boolean;
  hasSnapshot: boolean;
  historyCount: number;
  structuredLogCount: number;
};

export type ListSessionsData = {
  sessions: SessionListItem[];
  total: number;
  scannedAt: number;
  codexRoot: string;
  warnings: string[];
};

export type DeleteSessionsRequest = {
  sessionIds: string[];
  requireCodexStopped: boolean;
};

export type SessionDeleteStatus =
  | "deleted"
  | "partial_failure"
  | "failed"
  | "not_found";

export type SessionDeleteReport = {
  sessionId: string;
  status: SessionDeleteStatus;
  deletedStateRow: boolean;
  deletedHistoryEntries: number;
  deletedStructuredLogRows: number;
  deletedRolloutFile: boolean;
  deletedSnapshotFile: boolean;
  warnings: string[];
  error?: string;
};

export type DeleteSessionsData = {
  requestedCount: number;
  deletedCount: number;
  partialFailureCount: number;
  failedCount: number;
  notFoundCount: number;
  reports: SessionDeleteReport[];
  warnings: string[];
};

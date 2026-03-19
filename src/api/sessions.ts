import { invoke } from "@tauri-apps/api/core";
import type {
  ApiErrorCode,
  ApiResponse,
  DeleteSessionsData,
  DeleteSessionsRequest,
  ListSessionsData,
  ListSessionsRequest,
  SessionPromptsData,
  SessionPromptsRequest,
} from "../types";

type BrowserMockSession = ListSessionsData["sessions"][number];

type BrowserMockState = {
  sessions: BrowserMockSession[];
  prompts: Record<string, SessionPromptsData>;
};

const browserMockState: BrowserMockState = {
  sessions: createMockSessions(),
  prompts: createMockPrompts(),
};

export type SessionCommandErrorCode = ApiErrorCode | "command_rejected";

export class SessionCommandError extends Error {
  readonly code: SessionCommandErrorCode;
  readonly details: string[];

  constructor(
    code: SessionCommandErrorCode,
    message: string,
    details: string[] = [],
  ) {
    super(message);
    this.name = "SessionCommandError";
    this.code = code;
    this.details = details;
  }
}

export function toSessionCommandError(
  error: unknown,
  fallbackMessage = "The Tauri command failed unexpectedly.",
): SessionCommandError {
  if (error instanceof SessionCommandError) {
    return error;
  }

  if (typeof error === "string") {
    return new SessionCommandError("command_rejected", error);
  }

  if (error instanceof Error) {
    return new SessionCommandError("command_rejected", error.message);
  }

  return new SessionCommandError("command_rejected", fallbackMessage);
}

async function invokeCommand<T>(
  command: string,
  input: Record<string, unknown>,
): Promise<T> {
  if (shouldUseBrowserMock()) {
    return invokeBrowserMock<T>(command, input);
  }

  try {
    const response = await invoke<ApiResponse<T>>(command, { input });

    if (response.ok) {
      return response.data;
    }

    throw new SessionCommandError(
      response.error.code,
      response.error.message,
      response.error.details,
    );
  } catch (error) {
    throw toSessionCommandError(error);
  }
}

function shouldUseBrowserMock(): boolean {
  return typeof window !== "undefined" && !("__TAURI_INTERNALS__" in window);
}

async function invokeBrowserMock<T>(
  command: string,
  input: Record<string, unknown>,
): Promise<T> {
  if (command === "list_sessions") {
    return {
      sessions: browserMockState.sessions.slice(),
      total: browserMockState.sessions.length,
      scannedAt: Math.floor(Date.now() / 1000),
      codexRoot: "~/.codex (browser mock)",
      warnings: [],
    } as T;
  }

  if (command === "session_prompts") {
    const sessionId = String(input.sessionId ?? "");
    const data = browserMockState.prompts[sessionId];

    if (!data) {
      return {
        sessionId,
        prompts: [],
        warnings: [],
      } as T;
    }

    return {
      sessionId: data.sessionId,
      prompts: [...data.prompts],
      warnings: [...data.warnings],
    } as T;
  }

  if (command === "delete_sessions") {
    const sessionIds = Array.isArray(input.sessionIds)
      ? [...new Set(input.sessionIds.map((sessionId) => String(sessionId)))]
      : [];

    const reports = sessionIds.map((sessionId) => {
      const index = browserMockState.sessions.findIndex(
        (session) => session.id === sessionId,
      );
      const existed = index >= 0;

      if (existed) {
        browserMockState.sessions.splice(index, 1);
        delete browserMockState.prompts[sessionId];
      }

      return {
        sessionId,
        status: existed ? "deleted" : "not_found",
        deletedStateRow: existed,
        deletedHistoryEntries: existed ? 12 : 0,
        deletedStructuredLogRows: existed ? 4 : 0,
        deletedRolloutFile: existed,
        deletedSnapshotFile: existed,
        warnings: [],
      };
    });

    return {
      requestedCount: sessionIds.length,
      deletedCount: reports.filter((report) => report.status === "deleted").length,
      partialFailureCount: 0,
      failedCount: 0,
      notFoundCount: reports.filter((report) => report.status === "not_found").length,
      reports,
      warnings: [],
    } as T;
  }

  throw new SessionCommandError(
    "command_rejected",
    `Unknown browser mock command: ${command}`,
  );
}

function createMockSessions(): BrowserMockSession[] {
  return Array.from({ length: 16 }, (_, index) => {
    const number = index + 1;
    const archived = number % 5 === 0;

    return {
      id: `session-${number.toString().padStart(2, "0")}`,
      summary:
        number % 3 === 0
          ? `Investigate a layout edge case for session ${number} and verify the scroll region behaves correctly.`
          : `Discussed Codex session ${number} and the next cleanup step.`,
      contentPreview: [
        `Preview prompt ${number}.1`,
        `Preview prompt ${number}.2`,
        `Preview prompt ${number}.3`,
      ],
      cwd: `/Users/Admin/Projects/Toys/CCSession/packages/app-${number}`,
      createdAt: 1710000000 + number * 1800,
      updatedAt: 1710009000 + number * 1800,
      tokensUsed: 4800 + number * 321,
      archived,
      source: "cli",
      modelProvider: "openai",
    };
  });
}

function createMockPrompts(): Record<string, SessionPromptsData> {
  return Object.fromEntries(
    createMockSessions().map((session) => [
      session.id,
      {
        sessionId: session.id,
        prompts: Array.from({ length: 6 }, (_, index) => {
          const promptNumber = index + 1;
          return `${session.id} prompt ${promptNumber}: verify the session detail panel and metadata layout.`;
        }),
        warnings: session.id === "session-06" ? ["One prompt line was malformed and skipped."] : [],
      },
    ]),
  );
}

export async function listSessions(
  input: Partial<ListSessionsRequest> = {},
): Promise<ListSessionsData> {
  const request: ListSessionsRequest = {
    ...input,
    includeArchived: input.includeArchived ?? true,
  };

  return invokeCommand<ListSessionsData>("list_sessions", {
    ...request,
  });
}

export async function deleteSessions(
  input: DeleteSessionsRequest,
): Promise<DeleteSessionsData> {
  const request: DeleteSessionsRequest = {
    ...input,
    requireCodexStopped: input.requireCodexStopped ?? true,
  };

  return invokeCommand<DeleteSessionsData>("delete_sessions", {
    ...request,
  });
}

export async function sessionPrompts(
  input: SessionPromptsRequest,
): Promise<SessionPromptsData> {
  return invokeCommand<SessionPromptsData>("session_prompts", {
    ...input,
  });
}

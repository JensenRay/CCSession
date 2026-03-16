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

function toSessionCommandError(error: unknown): SessionCommandError {
  if (error instanceof SessionCommandError) {
    return error;
  }

  if (typeof error === "string") {
    return new SessionCommandError("command_rejected", error);
  }

  if (error instanceof Error) {
    return new SessionCommandError("command_rejected", error.message);
  }

  return new SessionCommandError(
    "command_rejected",
    "The Tauri command failed unexpectedly.",
  );
}

async function invokeCommand<T>(
  command: string,
  input: Record<string, unknown>,
): Promise<T> {
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

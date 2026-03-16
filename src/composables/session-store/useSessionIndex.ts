import { computed, ref } from "vue";
import { listSessions, toSessionCommandError } from "../../api/sessions";
import type { ListSessionsData, SessionListItem } from "../../types";

export function createSessionIndexState() {
  const sessionsData = ref<ListSessionsData | null>(null);
  const loading = ref(false);
  const hasLoaded = ref(false);
  const errorMessage = ref("");
  const selectedIds = ref<string[]>([]);

  const sessions = computed(() => sessionsData.value?.sessions ?? []);
  const warnings = computed(() => sessionsData.value?.warnings ?? []);
  const allSelected = computed(
    () =>
      sessions.value.length > 0 &&
      selectedIds.value.length === sessions.value.length,
  );

  async function ensureSessionsLoaded(force = false): Promise<void> {
    if (!force && (hasLoaded.value || loading.value)) {
      return;
    }

    await refreshSessions();
  }

  async function refreshSessions(): Promise<void> {
    loading.value = true;
    errorMessage.value = "";

    try {
      const nextData = await listSessions();
      setSessionsData(nextData);
    } catch (error) {
      const commandError = toSessionCommandError(
        error,
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

  function setSessionsData(nextData: ListSessionsData): void {
    const sessionIds = new Set(nextData.sessions.map((session) => session.id));

    sessionsData.value = nextData;
    selectedIds.value = selectedIds.value.filter((sessionId) =>
      sessionIds.has(sessionId),
    );
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

  function clearSelection(): void {
    selectedIds.value = [];
  }

  return {
    sessions,
    warnings,
    loading,
    hasLoaded,
    errorMessage,
    selectedIds,
    ensureSessionsLoaded,
    refreshSessions,
    getSessionById,
    toggleSessionSelection,
    toggleAllSelection,
    clearSelection,
  };
}

export type SessionIndexState = ReturnType<typeof createSessionIndexState>;

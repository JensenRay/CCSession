<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { NFlex } from "naive-ui";
import SessionDetailHeader from "../components/session/SessionDetailHeader.vue";
import SessionDetailStateCard from "../components/session/SessionDetailStateCard.vue";
import SessionMetadataCard from "../components/session/SessionMetadataCard.vue";
import SessionPromptsCard from "../components/session/SessionPromptsCard.vue";
import { useSessionPrompts } from "../composables/useSessionPrompts";
import { useSessionStore } from "../composables/useSessionStore";

const route = useRoute();
const router = useRouter();
const store = useSessionStore();

const sessionId = computed(() => String(route.params.sessionId ?? ""));
const session = computed(() => store.getSessionById(sessionId.value));
const activeSession = computed(
  () => session.value as NonNullable<typeof session.value>,
);
const previewEntries = computed(() => session.value?.contentPreview ?? []);
const selected = computed(() => {
  const currentSession = session.value;
  return currentSession ? store.selectedIds.includes(currentSession.id) : false;
});
const isDeletingCurrent = computed(() => {
  const currentSession = session.value;
  return currentSession
    ? store.activeDeleteIds.includes(currentSession.id)
    : false;
});
const {
  promptEntries,
  promptEntriesLoading,
  promptEntriesError,
  promptEntriesWarnings,
} = useSessionPrompts(sessionId, previewEntries);

const hasError = computed(() => Boolean(store.errorMessage) && !session.value);
const isLoading = computed(() => store.loading && !session.value);
const isMissing = computed(
  () => !session.value && !hasError.value && !isLoading.value,
);

function goBack(): void {
  void router.push({ name: "session-list" });
}

function requestDelete(targetSessionId: string): void {
  store.openDeleteDialog([targetSessionId]);
}
</script>

<template>
  <SessionDetailStateCard
    v-if="hasError"
    state="error"
    :error-message="store.errorMessage"
    @retry="store.refreshSessions"
    @back="goBack"
  />

  <SessionDetailStateCard
    v-else-if="isLoading"
    state="loading"
    @retry="store.refreshSessions"
    @back="goBack"
  />

  <SessionDetailStateCard
    v-else-if="isMissing"
    state="missing"
    @retry="store.refreshSessions"
    @back="goBack"
  />

  <template v-else>
    <n-flex vertical style="flex: 1; min-height: 0; gap: 16px; overflow: hidden;">
      <SessionDetailHeader
        style="flex-shrink: 0;"
        :selected="selected"
        :is-deleting-current="isDeletingCurrent"
        :is-deleting="store.isDeleting"
        :loading="store.loading"
        @back="goBack"
        @toggle-selection="store.toggleSessionSelection(activeSession.id)"
        @delete="requestDelete(activeSession.id)"
        @refresh="store.refreshSessions"
      />

      <SessionMetadataCard style="flex-shrink: 0;" :session="activeSession" />

      <SessionPromptsCard
        style="flex: 1; min-height: 0;"
        :session-id="activeSession.id"
        :prompt-entries="promptEntries"
        :prompt-entries-loading="promptEntriesLoading"
        :prompt-entries-error="promptEntriesError"
        :prompt-entries-warnings="promptEntriesWarnings"
      />
    </n-flex>
  </template>
</template>

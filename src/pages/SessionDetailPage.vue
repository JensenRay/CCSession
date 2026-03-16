<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import SessionDetail from "../components/SessionDetail.vue";
import StatePanel from "../components/StatePanel.vue";
import { useSessionPrompts } from "../composables/useSessionPrompts";
import { useSessionStore } from "../composables/useSessionStore";

const route = useRoute();
const router = useRouter();
const store = useSessionStore();

const sessionId = computed(() => String(route.params.sessionId ?? ""));
const session = computed(() => store.getSessionById(sessionId.value));
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

function goBack(): void {
  void router.push({ name: "session-list" });
}

function requestDelete(targetSessionId: string): void {
  store.openDeleteDialog([targetSessionId]);
}
</script>

<template>
  <StatePanel v-if="store.errorMessage && !session" eyebrow="Load Failed" title="The session detail could not be loaded"
    tone="error">
    {{ store.errorMessage }}
    <template #actions>
      <button class="ui-button" type="button" @click="store.refreshSessions">
        Retry
      </button>
      <button class="ui-button ui-button--ghost" type="button" @click="goBack">
        Back to List
      </button>
    </template>
  </StatePanel>

  <StatePanel v-else-if="store.loading && !session" eyebrow="Loading" title="Preparing the session detail page">
    Waiting for the session index to finish loading before opening this record.
  </StatePanel>

  <StatePanel v-else-if="!session" eyebrow="Missing" title="This session is no longer available">
    It may have been deleted or the local index changed. Return to the list and
    pick another session.
    <template #actions>
      <button class="ui-button ui-button--ghost" type="button" @click="goBack">
        Back to List
      </button>
      <button class="ui-button ui-button--ghost" type="button" @click="store.refreshSessions">
        Refresh Index
      </button>
    </template>
  </StatePanel>

  <section v-else class="detail-page">
    <SessionDetail :session="session" :selected="selected" :selection-disabled="store.isDeleting"
      :loading="store.loading" :delete-disabled="store.isDeleting" :is-deleting="isDeletingCurrent"
      @go-back="goBack" @refresh="store.refreshSessions" @toggle-select="store.toggleSessionSelection"
      :prompt-entries="promptEntries" :prompt-entries-loading="promptEntriesLoading"
      :prompt-entries-error="promptEntriesError" :prompt-entries-warnings="promptEntriesWarnings"
      @request-delete="requestDelete" />
  </section>
</template>

<style scoped>
.detail-page {
  display: grid;
  height: 100%;
  min-height: 0;
}
</style>

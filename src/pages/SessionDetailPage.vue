<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import SessionDetail from "../components/SessionDetail.vue";
import StatePanel from "../components/StatePanel.vue";
import { useSessionStore } from "../composables/useSessionStore";

const route = useRoute();
const router = useRouter();
const store = useSessionStore();

const sessionId = computed(() => String(route.params.sessionId ?? ""));
const session = computed(() => store.getSessionById(sessionId.value));
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
      :delete-disabled="store.isDeleting" :is-deleting="isDeletingCurrent" @toggle-select="store.toggleSessionSelection"
      @request-delete="requestDelete">
      <template #toolbar-start>
        <button class="ui-button ui-button--ghost" type="button" @click="goBack">
          Back to List
        </button>
        <button class="ui-button" type="button" :disabled="store.loading" @click="store.refreshSessions">
          {{ store.loading ? "Refreshing..." : "Refresh" }}
        </button>
      </template>
    </SessionDetail>
  </section>
</template>

<style scoped>
.detail-page {
  display: grid;
}
</style>

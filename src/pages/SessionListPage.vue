<script setup lang="ts">
import { useRouter } from "vue-router";
import SessionList from "../components/SessionList.vue";
import StatePanel from "../components/StatePanel.vue";
import { useSessionStore } from "../composables/useSessionStore";

const router = useRouter();
const store = useSessionStore();

function openSession(sessionId: string): void {
  void router.push({
    name: "session-detail",
    params: { sessionId },
  });
}
</script>

<template>
  <StatePanel v-if="store.errorMessage" eyebrow="Load Failed" title="The session list could not be loaded" tone="error">
    {{ store.errorMessage }}
    <template #actions>
      <button class="ui-button" type="button" @click="store.refreshSessions">
        Retry
      </button>
    </template>
  </StatePanel>

  <StatePanel v-else-if="store.loading && !store.sessions.length" eyebrow="Loading"
    title="Scanning the local Codex session index">
    Reading the session list, recent prompt previews, and structured log counts
    from <code>~/.codex</code>.
  </StatePanel>

  <StatePanel v-else-if="!store.sessions.length" eyebrow="Empty" title="No sessions were returned">
    Check whether <code>list_sessions</code> can reach your local Codex data,
    then refresh the index again.
  </StatePanel>

  <section v-else class="list-page">
    <SessionList :sessions="store.sessions" :selected-ids="store.selectedIds" :selection-disabled="store.isDeleting"
      :loading="store.loading" :can-bulk-delete="store.canBulkDelete" @toggle-all="store.toggleAllSelection"
      @toggle-select="store.toggleSessionSelection" @open-session="openSession" @refresh="store.refreshSessions"
      @request-batch-delete="store.openBatchDeleteDialog" />
  </section>
</template>

<style scoped>
.list-page {
  display: grid;
}
</style>

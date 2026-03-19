<script setup lang="ts">
import { computed } from "vue";
import { NFlex } from "naive-ui";
import { useRouter } from "vue-router";
import SessionListBrowser from "../components/session/SessionListBrowser.vue";
import SessionListStateCard from "../components/session/SessionListStateCard.vue";
import { useSessionStore } from "../composables/useSessionStore";

const router = useRouter();
const store = useSessionStore();

const hasError = computed(() => Boolean(store.errorMessage));
const isLoading = computed(() => store.loading && !store.sessions.length);
const isEmpty = computed(
  () => !hasError.value && !isLoading.value && !store.sessions.length,
);

function openSession(sessionId: string): void {
  void router.push({
    name: "session-detail",
    params: { sessionId },
  });
}
</script>

<template>
  <SessionListStateCard
    v-if="hasError"
    state="error"
    :error-message="store.errorMessage"
    @retry="store.refreshSessions"
  />

  <SessionListStateCard
    v-else-if="isLoading"
    state="loading"
    @retry="store.refreshSessions"
  />

  <SessionListStateCard
    v-else-if="isEmpty"
    state="empty"
    @retry="store.refreshSessions"
  />

  <n-flex v-else vertical style="flex: 1; min-height: 0; overflow: hidden;">
    <SessionListBrowser
      style="flex: 1; min-height: 0;"
      :sessions="store.sessions"
      :selected-ids="store.selectedIds"
      :can-bulk-delete="store.canBulkDelete"
      :is-deleting="store.isDeleting"
      :loading="store.loading"
      @refresh="store.refreshSessions"
      @bulk-delete="store.openBatchDeleteDialog"
      @toggle-all="store.toggleAllSelection"
      @toggle-session="store.toggleSessionSelection"
      @open-session="openSession"
    />
  </n-flex>
</template>

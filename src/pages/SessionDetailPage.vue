<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import SessionDetail from "../components/SessionDetail.vue";
import { useSessionStore } from "../composables/useSessionStore";

const route = useRoute();
const router = useRouter();
const store = useSessionStore();

const sessionId = computed(() => String(route.params.sessionId ?? ""));
const session = computed(() => store.getSessionById(sessionId.value));
const selected = computed(
  () => Boolean(session.value) && store.selectedIds.includes(session.value.id),
);
const isDeletingCurrent = computed(
  () =>
    Boolean(session.value) &&
    store.activeDeleteIds.includes(session.value.id),
);

function goBack(): void {
  void router.push({ name: "session-list" });
}

function requestDelete(targetSessionId: string): void {
  store.openDeleteDialog([targetSessionId]);
}
</script>

<template>
  <section
    v-if="store.errorMessage && !session"
    class="detail-page__state detail-page__state--error"
  >
    <p class="detail-page__eyebrow">Load Failed</p>
    <h2 class="detail-page__title">The session detail could not be loaded</h2>
    <p class="detail-page__message">{{ store.errorMessage }}</p>
    <div class="detail-page__actions">
      <button class="detail-page__button" type="button" @click="store.refreshSessions">
        Retry
      </button>
      <button class="detail-page__button detail-page__button--ghost" type="button" @click="goBack">
        Back to List
      </button>
    </div>
  </section>

  <section
    v-else-if="store.loading && !session"
    class="detail-page__state"
  >
    <p class="detail-page__eyebrow">Loading</p>
    <h2 class="detail-page__title">Preparing the session detail page</h2>
    <p class="detail-page__message">
      Waiting for the session index to finish loading before opening this
      record.
    </p>
  </section>

  <section v-else-if="!session" class="detail-page__state">
    <p class="detail-page__eyebrow">Missing</p>
    <h2 class="detail-page__title">This session is no longer available</h2>
    <p class="detail-page__message">
      It may have been deleted or the local index changed. Return to the list
      and pick another session.
    </p>
    <div class="detail-page__actions">
      <button class="detail-page__button" type="button" @click="goBack">
        Back to List
      </button>
      <button class="detail-page__button detail-page__button--ghost" type="button" @click="store.refreshSessions">
        Refresh Index
      </button>
    </div>
  </section>

  <section v-else class="detail-page">
    <header class="detail-page__toolbar">
      <div>
        <p class="detail-page__eyebrow">Session Detail Page</p>
        <h2 class="detail-page__title">
          Opened from the session list
        </h2>
        <p class="detail-page__message">
          Review the full preview and metadata here, then go back when you are
          done.
        </p>
      </div>

      <div class="detail-page__actions">
        <button class="detail-page__button detail-page__button--ghost" type="button" @click="goBack">
          Back to List
        </button>
        <button class="detail-page__button" type="button" :disabled="store.loading" @click="store.refreshSessions">
          {{ store.loading ? "Refreshing..." : "Refresh" }}
        </button>
      </div>
    </header>

    <SessionDetail
      :session="session"
      :selected="selected"
      :selection-disabled="store.isDeleting"
      :delete-disabled="store.isDeleting"
      :is-deleting="isDeletingCurrent"
      @toggle-select="store.toggleSessionSelection"
      @request-delete="requestDelete"
    />
  </section>
</template>

<style scoped>
.detail-page,
.detail-page__state {
  display: grid;
  gap: 1rem;
}

.detail-page__toolbar,
.detail-page__state {
  padding: 1.25rem;
  border-radius: 1.55rem;
  border: 1px solid var(--border);
  background:
    linear-gradient(180deg, rgba(13, 31, 24, 0.96), rgba(8, 18, 15, 0.98)),
    var(--panel);
  box-shadow: 0 26px 70px rgba(1, 8, 6, 0.34);
}

.detail-page__eyebrow {
  margin: 0;
  color: var(--accent-soft);
  font-size: 0.78rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
}

.detail-page__title {
  margin: 0.2rem 0 0;
  color: var(--heading);
  font-size: clamp(1.5rem, 3vw, 2.1rem);
}

.detail-page__message {
  margin: 0.55rem 0 0;
  color: var(--text-soft);
  line-height: 1.75;
}

.detail-page__toolbar {
  display: flex;
  justify-content: space-between;
  gap: 1rem;
  align-items: flex-start;
}

.detail-page__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.detail-page__button {
  min-height: 2.9rem;
  padding: 0 1.1rem;
  border: none;
  border-radius: 999px;
  background: linear-gradient(135deg, var(--accent-soft), #0b93ab);
  color: #05110d;
  font-weight: 800;
}

.detail-page__button--ghost {
  background: rgba(255, 255, 255, 0.06);
  color: var(--heading);
  border: 1px solid rgba(112, 193, 174, 0.12);
}

.detail-page__state--error {
  border-color: rgba(255, 123, 92, 0.28);
  background:
    linear-gradient(180deg, rgba(46, 18, 14, 0.95), rgba(24, 10, 8, 0.96)),
    var(--panel);
}

@media (max-width: 760px) {
  .detail-page__toolbar {
    flex-direction: column;
  }

  .detail-page__actions {
    width: 100%;
  }

  .detail-page__button {
    width: 100%;
  }
}
</style>

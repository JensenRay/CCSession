<script setup lang="ts">
import { useRouter } from "vue-router";
import SessionList from "../components/SessionList.vue";
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
  <section v-if="store.errorMessage" class="page-state page-state--error">
    <p class="page-state__eyebrow">Load Failed</p>
    <h2 class="page-state__title">The session list could not be loaded</h2>
    <p class="page-state__message">{{ store.errorMessage }}</p>
    <button class="page-state__button" type="button" @click="store.refreshSessions">
      Retry
    </button>
  </section>

  <section
    v-else-if="store.loading && !store.sessions.length"
    class="page-state"
  >
    <p class="page-state__eyebrow">Loading</p>
    <h2 class="page-state__title">Scanning the local Codex session index</h2>
    <p class="page-state__message">
      Reading the session list, recent prompt previews, and structured log
      counts from <code>~/.codex</code>.
    </p>
  </section>

  <section v-else-if="!store.sessions.length" class="page-state">
    <p class="page-state__eyebrow">Empty</p>
    <h2 class="page-state__title">No sessions were returned</h2>
    <p class="page-state__message">
      Check whether <code>list_sessions</code> can reach your local Codex data,
      then refresh the index again.
    </p>
  </section>

  <section v-else class="list-page">
    <div class="list-page__intro">
      <p class="list-page__eyebrow">Session List</p>
      <h2 class="list-page__title">Browse compact cards first</h2>
      <p class="list-page__message">
        Nothing is expanded by default. Pick a card only when you want to jump
        into a dedicated detail page.
      </p>
    </div>

    <SessionList
      :sessions="store.sessions"
      :selected-ids="store.selectedIds"
      active-session-id=""
      :selection-disabled="store.isDeleting"
      @toggle-all="store.toggleAllSelection"
      @toggle-select="store.toggleSessionSelection"
      @open-session="openSession"
    />
  </section>
</template>

<style scoped>
.list-page {
  display: grid;
  gap: 1rem;
}

.list-page__intro,
.page-state {
  display: grid;
  gap: 0.8rem;
  padding: 1.25rem;
  border-radius: 1.55rem;
  border: 1px solid var(--border);
  background:
    linear-gradient(180deg, rgba(13, 31, 24, 0.96), rgba(8, 18, 15, 0.98)),
    var(--panel);
  box-shadow: 0 26px 70px rgba(1, 8, 6, 0.34);
}

.list-page__eyebrow,
.page-state__eyebrow {
  margin: 0;
  color: var(--accent-soft);
  font-size: 0.78rem;
  font-weight: 700;
  letter-spacing: 0.16em;
  text-transform: uppercase;
}

.list-page__title,
.page-state__title {
  margin: 0;
  color: var(--heading);
  font-size: clamp(1.5rem, 3vw, 2.1rem);
}

.list-page__message,
.page-state__message {
  margin: 0;
  color: var(--text-soft);
  line-height: 1.75;
}

.page-state--error {
  border-color: rgba(255, 123, 92, 0.28);
  background:
    linear-gradient(180deg, rgba(46, 18, 14, 0.95), rgba(24, 10, 8, 0.96)),
    var(--panel);
}

.page-state__message code {
  padding: 0.1rem 0.42rem;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.08);
  color: var(--accent);
}

.page-state__button {
  min-height: 2.9rem;
  width: fit-content;
  padding: 0 1.1rem;
  border: none;
  border-radius: 999px;
  background: linear-gradient(135deg, var(--accent-soft), #0b93ab);
  color: #05110d;
  font-weight: 800;
}
</style>

import { reactive } from "vue";
import { createSessionDeleteState } from "./session-store/useSessionDelete";
import { createSessionIndexState } from "./session-store/useSessionIndex";

const sessionIndexState = createSessionIndexState();
const sessionDeleteState = createSessionDeleteState(sessionIndexState);

const sessionStore = reactive({
  ...sessionIndexState,
  ...sessionDeleteState,
});

export function useSessionStore() {
  return sessionStore;
}

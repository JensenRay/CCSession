import { computed, ref, watch, type Ref } from "vue";
import {
  sessionPrompts,
  toSessionCommandError,
} from "../api/sessions";

export function useSessionPrompts(
  sessionId: Readonly<Ref<string>>,
  previewEntries: Readonly<Ref<string[]>>,
) {
  const promptEntries = ref<string[]>([]);
  const promptEntriesLoading = ref(false);
  const promptEntriesError = ref("");
  const promptEntriesWarnings = ref<string[]>([]);
  const hasResolvedPromptEntries = ref(false);

  watch(
    sessionId,
    async (nextSessionId) => {
      promptEntries.value = [];
      promptEntriesError.value = "";
      promptEntriesWarnings.value = [];
      promptEntriesLoading.value = false;
      hasResolvedPromptEntries.value = false;

      if (!nextSessionId) {
        return;
      }

      promptEntriesLoading.value = true;

      try {
        const data = await sessionPrompts({ sessionId: nextSessionId });
        if (sessionId.value !== nextSessionId) {
          return;
        }

        promptEntries.value = data.prompts;
        promptEntriesWarnings.value = data.warnings;
        hasResolvedPromptEntries.value = true;
      } catch (error) {
        if (sessionId.value !== nextSessionId) {
          return;
        }

        const commandError = toSessionCommandError(
          error,
          "The full prompt history could not be loaded.",
        );

        promptEntriesError.value = [
          commandError.message,
          ...commandError.details,
        ].join("\n");
      } finally {
        if (sessionId.value === nextSessionId) {
          promptEntriesLoading.value = false;
        }
      }
    },
    { immediate: true },
  );

  const visiblePromptEntries = computed(() => {
    if (
      !hasResolvedPromptEntries.value &&
      !promptEntriesError.value &&
      !promptEntries.value.length
    ) {
      return previewEntries.value;
    }

    return promptEntries.value;
  });

  return {
    promptEntries: visiblePromptEntries,
    promptEntriesLoading,
    promptEntriesError,
    promptEntriesWarnings,
  };
}

<script setup lang="ts">
import { NAlert, NCard, NEl, NEmpty, NFlex, NList, NListItem, NScrollbar, NTag, NText } from "naive-ui";

defineProps<{
  sessionId: string;
  promptEntries: string[];
  promptEntriesLoading: boolean;
  promptEntriesError: string;
  promptEntriesWarnings: string[];
}>();
</script>

<template>
  <n-card
    size="small"
    title="Prompt History"
    style="height: 100%; min-height: 0;"
    content-style="height: 100%; display: flex; flex-direction: column; min-height: 0; overflow: hidden;"
  >
    <template #header-extra>
      <n-tag size="small" round>
        {{ promptEntries.length }} prompts
      </n-tag>
    </template>

    <n-flex vertical style="flex: 1; min-height: 0; gap: 12px;">
      <n-alert v-if="promptEntriesLoading" type="info" :show-icon="false">
        {{
          promptEntries.length
            ? "Loading the full prompt history. Showing the current preview while waiting."
            : "Loading the full prompt history..."
        }}
      </n-alert>

      <n-alert v-if="promptEntriesError" type="error">
        <n-text style="white-space: pre-wrap; overflow-wrap: anywhere;">
          {{ promptEntriesError }}
        </n-text>
      </n-alert>

      <n-alert
        v-for="warning in promptEntriesWarnings"
        :key="warning"
        type="warning"
        :show-icon="false"
      >
        {{ warning }}
      </n-alert>

      <n-el v-if="promptEntries.length" style="flex: 1; min-height: 0; overflow: hidden;">
        <n-scrollbar style="height: 100%;">
          <n-list bordered>
            <n-list-item v-for="(item, index) in promptEntries" :key="`${sessionId}-${index}`">
              <n-text style="white-space: pre-wrap; overflow-wrap: anywhere;">
                {{ item }}
              </n-text>
            </n-list-item>
          </n-list>
        </n-scrollbar>
      </n-el>

      <n-empty
        v-else-if="!promptEntriesError"
        description="No prompt entries were returned for this session."
      />
    </n-flex>
  </n-card>
</template>

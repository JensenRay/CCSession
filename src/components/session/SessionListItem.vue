<script setup lang="ts">
import { NCheckbox, NEl, NEllipsis, NListItem, NSpace, NTag, NText, NThing } from "naive-ui";
import type { SessionListItem } from "../../types";
import { compactPath, formatCount, getSessionSummary } from "../../utils/sessionFormat";

defineProps<{
  session: SessionListItem;
  selected: boolean;
  disabled: boolean;
}>();

defineEmits<{
  (event: "open"): void;
  (event: "toggle"): void;
}>();
</script>

<template>
  <n-list-item @click="$emit('open')">
    <template #prefix>
      <n-el @click.stop>
        <n-checkbox
          aria-label="Select session"
          :checked="selected"
          :disabled="disabled"
          @update:checked="$emit('toggle')"
        />
      </n-el>
    </template>

    <n-thing>
      <template #header>
        <n-ellipsis :line-clamp="2">
          {{ getSessionSummary(session) }}
        </n-ellipsis>
      </template>

      <template #description>
          <n-space vertical size="small">
            <n-text depth="3">
              {{ compactPath(session.cwd) }}
            </n-text>
            <n-space>
              <n-tag v-if="session.archived" size="small" round>
                Archived
              </n-tag>
              <n-tag size="small" round type="info">
                {{ formatCount(session.tokensUsed, true) }} tok
              </n-tag>
            </n-space>
          </n-space>
      </template>
    </n-thing>
  </n-list-item>
</template>

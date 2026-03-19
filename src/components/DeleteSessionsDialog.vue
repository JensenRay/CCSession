<script setup lang="ts">
import {
  NButton,
  NCard,
  NFlex,
  NList,
  NListItem,
  NModal,
  NSpace,
  NTag,
  NText,
} from "naive-ui";
import { computed } from "vue";
import { useSessionStore } from "../composables/useSessionStore";

const store = useSessionStore();

const isOpen = computed(() => Boolean(store.pendingDeleteIds.length));

function handleUpdateShow(value: boolean): void {
  if (!value) {
    store.closeDeleteDialog();
  }
}
</script>

<template>
  <n-modal
    :show="isOpen"
    :mask-closable="!store.isDeleting"
    :close-on-esc="!store.isDeleting"
    @update:show="handleUpdateShow"
  >
    <n-card
      :title="store.deleteDialogTitle"
      size="large"
      role="dialog"
      aria-modal="true"
      style="width: min(640px, calc(100vw - 2rem)); max-height: calc(100vh - 2rem);"
    >
      <n-space vertical size="large">
        <n-text depth="3">
          {{ store.deleteDialogDescription }}
        </n-text>

        <n-list bordered style="max-height: min(50vh, 24rem); overflow: auto;">
          <n-list-item v-for="item in store.deleteDialogItems" :key="item.id">
            <n-space vertical size="small">
              <n-text>{{ item.summary }}</n-text>
              <n-text code>{{ item.id }}</n-text>
            </n-space>
          </n-list-item>
        </n-list>

        <n-flex justify="space-between" align="center" :wrap="true">
          <n-tag v-if="store.deleteDialogItems.length > 1" size="small" round>
            {{ store.deleteDialogItems.length }} sessions
          </n-tag>

          <n-space>
            <n-button secondary :disabled="store.isDeleting" @click="store.closeDeleteDialog">
              Cancel
            </n-button>
            <n-button type="error" :loading="store.isDeleting" @click="store.confirmDelete">
              {{ store.deleteDialogConfirmLabel }}
            </n-button>
          </n-space>
        </n-flex>
      </n-space>
    </n-card>
  </n-modal>
</template>

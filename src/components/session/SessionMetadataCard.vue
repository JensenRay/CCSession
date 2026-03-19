<script setup lang="ts">
import { computed } from "vue";
import { NCard, NGrid, NGridItem, NEllipsis, NText } from "naive-ui";
import type { SessionListItem } from "../../types";
import { formatCount, formatFullTimestamp } from "../../utils/sessionFormat";

const props = defineProps<{
  session: SessionListItem;
}>();

const metadataItems = computed(() => [
  {
    label: "Session ID",
    value: props.session.id,
    code: true,
  },
  {
    label: "Project",
    value: props.session.cwd,
    code: true,
  },
  {
    label: "Created",
    value: formatFullTimestamp(props.session.createdAt),
    code: false,
  },
  {
    label: "Updated",
    value: formatFullTimestamp(props.session.updatedAt),
    code: false,
  },
  {
    label: "Tokens Used",
    value: formatCount(props.session.tokensUsed),
    code: false,
  },
  {
    label: "Source",
    value: props.session.source || "Unknown",
    code: false,
  },
  {
    label: "Provider",
    value: props.session.modelProvider || "Unknown",
    code: false,
  },
  {
    label: "Archived",
    value: props.session.archived ? "Yes" : "No",
    code: false,
  },
]);
</script>

<template>
  <n-card size="small" title="Metadata">
    <n-grid :cols="4" :x-gap="12" :y-gap="12">
      <n-grid-item v-for="item in metadataItems" :key="item.label" style="display: flex;">
        <n-card
          size="small"
          embedded
          :title="item.label"
          style="width: 100%;"
          content-style="min-width: 0; height: 32px; display: flex; align-items: center;"
        >
          <n-ellipsis style="width: 100%;" :line-clamp="1" :tooltip="true">
            <n-text :code="item.code">
              {{ item.value }}
            </n-text>
          </n-ellipsis>
        </n-card>
      </n-grid-item>
    </n-grid>
  </n-card>
</template>

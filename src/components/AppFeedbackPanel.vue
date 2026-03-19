<script setup lang="ts">
import { computed } from "vue";
import {
  NAlert,
  NFlex,
  NList,
  NListItem,
  NSpace,
  NTag,
  NText,
} from "naive-ui";
import type { ActionNotice } from "../composables/useSessionStore";
import type { SessionDeleteReport } from "../types";

const props = defineProps<{
  actionNotice: ActionNotice | null;
  summaryReports: SessionDeleteReport[];
  warnings: string[];
}>();

const noticeType = computed(() => {
  switch (props.actionNotice?.tone) {
    case "success":
      return "success";
    case "warning":
      return "warning";
    default:
      return "error";
  }
});

const hasContent = computed(
  () =>
    Boolean(props.actionNotice) ||
    props.summaryReports.length > 0 ||
    props.warnings.length > 0,
);
</script>

<template>
  <n-space v-if="hasContent" vertical size="large">
    <n-alert
      v-if="actionNotice"
      :type="noticeType"
      :title="actionNotice.title"
    >
      <n-space vertical :size="12">
        <n-text style="white-space: pre-line;">
          {{ actionNotice.message }}
        </n-text>

        <n-list v-if="actionNotice.details.length" bordered>
          <n-list-item v-for="detail in actionNotice.details" :key="detail">
            {{ detail }}
          </n-list-item>
        </n-list>

        <n-list v-if="summaryReports.length" bordered>
          <n-list-item v-for="report in summaryReports" :key="report.sessionId">
            <n-space vertical :size="8">
              <n-flex justify="space-between" align="center">
                <n-text strong>
                  {{ report.sessionId }}
                </n-text>
                <n-tag
                  size="small"
                  round
                  :type="report.error ? 'error' : report.status === 'deleted' ? 'warning' : 'default'"
                >
                  {{ report.status }}
                </n-tag>
              </n-flex>

              <n-text style="white-space: pre-line;">
                {{ report.error || report.warnings.join(" / ") }}
              </n-text>
            </n-space>
          </n-list-item>
        </n-list>
      </n-space>
    </n-alert>

    <n-alert v-if="warnings.length" type="warning" title="Warnings">
      <n-list bordered>
        <n-list-item v-for="warning in warnings" :key="warning">
          {{ warning }}
        </n-list-item>
      </n-list>
    </n-alert>
  </n-space>
</template>

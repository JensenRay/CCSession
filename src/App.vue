<script setup lang="ts">
import { onMounted } from "vue";
import { NConfigProvider, NFlex, NGlobalStyle } from "naive-ui";
import { RouterView } from "vue-router";
import AppFeedbackPanel from "./components/AppFeedbackPanel.vue";
import AppFrame from "./components/AppFrame.vue";
import DeleteSessionsDialog from "./components/DeleteSessionsDialog.vue";
import { useSessionStore } from "./composables/useSessionStore";
import { useThemeMode } from "./composables/useThemeMode";

const store = useSessionStore();
const { naiveTheme, themeMode, setThemeMode } = useThemeMode();

onMounted(() => {
  void store.ensureSessionsLoaded();
});
</script>

<template>
  <n-config-provider :theme="naiveTheme">
    <n-global-style />

    <AppFrame
      :theme-mode="themeMode"
      @update:theme-mode="setThemeMode"
    >
      <n-flex vertical size="large" style="height: 100%; min-height: 0; overflow: hidden;">
        <AppFeedbackPanel
          :action-notice="store.actionNotice"
          :summary-reports="store.summaryReports"
          :warnings="store.warnings"
        />

        <n-flex vertical style="flex: 1; min-height: 0; overflow: hidden;">
          <RouterView v-slot="{ Component }">
            <component :is="Component" style="flex: 1; min-height: 0; overflow: hidden;" />
          </RouterView>
        </n-flex>
      </n-flex>
    </AppFrame>

    <DeleteSessionsDialog />
  </n-config-provider>
</template>

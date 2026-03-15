import { createRouter, createWebHashHistory } from "vue-router";
import SessionDetailPage from "./pages/SessionDetailPage.vue";
import SessionListPage from "./pages/SessionListPage.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      name: "session-list",
      component: SessionListPage,
    },
    {
      path: "/session/:sessionId",
      name: "session-detail",
      component: SessionDetailPage,
    },
  ],
});

export default router;

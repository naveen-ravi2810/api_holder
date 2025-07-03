<template>
  <div class="top-Bar">
    <topbar :activeMenu="activeMenu" />
  </div>

  <div class="content-Container">
    <div>
      <sidebar :activeMenu="activeMenu" />
      <div class="p-4 bg-gray-50 min-h-screen">
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from "vue";
import { useRoute } from "vue-router";
import get from "lodash/get";
import UrlMenu from "~/utils/UrlMenu";
import topbar from "~/components/layout/topbar.vue";
import sidebar from "~/components/layout/sidebar.vue";
const route = useRoute();
const activeMenu = computed(() => {
  const current = UrlMenu[route.name];
  return get(current, "Menu");
});
const activeSubMenu = computed(() => {
  return get(UrlMenu[route.name], "subMenu");
});
</script>

<style scoped>
/* Add your styles here */
</style>

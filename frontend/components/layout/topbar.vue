<template>
  <div
    class="w-full h-12 flex gap-3 justify-between px-10 border-2 border-green-500 items-center bg-primary2"
  >
    <div class="text-surface">Company Logo</div>
    <div class="flex gap-3 h-full items-center">
      <Button
        v-for="menu in Menus"
        :key="menu.name"
        :label="menu.name"
        icon-name="dashboard"
      />
    </div>
  </div>
</template>
<script setup>
import Menus from "~/utils/MenuItems";
import Button from "~/components/core/Button.vue";
defineProps({
  activeMenu: String,
});
</script>

<style scoped></style>

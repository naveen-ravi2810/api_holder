<template>
  <div :class="getStyle">
    <slot />
  </div>
</template>

<script setup>
import { computed } from "vue";

const props = defineProps({
  columns: {
    type: Number,
    default: 4,
  },
  gap: {
    type: Number,
    default: 4,
  },
});

const getStyle = computed(() => {
  const columnsClass =
    {
      1: "grid-cols-1",
      2: "grid-cols-2",
      3: "grid-cols-3",
      4: "grid-cols-4",
      5: "grid-cols-5",
      6: "grid-cols-6",
      12: "grid-cols-12",
    }[props.columns] || "grid-cols-4";

  const gapClass =
    {
      0: "gap-0",
      1: "gap-1",
      2: "gap-2",
      4: "gap-4",
      6: "gap-6",
      8: "gap-8",
    }[props.gap] || "gap-4";

  return `grid ${columnsClass} grid-rows-2 ${gapClass}`;
});
</script>

<style scoped></style>

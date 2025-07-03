<template>
  <component :is="iconComponent" :color="getColor" />
</template>
<script setup>
import SystemColors from "~/utils/colors";
import Icons from "~/utils/Icons";
const props = defineProps({
  name: {
    type: String,
    required: true,
  },
  color: {
    type: String,
    default: "primary",
  },
});
const getColor = computed(() => {
  return SystemColors[props.color];
});
const iconComponent = computed(() => {
  return Icons[props.name];
});
</script>
<script scoped></script>

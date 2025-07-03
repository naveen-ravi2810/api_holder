<template>
  <div
    class="text-primary bg-primary-light flex items-center"
    @click="btnClicked()"
  >
    <div v-if="iconName && iconPosition === 'left'">
      <Icons :name="iconName" :color="iconColor" />
    </div>
    {{ label }}
    <div v-if="iconName && iconPosition === 'right'">
      <Icons :name="iconName" :color="iconColor" />
    </div>
  </div>
</template>
<script setup>
import Icons from "~/components/core/Icons.vue";
defineProps({
  label: {
    type: String,
    required: true,
  },
  iconName: {
    type: String,
    default: "",
  },
  iconPosition: {
    type: String,
    default: "left",
  },
  iconColor: {
    type: String,
    default: "surface",
  },
});
const btnClicked = () => {
  alert("BTN Clicked");
};
</script>
<style scoped></style>

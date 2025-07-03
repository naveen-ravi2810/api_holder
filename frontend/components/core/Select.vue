<template>
  <div>
    <select @change="setValue">
      <option
        v-for="option in options"
        :key="option.value"
        :value="option.value"
        :selected="option.value === defaultValue"
      >
        {{ option.label }}
      </option>
    </select>
  </div>
</template>

<script setup>
const props = defineProps({
  options: {
    type: Array,
    default: () => [],
  },
  defaultValue: {
    type: String,
    default: "",
  },
});

const emit = defineEmits(["selectedValue"]);
const setValue = (e) => {
  const selected = e.target.value;
  emit("selectedValue", selected);
};
</script>

<style scoped></style>

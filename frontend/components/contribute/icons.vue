<template>
  <div>
    <div class="ThanksGiving flex justify-between">
      <p
        class="flex items-center gap-2 bg-danger w-fit px-2 border-2 border-text text-surface"
      >
        Icons from
        <a href="https://www.svgrepo.com/"
          ><img src="https://www.svgrepo.com/logo.svg" height="32" width="32"
        /></a>
      </p>
      <p class="flex justify-center gap-2">
        Select Color
        <Select
          :options="colorOptions"
          @selectedValue="(e) => (iconColor = e)"
          :defaultValue="iconColor"
        />
      </p>
    </div>
    <Grid :columns="3">
      <div
        v-for="(component, name) in IconsComponent"
        class="flex items-center gap-2"
      >
        <Icons :name="name" :color="iconColor" />
        {{ name }}
      </div>
    </Grid>
  </div>
</template>
<script setup>
import Icons from "~/components/core/Icons.vue";
import Grid from "~/components/core/Grid.vue";
import IconsComponent from "~/utils/Icons";
import SystemColors from "~/utils/colors";
import Select from "~/components/core/Select.vue";
import forEach from "lodash/forEach";
import capitalize from "lodash/capitalize";
import { ref, computed } from "vue";
const iconColor = ref("secondary2");
const colorOptions = computed(() => {
  const colorSelectOptions = [];
  forEach(SystemColors, (value, key) => {
    colorSelectOptions.push({
      label: capitalize(key),
      value: key,
    });
  });
  return colorSelectOptions;
});
</script>
<script scoped></script>

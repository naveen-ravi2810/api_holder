<template>
  <div>
    <Grid :columns="3" :gap="3">
      <div
        v-for="(hexa, code) in colors"
        :key="code"
        :style="{ color: hexa }"
        class="flex gap-2 items-center"
        :title="hexa + '->' + code"
      >
        <div
          class="h-3 w-3 border-black border-2"
          :style="{ background: hexa }"
        ></div>
        {{ hexa }} -> {{ code }}
      </div>
    </Grid>
  </div>
</template>
<script setup>
import colors from "~/utils/colors";
import Grid from "../core/Grid.vue";
defineProps({});
</script>
<style scoped></style>

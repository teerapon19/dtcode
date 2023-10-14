<script setup lang="ts">
import { ref } from "vue";
import useChangeTab from "../stores/useChangeNav";

const bar = ref<HTMLElement>();
const barSize = ref(0);

let startPosition = 0;

const dragStart = (de: DragEvent) => {
  barSize.value = bar.value?.clientWidth || 0;
  startPosition = de.x;
};
const dragMove = (de: DragEvent) => {
  if (bar.value) {
    const difX = de.x - startPosition;
    startPosition = de.x;
    barSize.value += difX;
    bar.value.style.width = `${barSize.value}px`;
  }
};

const tab = useChangeTab();
</script>

<template>
  <div
    ref="bar"
    class="h-screen w-48 relative bg-gray-900 no-selection overflow-hidden"
  >
    <div class="flex flex-col text-white">
      <h6 class="p-3 text-sm uppercase">{{ tab.getTitle }}</h6>
      <div class="p-3 text-sm">
        <div class="mb-2 cursor-pointer">JSON format/validate</div>
        <div class="mb-2 cursor-pointer">JWT Debugger</div>
        <div class="mb-2 cursor-pointer">Remove empty</div>
      </div>
    </div>
    <div
      draggable="true"
      @dragstart="dragStart"
      @drag="dragMove"
      style="width: 1px; cursor: col-resize"
      class="absolute right-0 top-0 hover:bg-blue-500 hover:w-3 bg-gray-700 h-screen"
    ></div>
  </div>
</template>

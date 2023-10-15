<script setup lang="ts">
// import { ref } from "vue";
import useTab from "../stores/useTab";

// import Notes from "./notes/index.vue";

const tab = useTab();
</script>

<template>
  <div class="h-screen w-full bg-white">
    <div class="flex flex-col h-full">
      <div class="flex w-full bg-gray-900 overflow-x-scroll no-scrollbar">
        <div
          v-for="[index, item] of tab.items.entries()"
          @click="tab.focus(index)"
          style="min-width: 125px; max-width: 200px"
          :class="tab.focusAt === index ? 'bg-gray-700' : 'hover:bg-gray-800'"
          class="px-4 py-1 text-white border-r border-gray-700 cursor-pointer flex flex-row justify-between no-selection"
        >
          <div class="overflow-hidden text-ellipsis whitespace-nowrap">
            {{ item.title }}
          </div>
          <div @click="tab.closeTab(item.id, index)" class="pl-4 font-bold">
            x
          </div>
        </div>
      </div>
      <div class="flex-1 h-full bg-gray-700">
        <!-- <Notes /> -->
      </div>
    </div>
  </div>
</template>

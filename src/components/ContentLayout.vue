<script setup lang="ts">
import { ref } from "vue";

// import Notes from "./notes/index.vue";

const tabs = ref(10);
</script>

<template>
  <div class="h-screen w-full bg-white">
    <div class="flex flex-col h-full">
      <div class="flex w-full bg-gray-900 overflow-x-scroll no-scrollbar">
        <div
          v-for="_ of tabs"
          style="min-width: 125px"
          class="px-4 py-1 text-white border-r border-gray-700 hover:bg-gray-700 cursor-pointer flex flex-row justify-between no-selection"
        >
          <div>Untitle</div>
          <div>x</div>
        </div>
      </div>
      <div class="flex-1 h-full bg-gray-700">
        <!-- <Notes /> -->
      </div>
    </div>
  </div>
</template>

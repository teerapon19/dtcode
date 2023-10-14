<script setup lang="ts">
import LeftNav from "./components/LeftNav.vue";
import NavExplorer from "./components/NavExplorer.vue";
import ContentLayout from "./components/ContentLayout.vue";
</script>

<template>
  <div class="fixed top-0 left-0 h-screen w-screen bg-gray-800">
    <div class="flex">
      <LeftNav />
      <NavExplorer />
      <ContentLayout class="flex-1 overflow-hidden" />
    </div>
  </div>
</template>

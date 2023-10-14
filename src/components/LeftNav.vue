<script setup lang="ts">
import AppIcon from "../assets/app.svg";
import NoteIcon from "../assets/note.svg";
import useChangeTab from "../stores/useChangeNav";

const tabs = useChangeTab();
</script>

<template>
  <div class="h-screen w-14 bg-gray-900 border-r border-gray-700 no-selection">
    <div class="flex flex-col justify-center">
      <div
        class="py-3 cursor-pointer"
        :class="tabs.position == 0 ? 'bg-slate-600' : ''"
        @click="
          () => {
            tabs.setPosition(0);
          }
        "
      >
        <img class="w-8 h-8 mx-auto" :src="NoteIcon" />
      </div>
      <div
        class="py-3 cursor-pointer"
        :class="tabs.position == 1 ? 'bg-slate-600' : ''"
        @click="
          () => {
            tabs.setPosition(1);
          }
        "
      >
        <img class="w-8 h-8 mx-auto" :src="AppIcon" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import AppIcon from "../assets/app.svg";
import NoteIcon from "../assets/note.svg";
import useNav from "../stores/useNav";

const nav = useNav();
</script>

<template>
  <div class="h-screen w-14 bg-gray-900 border-r border-gray-700 no-selection">
    <div class="flex flex-col justify-center">
      <div
        class="py-3 cursor-pointer"
        :class="nav.position == 0 ? 'bg-slate-600' : ''"
        @click="
          () => {
            nav.setPosition(0);
          }
        "
      >
        <img class="w-8 h-8 mx-auto" :src="NoteIcon" />
      </div>
      <div
        class="py-3 cursor-pointer"
        :class="nav.position == 1 ? 'bg-slate-600' : ''"
        @click="
          () => {
            nav.setPosition(1);
          }
        "
      >
        <img class="w-8 h-8 mx-auto" :src="AppIcon" />
      </div>
    </div>
  </div>
</template>

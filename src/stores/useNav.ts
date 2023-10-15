import { defineStore } from "pinia";

type Tab = {
  id: string;
  title: string;
};

const NAVS: Tab[] = [
  {
    id: "note",
    title: "Note",
  },
  {
    id: "app",
    title: "Application",
  },
];

const useChangeTab = defineStore("navs", {
  state: () => ({
    position: 0,
  }),
  getters: {
    getId: (state) => NAVS[state.position].id,
    getTitle: (state) => NAVS[state.position].title,
  },
  actions: {
    setPosition(pos: number) {
      this.position = pos;
    },
  },
});

export default useChangeTab;

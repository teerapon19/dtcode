import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";

type Tab = {
  id: number;
  title: string;
  content: string;
};

const useTab = defineStore("tabs", {
  state: () => ({
    items: [] as Tab[],
    focusAt: 1,
  }),
  actions: {
    focus(index: number) {
      this.focusAt = index;
    },
    async fetchTab() {
      const tabs = await invoke<Tab[]>("get_tabs");
      this.items = tabs;
    },
    async createNewTab(title: string, content: string) {
      await invoke<number>("create_new_tab", {
        title,
        content,
      });
      await this.fetchTab();
      this.focus(this.items.length - 1);
    },
    async closeTab(id: number, layoutPos: number) {
      await invoke("close_tab", {
        id,
      });
      if (layoutPos === this.items.length - 1) {
        const pos = layoutPos > 1 ? layoutPos - 1 : 0;
        this.focus(pos);
      }
      await this.fetchTab();
    },
  },
});

export default useTab;

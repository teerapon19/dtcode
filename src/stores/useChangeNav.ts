import { defineStore } from 'pinia'

type Tab = {
    id: string,
    title: string
}

const TABS: Tab[] = [
    {
        id: "note",
        title: "Note",
    },
    {
        id: "app",
        title: "Application",
    }
]

const useChangeTab = defineStore('tabs', {
    state: () => ({
      position: 0
    }),
    getters: {
        getId: (state) => TABS[state.position].id,
        getTitle: (state) => TABS[state.position].title
    },
    actions: {
        setPosition(pos: number) {
            this.position = pos;
        }
    }
})

export default useChangeTab;
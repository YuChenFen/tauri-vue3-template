import { defineStore } from 'pinia'

export const useThemeStore = defineStore('theme', {
    state: () => ({
        theme: "light",
        _cssArray: [
            "theme-color",
            "text-color",
            "text-button-background-color",
            "text-button-color",
            "text-button-hover-background-color",
            "text-button-shadow",
            "text-button-hover-color",
            "tag-button-select-text-color"
        ]
    }),
    getters: {},
    actions: {
        async changeTheme(theme) {
            this.theme = theme;
            let root = document.querySelector(":root")
            for(let i = 0, len = this._cssArray.length; i < len; i++){
                root.style.setProperty(`--${this._cssArray[i]}`, `var(--${theme}-${this._cssArray[i]})`);
            }
            
        }
    },
})
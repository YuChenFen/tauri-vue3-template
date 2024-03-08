import { defineStore } from 'pinia'

export const useThemeStore = defineStore('theme', {
    state: () => ({
        theme: "light",
        _cssArray: [
            "theme-color",
            "text-color",
            "app-background-color",
            "app-color",
            "app-hover-background-color",
            "app-shadow",
            "app-hover-color",
            "app-highlight-text-color",
            "app-card-content-background-color",
            "app-card-footer-background-color"
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
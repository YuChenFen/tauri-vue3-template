import { defineStore } from 'pinia'

export const useThemeStore = defineStore('theme', {
    state: () => ({
        // light and dark
        _theme: "light",
    }),
    getters: {},
    actions: {
        async getTheme() {
            return this._theme;
        },
        async setTheme(theme) {
            this._theme = theme;
            document.documentElement.setAttribute('data-theme', theme);
        },
        async changeTheme() {
            this._theme = this._theme === "light" ? "dark" : "light";
            document.documentElement.setAttribute('data-theme', this._theme);
        }
    },
})
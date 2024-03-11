import { defineStore } from 'pinia'

export const useThemeStore = defineStore('theme', {
    state: () => ({
        _theme: "light",
    }),
    getters: {},
    actions: {
        async changeTheme(theme) {
            this._theme = theme;
            document.documentElement.setAttribute('data-theme', theme);
        },
        async getTheme() {
            return this._theme;
        }
    },
})
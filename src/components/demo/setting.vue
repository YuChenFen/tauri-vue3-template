<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">设置</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>setting</span>
        </div>
        <div class="line">
            <text-button text="切换主题" @click="chuangeTheme"></text-button>
        </div>
    </div>
</template>

<script setup>
import textButton from '@/components/button/textButton.vue';
import { useThemeStore } from '@/stores/theme.js';
import { ref } from 'vue';
const themeStore = useThemeStore();
function chuangeTheme() {
    themeStore.changeTheme();
}
</script>

<style scoped></style>
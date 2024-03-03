<template>
    <div data-tauri-drag-region class="header">
        <div style="width: 100%;height: 3px;position: absolute;top: 0;"></div>
        <div class="header-btn close" @click="appWindow.close()">
            <svg-icon name="header-close"></svg-icon>
        </div>
        <div v-show="!isMaximize" class="header-btn" @click="() => { appWindow.maximize(); isMaximize = true }">
            <svg-icon name="header-maximize"></svg-icon>
        </div>
        <div v-show="isMaximize" class="header-btn" @click="() => { appWindow.unmaximize(); isMaximize = false }">
            <svg-icon name="header-unmaximize"></svg-icon>
        </div>
        <div class="header-btn" @click="appWindow.minimize()">
            <svg-icon name="header-minimize"></svg-icon>
        </div>
    </div>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';
import { ref } from 'vue'

let isMaximize = ref(false);
appWindow.onResized(async () => {
    isMaximize.value = await appWindow.isMaximized();
})
</script>

<style scoped>
.header {
    height: 30px;
    width: 100%;
    background: rgb(var(--theme-color));
    display: flex;
    flex-direction: row-reverse;
    align-items: center;
}

.header-btn {
    height: 100%;
    width: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
}

.header-btn:hover {
    background-color: rgba(150, 150, 150, 0.25);
}
.header-btn :deep(svg) {
    fill: rgb(var(--text-color));
}
.close:hover {
    background-color: rgb(232 17 35);
}
.close:hover :deep(svg) {
    fill: white;
}
</style>
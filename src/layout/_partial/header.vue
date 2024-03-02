<template>
    <div data-tauri-drag-region class="header">
        <button @click="appWindow.close()">close</button>
    </div>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';

</script>

<style scoped>
.header{
    height: 30px;
    width: 100%;
    background: var(--theme-color);
    display: flex;
    flex-direction: row-reverse;
    align-items: center;
}
</style>
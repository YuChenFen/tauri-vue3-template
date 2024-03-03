<template>
    <div>
        <textButton style="margin: 10px;" text="切换主题" @click="chuangeTheme"></textButton>
        <textButton style="margin: 10px;" text="按钮"></textButton>
        <iconButton>
            <svg-icon name="header-close"></svg-icon>
        </iconButton>
        <tagButton text="标签" :select="select">
            <svg-icon name="tag"></svg-icon>
        </tagButton>
        <twoCheckbox text="复选框"></twoCheckbox>
    </div>
</template>

<script setup>
import textButton from '@/components/button/textButton.vue';
import iconButton from '@/components/button/iconButton.vue';
import tagButton from '@/components/button/tagButton.vue';
import twoCheckbox from './checkbox/twoCheckbox.vue';
import { useThemeStore } from '@/stores/theme.js';
import { ref } from 'vue';

let select = ref(false);

const themeStore = useThemeStore();
let a = 0;
function chuangeTheme() {
    if (a) {
        themeStore.changeTheme('bright');
        a = 0;
    } else {
        themeStore.changeTheme('dark');
        a = 1;
    }
}
</script>

<style scoped></style>
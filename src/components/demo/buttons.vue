<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">按钮</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>button</span>
        </div>
        <div class="line">
            <text-button text="按钮" @click="() => { console.log('点击了按钮'); }"></text-button>
        </div>
        <div class="line">
            <text-button text="禁用按钮" :disabled="true"></text-button>
        </div>
        <div class="line">
            <icon-button @click="() => { console.log('点击了按钮'); }">
                <svg-icon style="margin: 2px;" name="header-close"></svg-icon>
            </icon-button>
        </div>
        <div class="line">
            <icon-button @click="() => { console.log('点击了按钮'); }">
                <svg-icon name="header-close"></svg-icon>
                <p style="margin-left: 5px;text-overflow: ellipsis;white-space: nowrap;">关闭</p>
            </icon-button>
        </div>
        <div class="line">
            <tag-button text="标签" v-model:select="select" @change="(v) => { console.log('点击了标签', select); }">
                <svg-icon name="tag"></svg-icon>
            </tag-button>
        </div>
    </div>
</template>

<script setup>
import textButton from '@/components/button/textButton.vue';
import iconButton from '@/components/button/iconButton.vue';
import tagButton from '@/components/button/tagButton.vue';
import { ref } from 'vue';

let select = ref(false);
</script>

<style></style>
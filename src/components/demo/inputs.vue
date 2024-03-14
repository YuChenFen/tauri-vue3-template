<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">输入框</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>input</span>
        </div>
        <div class="line">
            <text-input v-model:value="inputValue" input-type="text" placeholder="请输入内容" :show-clear="true"
                @change="(v) => { console.log(inputValue); }" width="200px">
            </text-input>
            <text-input v-model:value="inputValue" :input-type="passwordType" placeholder="请输入内容" :show-clear="true"
                width="200px">
                <template #suffix>
                    <div style="margin-right: 10px;display: flex;align-items: center;"
                        @mousedown="passwordType = 'text'" @mouseup="passwordType = 'password'"
                        @mouseleave="passwordType = 'password'" class="eyeIcon">
                        <svg-icon name="eye"></svg-icon>
                    </div>
                </template>
            </text-input>
        </div>
    </div>
</template>

<script setup>
import textInput from '@/components/input/textInput.vue';
import { ref } from 'vue';

const inputValue = ref('');
const passwordType = ref('password');
</script>

<style scoped>
.eyeIcon :deep(svg){
    fill: rgb(var(--app-color));
}
.eyeIcon:active :deep(svg) {
    fill: var(--app-hover-color);
}
</style>
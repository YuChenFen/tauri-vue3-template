<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">选择器</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>select</span>
        </div>
        <div class="line">
            <text-select :options="items" :defaultSelected="item" :width="120" @change="(i) => item = i"
                style="margin-left: 10px;"></text-select>
        </div>
    </div>
</template>

<script setup>
import textSelect from '@/components/select/textSelect.vue';
import { ref } from 'vue';

const items = [
    '选项一',
    '选项二',
    '选项三',
    '选项四',
    '选项五',
    '选项六',
    '选项七',
    '选项八'
];
const item = ref('选项一');
</script>
<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">复选框</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>checkbox</span>
        </div>
        <div class="line">
            <two-checkbox text="复选框" v-model:value="checkboxValue"
                @change="(v) => { console.log('点击了复选框', checkboxValue); }"></two-checkbox>
        </div>
    </div>
</template>

<script setup>
import twoCheckbox from '@/components/checkbox/twoCheckbox.vue';
import { ref } from 'vue';

const checkboxValue = ref(false);
</script>

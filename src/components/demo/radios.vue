<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">单选框</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>radio</span>
        </div>
        <div class="line">
            <radio :group="['选项一', '选项二', '选项三', '选项四']" v-model:value="value" @change="(i) => { console.log(i); console.log(value); }">
            </radio>
        </div>
        <div class="line">
            <navigation-radio :group="['选项一', '选项二', '选项三', '选项四']" v-model:value="value"  @change="(i) => { console.log(i); }"></navigation-radio>
        </div>
    </div>
</template>

<script setup>
import radio from '@/components/radio/radio.vue';
import navigationRadio from '@/components/radio/navigationRadio.vue';
import { ref } from 'vue';

const value = ref('选项一');
</script>

<style>

</style>
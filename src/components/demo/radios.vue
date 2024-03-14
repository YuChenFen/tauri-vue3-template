<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">单选框</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>radio</span>
        </div>
        <div class="line">
            <radio :group="['选项一', '选项二', '选项三', '选项四']" defaultRadio="选项一" @change="(i) => { console.log(i); }">
            </radio>
        </div>
    </div>
</template>

<script setup>
import radio from '@/components/radio/radio.vue';
</script>

<style>

</style>
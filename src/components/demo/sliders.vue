<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">滑动条</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>slider</span>
        </div>
        <div class="line">
            <slider v-model:value="sliderValue" :width="300" :show-tips="true"
                @change="(v) => { console.log(sliderValue); }"></slider>
        </div>
    </div>
</template>

<script setup>
import slider from '@/components/slider/slider.vue';
import { ref } from 'vue';

const sliderValue = ref(0);
</script>

<style></style>
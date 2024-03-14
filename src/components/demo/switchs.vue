<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">开关</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>switch</span>
        </div>
        <div class="line">
            <toggle-switch v-model:value="switchValue" @change="(e) => { console.log(switchValue); }"></toggle-switch>
        </div>
    </div>
</template>

<script setup>
import toggleSwitch from '@/components/switch/toggleSwitch.vue';
import { ref } from 'vue';

const switchValue = ref(false);
</script>

<style></style>
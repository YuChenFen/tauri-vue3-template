<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">卡片</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>card</span>
        </div>
        <div class="line">
            <code-card>
                <template #main>
                    <p style="color: rgb(var(--text-color));padding: 10px;">this is a card</p>
                </template>
            </code-card>
        </div>
    </div>
</template>

<script setup>
import codeCard from '../card/codeCard.vue';
</script>

<style>

</style>
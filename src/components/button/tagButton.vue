<template>
    <div class="tag-button" :class="{'select': select}" @click.stop="click">
        <div class="content">
            <slot></slot>
            <p>{{ text }}</p>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue';

const props = defineProps({
    // 按钮文本
    text: {
        type: String,
        required: true,
        default: ''
    },
    // 值
    select: {
        type: Boolean,
        required: true,
        default: false
    }
})

const emits = defineEmits(['update:select', 'change']);
const select = ref(props.select);
function click(e) {
    select.value = !select.value;
    emits('update:select', select.value);
    emits('change', select.value);
}
</script>

<style scoped>
.tag-button {
    display: inline-block;
    padding: 6px 10px;
    border: 1px solid rgba(var(--app-color), .1);
    border-radius: 100vh;
    background-color: var(--app-background-color);
    color: rgb(var(--app-color));
    cursor: pointer;
}

.tag-button .content {
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    font-size: 14px;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
}

.tag-button p {
    font-size: 14px;
    text-overflow: ellipsis;
    white-space: nowrap;
}
.select{
    background-color: var(--app-highlight-background-color);
    color: var(--app-highlight-text-color);
}
</style>
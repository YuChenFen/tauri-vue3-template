<template>
    <button class="text-button" @click.stop="click">
        <p>{{ text }}</p>
    </button>
</template>

<script setup>
const props = defineProps({
    // 按钮文本
    text:{
        type: String,
        required: true,
        default: ''
    },
    // 是否禁用
    disabled: {
        type: Boolean,
        required: false,
        default: false
    }
})

const emits = defineEmits(['click']);

function click(e) {
    if(!props.disabled){
        emits('click', e);
    }
}
</script>

<style scoped>
.text-button {
    display: inline-block;
    padding: 6px 10px;
    border: 1px solid rgba(var(--app-color), .1);
    box-shadow: var(--app-shadow);
    border-radius: 5px;
    background-color: var(--app-background-color);
    color: rgb(var(--app-color));
    cursor: pointer;
}
.text-button:hover {
    background-color: var(--app-hover-background-color);
}
.text-button:active {
    /* background-color: var(--app-background-color); */
    color: var(--app-hover-color);
    box-shadow: none;
}
.text-button p{
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    font-size: 14px;
    text-overflow: ellipsis;
    white-space: nowrap;
}
</style>
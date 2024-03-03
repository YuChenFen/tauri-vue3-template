<template>
    <div class="text-button" @click.stop="click">
        <p>{{ text }}</p>
    </div>
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
    border: 1px solid rgba(var(--text-button-color), .1);
    box-shadow: var(--text-button-shadow);
    border-radius: 5px;
    background-color: var(--text-button-background-color);
    color: rgb(var(--text-button-color));
    cursor: pointer;
}
.text-button:hover {
    background-color: var(--text-button-hover-background-color);
}
.text-button:active {
    /* background-color: var(--text-button-background-color); */
    color: var(--text-button-hover-color);
    box-shadow: none;
}
.text-button p{
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    font-size: 14px;
}
</style>
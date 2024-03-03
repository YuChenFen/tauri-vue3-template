<template>
    <div class="text-button" @click.stop="click">
        <div class="icon">
            <slot>
            </slot>
        </div>
    </div>
</template>

<script setup>
const props = defineProps({
    // 是否禁用
    disabled: {
        type: Boolean,
        required: false,
        default: false
    }
})

const emits = defineEmits(['click']);

function click(e) {
    if (!props.disabled) {
        emits('click', e);
    }
}
</script>

<style scoped>
.text-button {
    display: inline-block;
    padding: 8px;
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

.text-button .icon {
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
}
</style>
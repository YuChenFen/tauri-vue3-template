<template>
    <div class="icon-button" @click.stop="click">
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
.icon-button {
    display: inline-block;
    padding: 6px;
    border: 1px solid rgba(var(--app-color), .1);
    box-shadow: var(--app-shadow);
    border-radius: 5px;
    background-color: var(--app-background-color);
    color: rgb(var(--app-color));
    cursor: pointer;
}

.icon-button:hover {
    background-color: var(--app-hover-background-color);
}

.icon-button:active {
    /* background-color: var(--app-background-color); */
    color: var(--app-hover-color);
    box-shadow: none;
}

.icon-button .icon {
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
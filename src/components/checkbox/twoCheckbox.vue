<template>
    <div class="two-checkbox">
        <label class="custom-checkbox">
            <input name="dummy" type="checkbox" @click.stop="change($event.target.checked)" :checked="value">
            <span class="checkmark"></span>
        </label>
        <p>
            {{ text }}
        </p>
    </div>
</template>

<script setup>
const props = defineProps({
    value:{
        type: Boolean,
        required: true,
        default: false
    },
    // 按钮文本
    text: {
        type: String,
        required: true,
        default: ''
    }
})
const emits = defineEmits(['update:value','change'])
const change = (e) => {
    emits('update:value', e);
    emits('change', e);
}
</script>

<style scoped>
.two-checkbox {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
}

.two-checkbox p {
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    font-size: 14px;
    color: rgb(var(--app-color));
}

.custom-checkbox {
    display: inline-flex;
    align-items: center;
    cursor: pointer;
    user-select: none;
    font-size: 16px;
    color: #333;
    transition: color 0.3s;
}

.custom-checkbox input[type="checkbox"] {
    display: none;
}

.custom-checkbox .checkmark {
    width: 18px;
    height: 18px;
    border: 1.5px solid rgb(var(--app-color));
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.3s, border-color 0.3s, transform 0.3s;
}

.custom-checkbox input[type="checkbox"]:checked+.checkmark {
    background-color: var(--app-highlight-background-color);
    border-width: 0.5px;
    transform: scale(1.1) rotateZ(360deg) rotateY(360deg);
}

.custom-checkbox .checkmark::before {
    content: "✔";
    font-size: 12px;
    color: transparent;
    transition: color 0.3s, transform 0.3s;
}
.custom-checkbox input[type="checkbox"]:checked + .checkmark::before {
    color: var(--app-background-color);
}
</style>
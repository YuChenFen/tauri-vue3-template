<template>
    <div style="position: relative;">
        <input type="range" :value="value" @change="change($event)" @input="($event) => value = $event.target.value"
            :style="`background-size: ${value}%;width: ${width}px;`" />
        <span class="tips" :style="`left: ${getTipsLeft(value)}px`" v-if="showTips">
            <p>{{ value }}</p>
        </span>
    </div>
</template>

<script setup>
import { ref } from 'vue';

const props = defineProps({
    value: {
        type: Number,
        default: 0
    },
    width: {
        type: Number,
        default: 130
    },
    showTips: {
        type: Boolean,
        default: false
    }
});
const value = ref(props.value);
const emits = defineEmits(['update:value', 'change']);
function change(e) {
    emits('update:value', e.target.value);
    emits('change', e.target.value);
}
function getTipsLeft(value) {
    return ((props.width - 18) * value / 100) - 9;
}
</script>

<style scoped>
input[type=range] {
    -webkit-appearance: none;
    height: 4px;
    background: -webkit-linear-gradient(var(--app-highlight-background-color), var(--app-highlight-background-color)) no-repeat, #949494;
    border-radius: 5px;
    box-shadow: var(--app-shadow);

}

input[type=range]::-webkit-slider-thumb {
    -webkit-appearance: none;
}

input[type=range]:focus {
    outline: none;
}

input[type=range]::-webkit-slider-thumb {
    -webkit-appearance: none;
    height: 18px;
    width: 18px;
    background-color: var(--app-highlight-background-color);
    border: 5px solid var(--app-background-color);
    border-radius: 50%;
    box-shadow: var(--app-shadow);
    cursor: pointer;
}

input[type=range]::-webkit-slider-thumb:hover {
    border-width: 4px;
}

input[type=range]::-webkit-slider-thumb:active {
    border-width: 6px;
}

.tips {
    position: absolute;
    left: 0px;
    top: -30px;
    box-shadow: var(--app-shadow);
    border-radius: 3px;
    color: rgb(var(--app-color));
    width: 36px;
    height: 25px;
    transition: opacity 0.3s;
    opacity: 0;
}
.tips p{
    position: absolute;
    width: 100%;
    height: 100%;
    font-size: 14px;
    text-align: center;
    line-height: 25px;
    background-color: var(--app-background-color);
    border-radius: 3px;
    z-index: 1;
}
.tips::after {
    content: '';
    position: absolute;
    top: 19px;
    left: 13px;
    width: 10px;
    height: 10px;
    background-color: var(--app-background-color);
    transform: rotate(45deg);
    box-shadow: rgb(192 192 192 / 59%) 1px 1px 2px 0px;
    z-index: 0;
}

input[type=range]:hover~.tips {
    opacity: 1;
}
</style>
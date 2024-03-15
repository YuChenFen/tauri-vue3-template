<template>
    <div class="container">
        <div class="tabs">
            <template v-for="item in group" :key="item">
                <input type="radio" :name="inputName" :id="item" :checked="item === value" @change="change(item)"
                    :value="item">
                <label class="tab" :for="item">{{ item }}</label>
            </template>
            <span :style="{ left: `${left}px` }" class="glider"></span>
        </div>
    </div>
</template>

<script setup>
import { ref, watch } from 'vue';
const props = defineProps({
    group: {
        type: Array,
        required: false,
        default: () => []
    },
    value: {
        type: String,
        required: false,
        default: ''
    }
})
const emits = defineEmits(['update:value', 'change']);

const inputName = Number(Math.random().toString().substr(3, 10) + Date.now()).toString(36)
const left = ref(2);
const width = 105;

watch(() => props.value, (e) => {
    let index = props.group.indexOf(e);
    left.value = 2 + index * width;
})
const change = (item) => {
    emits('update:value', item);
    emits('change', item);
}
</script>

<style scoped>
.tabs {
    display: flex;
    position: relative;
    background-color: rgba(var(--text-color), 0.05);
    color: rgb(var(--app-color));
    padding: 2px;
    border-radius: 5px;
    border: 1px solid #cfcfcf94;
    gap: 5px;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
}

.tabs * {
    z-index: 2;
}

.container input[type="radio"] {
    display: none;
}

.tab {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 30px;
    width: 100px;
    font-size: .8rem;
    font-weight: 500;
    border-radius: 5px;
    cursor: pointer;
    transition: color 0.15s ease-in;
}

.tab:hover {
    background-color: rgba(var(--app-color), 0.05);
}

.glider {
    position: absolute;
    display: flex;
    height: 30px;
    width: 100px;
    background-color: var(--app-highlight-text-color);
    z-index: 1;
    border-radius: 5px;
    transition: 0.1s ease-out;
}

.glider::after {
    content: "";
    position: absolute;
    bottom: 0;
    left: 42%;
    width: 16%;
    height: 3px;
    background-color: var(--app-highlight-background-color);
    border-radius: 9999px;
}
</style>
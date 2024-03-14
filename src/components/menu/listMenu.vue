<template>
    <div class="list">
        <div v-for="item in options" :key="item[optionKey]" class="item"
            :class="{ 'highlight-item': item[optionKey] === highlightItem }" @click="click(item)">
            <component :is="item.component"></component>
        </div>
    </div>
</template>

<script setup>
const props = defineProps({
    highlightItem: {
        default: ''
    },
    options: {
        type: Array,
        default: () => []
    },
    // 选项键
    optionKey: {
        type: String,
        required: true,
        default: ''
    },
})

const emits = defineEmits(['update:highlightItem', 'change'])
const click = (item) => {
    if (item[props.optionKey] === props.highlightItem) {
        return;
    }
    emits('update:highlightItem', item[props.optionKey]);
    emits('change', item);
}

</script>

<style scoped>
.list {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 3px;
}

.item {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex-direction: column;
    position: relative;
    border-radius: 5px;
    cursor: pointer;
}

.item:hover {
    background-color: rgba(var(--text-color), 0.05);
}

.highlight-item {
    background-color: rgba(var(--text-color), 0.05);
}

.highlight-item::before {
    content: '';
    width: 3px;
    height: 50%;
    border-radius: 9999px;
    top: 25%;
    position: absolute;
    left: 0;
    background-color: var(--app-highlight-background-color);
}
</style>
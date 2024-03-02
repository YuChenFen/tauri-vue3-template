<template>
    <svg v-if="isName" :class="svgClass" v-bind="$attrs" :style="{ color: color }">
        <use :xlink:href="iconName"></use>
    </svg>
    <div v-else v-html="name" class="svg-no-name"></div>
</template>

<script setup>
const props = defineProps({
    name: {
        type: String,
        required: true
    },
    color: {
        type: String,
        default: ''
    }
})
let isName = true;
if (/svg/.test(props.name)) {
    isName = false;
}
let iconName = '';
let svgClass = '';
if (isName) {
    iconName = `#icon-${props.name}`;
    svgClass = `svg-icon icon-${props.name}`;
}
// const iconName = computed(() => `#icon-${props.name}`);
// const svgClass = computed(() => {
//     if (props.name) return `svg-icon icon-${props.name}`
//     return 'svg-icon'
// });
</script>

<style scoped>
.svg-icon {
    width: 1em;
    height: 1em;
    fill: currentColor;
    vertical-align: middle;
}

.svg-no-name :deep(svg) {
    width: 100%;
    height: 100%;
}
</style>

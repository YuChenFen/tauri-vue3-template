<template>
    <div ref="slide">
        <slot></slot>
    </div>
</template>

<script setup>
import { onBeforeUnmount, onMounted, ref } from 'vue';
const slide = ref(null);
const observer = new MutationObserver((e) => { play(); });
let first, last;
function animation() {
    let x = first.x - last.x;
    let y = first.y - last.y;
    let sw = first.width / last.width;
    let sh = first.height / last.height;
    slide.value.animate([
        {
            transform: `translate(${x}px, ${y}px) scale(${sw}, ${sh})`
        },
        {
            transform: ``
        }
    ], {
        duration: 500,
        easing: 'cubic-bezier(0.165, 0.84, 0.44, 1)'
    })
}

const play = () => {
    if (slide.value) {
        slide.value.style.position = 'relative';
        slide.value.style.top = '-80px';
        last = slide.value.getBoundingClientRect();
        slide.value.style.top = '0';
        first = slide.value.getBoundingClientRect();
        animation();
    }
}
onMounted(() => {
    play();
    observer.observe(slide.value, {
        attributes: false,
        childList: true,
        subtree: false
    });
})

onBeforeUnmount(() => { observer.disconnect(); })
</script>

<style scoped></style>
<template>
    <div>
        <input class="switch" type="checkbox" :checked="value" @change="change">
    </div>
</template>

<script setup>
const props = defineProps({
    value: {
        type: Boolean,
        default: false
    }
})

const emit = defineEmits(['update:value', 'change'])
function change(e) {
    emit('update:value', e.target.checked)
    emit('change', e.target.checked)
}

</script>

<style scoped>
.switch {
    position: relative;
    height: 22px;
    width: 44px;
    cursor: pointer;
    appearance: none;
    -webkit-appearance: none;
    border-radius: 999px;
    background-color: rgba(100, 116, 139, 0.377);
    transition: all .3s ease;
}

.switch:checked {
    background-color: var(--app-highlight-background-color);
}

.switch::before {
    position: absolute;
    content: "";
    left: 5px;
    top: 5px;
    display: block;
    height: 12px;
    width: 12px;
    cursor: pointer;
    border-radius: 9999px;
    background-color: var(--app-highlight-text-color);
    transition: all .3s ease;
}

.switch:hover::before {
    box-shadow: 0 0 0px 2px rgba(0, 0, 0, .25)
}

.switch:checked:before {
    left: 26px;
    border-color: rgba(236, 72, 153, 1);
}
</style>
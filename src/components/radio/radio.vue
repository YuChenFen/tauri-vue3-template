<template>
    <div class="radio-group">
        <label class="radio-button" v-for="item in group" :key="item">
            <input type="radio" :name="inputName" :value="item" :checked="defaultRadio == item"
                @change="change(item)" />
            <span class="radio">
                <span class="radio-inner"></span>
            </span>
            <p>{{ item }}</p>
        </label>
    </div>

</template>

<script setup>
const props = defineProps({
    group: {
        type: Array,
        required: false,
        default: () => []
    },
    defaultRadio: {
        type: String,
        required: false,
        default: ''
    }
})
const inputName = Number(Math.random().toString().substr(3, 10) + Date.now()).toString(36)
const emits = defineEmits(['change'])

function change(item) {
    emits('change', item)
}

</script>

<style scoped>
.radio-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    color: rgb(var(--app-color));
    font-size: 14px;
}

.radio-button p {
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
}

.radio-button {
    display: flex;
    gap: 10px;
    justify-content: center;
    margin: 8px 5px;
    position: relative;
    align-items: center;
}

.radio-button input[type="radio"] {
    position: absolute;
    opacity: 0;
}

.radio {
    position: relative;
    width: 22px;
    height: 22px;
    border-radius: 50%;
    border: 2px solid var(--app-hover-color);
    box-shadow: var(--app-shadow);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    background-color: rgba(var(--text-color), .05);
    transition: all 0.3s ease-in-out;
}

.radio .radio-inner {
    transform-origin: center;
    position: relative;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background-color: var(--app-highlight-text-color);
    opacity: 0;
}

.radio:hover {
    background-color: rgba(var(--text-color), .15);
}

.radio:hover .radio-inner {
    transform: scale(1.1);
}

.radio-button input[type="radio"]:checked+.radio {
    border: none;
    background-color: var(--app-highlight-background-color);
}

.radio-button input[type="radio"]:checked+.radio .radio-inner {
    opacity: 1;
}
</style>
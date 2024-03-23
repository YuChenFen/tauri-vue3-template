<template>
    <div class="code-card" :style="{ 'width': width, 'height': height }">
        <header>
            <div class="header-right">
                <div style="background-color: #ff6057;" @click.stop=""></div>
                <div style="background-color: #ffbd2f;" @click.stop=""></div>
                <div style="background-color: #28c93f;" @click.stop=""></div>
            </div>
            <slot name="header-title"></slot>
        </header>
        <main class="main">
            <slot name="main"></slot>
        </main>
    </div>
</template>

<script setup>
const props = defineProps({
    width: {
        type: String,
        default: '100%'
    },
    height: {
        type: String,
        default: '100%'
    }
})

</script>

<style scoped>
.code-card {
    width: 50%;
    height: max-content;
    min-width: 76px;
    min-height: 35px;
    background-color: rgb(var(--theme-color));
    overflow: hidden;
    resize: both;
    border-radius: 5px;
    border: solid #dadada 1px;
    box-shadow: rgba(99, 99, 99, 0.2) 0px 2px 8px 0px;
    transition: background .8s, color .8s;
}

.code-card::-webkit-resizer {
    background: transparent;
}

header {
    width: 100%;
    height: 35px;
    border-bottom: solid #dadada 1px;
    background-color: var(--app-background-color);
    color: rgb(var(--app-color));
    display: flex;
    justify-content: space-between;
    align-items: center;
}

header .header-right {
    display: flex;
    height: 100%;
    align-items: center;
}

header .header-right div {
    width: 12px;
    height: 12px;
    margin-right: 10px;
    border-radius: 50%;
    cursor: pointer;
}

header .header-right div:nth-child(1) {
    margin-left: 10px;
}
</style>
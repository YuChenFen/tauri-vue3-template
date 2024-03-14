<template>
    <div class="vertical-menu-wrapper">
        <div class="vertical-menu" :style="{ width: width, 'background-color': barBackgroundColor }">
            <header>
                <slot name="header"></slot>
            </header>
            <main>
                <slot name="menu"></slot>
            </main>
            <footer>
                <slot name="footer"></slot>
            </footer>
        </div>
        <div class="content">
            <slot name="content"></slot>
        </div>
    </div>

</template>

<script setup>
const props = defineProps({
    width: {
        type: String,
        default: '50px'
    },
    barBackgroundColor: {
        type: String,
        default: "rgb(var(--theme-color))"
    }
})
</script>

<style scoped>
.vertical-menu-wrapper {
    position: relative;
    display: flex;
    height: 100%;
}

.vertical-menu {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
}
.vertical-menu header,
.vertical-menu main,
.vertical-menu footer {
    display: flex;
    flex-direction: column;
    width: 100%;
}


.vertical-menu main {
    flex: 1;
    overflow-y: scroll;
}


.vertical-menu main::-webkit-scrollbar {
    -webkit-appearance: none;
    display: none;
}

.vertical-menu-wrapper .content {
    flex: 1;
    height: 100%;
    overflow-y: scroll;
}
</style>
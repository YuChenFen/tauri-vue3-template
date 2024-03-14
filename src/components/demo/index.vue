<template>
    <div class="demo-page">
        <vertical-menu bar-background-color="">
            <template #header>
                <div style="padding: 15px;" class="icon">
                    <svg-icon name="menu"></svg-icon>
                </div>
                <listMenu :options="listMenuOptions1" optionKey="id" v-model:highlightItem="listMenuHighlightItem"
                    @change="changePage"></listMenu>
                <span
                    style="height: 1px;width: 100%;background-color: rgba(var(--text-color), 0.05);margin: 5px 0;"></span>
            </template>
            <template #menu>
                <listMenu :options="listMenuOptions2" optionKey="id" v-model:highlightItem="listMenuHighlightItem"
                    @change="changePage"></listMenu>
            </template>
            <template #footer>
                <listMenu :options="listMenuOptions3" optionKey="id" v-model:highlightItem="listMenuHighlightItem"
                    @change="changePage" style="margin-bottom: 8px;"></listMenu>
            </template>
            <template #content>
                <slide>
                    <component :is="curPage"></component>
                </slide>
            </template>
        </vertical-menu>
    </div>
</template>

<script setup>
import verticalMenu from '@/components/menu/verticalMenu.vue';
import listMenu from '@/components/menu/listMenu.vue';
import svgIcon from "@/assets/icons/svgIcon.vue";
import slide from "@/components/animation/slide.vue";
import home from './home.vue';
import setting from './setting.vue';
import icons from './icons.vue';
import buttons from './buttons.vue';
import checkboxs from './checkboxs.vue';
import selects from './selects.vue';
import radios from './radios.vue';
import sliders from './sliders.vue';
import switchs from './switchs.vue';
import messages from './messages.vue';
import tables from './tables.vue';
import inputs from './inputs.vue';
import menus from './menus.vue';
import { ref, h, shallowRef } from 'vue';

const curPage = shallowRef(home)
const changePage = (e) =>{
    switch(e.id){
        case 0:
            curPage.value = home
            break
        case 1:
            curPage.value = icons
            break
        case 2:
            curPage.value = buttons
            break
        case 3:
            curPage.value = checkboxs
            break
        case 4:
            curPage.value = selects
            break
        case 5:
            curPage.value = radios
            break
        case 6:
            curPage.value = sliders
            break
        case 7:
            curPage.value = switchs
            break
        case 8:
            curPage.value = messages
            break
        case 9:
            curPage.value = tables
            break
        case 10:
            curPage.value = inputs
            break
        case 11:
            curPage.value = menus
            break
        case 12:
            curPage.value = setting
    }
}

const listMenuHighlightItem = ref(0);
const iconStyle = {
    height: '18px', width: '18px', margin: '10px', fill: 'rgb(var(--app-color))'
}
const listMenuOptions1 = [
    {
        component: h(svgIcon, { name: 'home', style: iconStyle }, null),
        id: 0,
    },
    {
        component: h(svgIcon, { name: 'application', style: iconStyle }, null),
        id: 1,
    }
]
const listMenuOptions2 = [
    {
        component: h(svgIcon, { name: 'play', style: iconStyle }, null),
        id: 2,
    },
    {
        component: h(svgIcon, { name: 'checkbox', style: iconStyle }, null),
        id: 3,
    },
    {
        component: h(svgIcon, { name: 'flag', style: iconStyle }, null),
        id: 4,
    },
    {
        component: h(svgIcon, { name: 'radio', style: iconStyle }, null),
        id: 5,
    },
    {
        component: h(svgIcon, { name: 'slider', style: iconStyle }, null),
        id: 6,
    },
    {
        component: h(svgIcon, { name: 'switch', style: iconStyle }, null),
        id: 7,
    },
    {
        component: h(svgIcon, { name: 'message', style: iconStyle }, null),
        id: 8,
    },
    {
        component: h(svgIcon, { name: 'save', style: iconStyle }, null),
        id: 9,
    },
    {
        component: h(svgIcon, { name: 'edit', style: iconStyle }, null),
        id: 10,
    },
    {
        component: h(svgIcon, { name: 'menu', style: iconStyle }, null),
        id: 11,
    }
]
const listMenuOptions3 = [
    {
        component: h(svgIcon, { name: 'setting', style: iconStyle }, null),
        id: 12,
    },
]
</script>

<style scoped>
.demo-page {
    height: 100%;
}
.demo-page :deep(.line) {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
    padding: 10px;
    border-radius: 5px;
    margin: 10px;
    margin-left: 0px;
    background-color: rgb(var(--theme-color));
}

.icon {
    display: flex;
    align-items: center;
    justify-content: center;
}

.icon :deep(svg) {
    width: 18px;
    height: 18px;
    fill: rgb(var(--app-color));
    cursor: pointer;
    transition: all .3s;
}

.icon :deep(svg):hover {
    fill: var(--app-hover-color);
}
</style>
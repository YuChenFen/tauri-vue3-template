<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">菜单</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>menu</span>
        </div>
        <div style="margin-top: 10px;">
            <span style="font-weight: 500;color: rgb(var(--text-color));">下拉菜单</span>
        </div>
        <div class="line">
            <item-menu :options="iconItems" @change="(i) => { console.log(i); }" v-model:show="showItemMenu" option-height="35px"
                position="right">
                <icon-button @click="showItemMenu = true">
                    <div style="display: flex;gap: 10px;margin-right: 5px;">
                        <svg-icon style="margin: 2px;" name="mail"></svg-icon>
                        <p>邮件</p>
                    </div>
                </icon-button>
            </item-menu>
        </div>
        <div style="margin-top: 10px;">
            <span style="font-weight: 500;color: rgb(var(--text-color));">列表菜单</span>
        </div>
        <div class="line">
            <listMenu :options="listMenuOptions1" optionKey="id" v-model:highlightItem="listMenuHighlightItem"
                    @change="(e)=>{console.log(e);}"></listMenu>
        </div>
    </div>
</template>

<script setup>
import iconButton from '@/components/button/iconButton.vue';
import itemMenu from '@/components/menu/itemMenu.vue';
import listMenu from '@/components/menu/listMenu.vue';
import svgIcon from "@/assets/icons/svgIcon.vue";
import { ref, h } from 'vue';

const iconStyle = {
    height: '18px', width: '18px', margin: '10px', fill: 'rgb(var(--app-color))'
}
const iconItemsStyle = {
    height: '20px', width: '20px', margin: 'auto', fill: 'rgb(var(--app-color))'
}
const showItemMenu = ref(false);
const iconItems = [
    {
        icon: h(svgIcon, { name: 'send', style: iconItemsStyle }, null),
        text: '发送'
    },
    {
        icon: h(svgIcon, { name: 'eye', style: iconItemsStyle }, null),
        text: '查看',
        items: [
            {
                icon: h(svgIcon, { name: 'delete', style: iconItemsStyle }, null),
                text: '删除'
            },
            {
                icon: h(svgIcon, { name: 'save', style: iconItemsStyle }, null),
                text: '保存'
            },
        ]
    }
]

const listMenuOptions1 = [
    {
        component: h(svgIcon, { name: 'home', style: iconStyle }, null),
        id: 0,
    },
    {
        component: h(svgIcon, { name: 'cut', style: iconStyle }, null),
        id: 1,
    },
    {
        component: h(svgIcon, { name: 'paste', style: iconStyle }, null),
        id: 3,
    }
]
const listMenuHighlightItem = ref(0)
</script>

<template>
    <div>
        <div class="line">
            <text-button style="margin-left: 10px;" text="切换主题" @click="chuangeTheme"></text-button>
            <text-button text="按钮"></text-button>
            <icon-button>
                <svg-icon style="margin: 2px;" name="header-close"></svg-icon>
            </icon-button>
            <icon-button>
                <svg-icon name="header-close"></svg-icon>
                <p style="margin-left: 5px;">关闭</p>
            </icon-button>
            <tag-button text="标签" :select="select">
                <svg-icon name="tag"></svg-icon>
            </tag-button>
        </div>
        <div class="line">
            <two-checkbox text="复选框"></two-checkbox>
        </div>
        <div class="line">
            <text-select :options="items" :defaultSelected="item" :width="100" @change="(i) => item = i"
                style="margin-left: 10px;"></text-select>
            <select-button :options="iconItems" :icon="iconTag" text="打开" :width="103.6"
                @change="(i) => { console.log(i); }" style="margin-left: 10px;"></select-button>
        </div>
        <div class="line">
            <radio :group="['选项一', '选项二', '选项三', '选项四']" defaultRadio="选项一" @change="(i) => { console.log(i); }"></radio>
            <!-- <radio :group="[1,2,3,4,5]"></radio> -->
        </div>
    </div>
</template>

<script setup>
import textButton from '@/components/button/textButton.vue';
import iconButton from '@/components/button/iconButton.vue';
import tagButton from '@/components/button/tagButton.vue';
import twoCheckbox from '@/components/checkbox/twoCheckbox.vue';
import textSelect from './select/textSelect.vue';
import selectButton from './button/selectButton.vue';
import radio from './radio/radio.vue';
import { useThemeStore } from '@/stores/theme.js';
import svgIcon from "@/assets/icons/svgIcon.vue";
import { ref, h } from 'vue';
let select = ref(false);

const themeStore = useThemeStore();
let a = 0;
function chuangeTheme() {
    if (a) {
        themeStore.changeTheme('bright');
        a = 0;
    } else {
        themeStore.changeTheme('dark');
        a = 1;
    }
}

const items = [
    '选项一',
    '选项二',
    '选项三',
    '选项四',
    '选项五',
    '选项六',
    '选项七',
    '选项八'
];
const item = ref('选项一');
const iconTag = h(svgIcon, { name: 'tag' }, null)
const iconItems = [
    {
        icon: h(svgIcon, { name: 'header-close' }, null),
        text: '关闭'
    },
    {
        icon: h(svgIcon, { name: 'header-minimize' }, null),
        text: '最大化'
    }
]
const iconItem = ref(0);
</script>

<style scoped>
.line {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    border-radius: 5px;
    margin: 10px;
    background-color: rgb(var(--theme-color));
}
</style>
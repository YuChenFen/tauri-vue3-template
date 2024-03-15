<template>
    <div class="item-menu" :position="position">
        <slot></slot>
        <transition name="options-animation">
            <div v-show="show" class="options">
                <div v-if="options.length === 0" class="option" :style="{ height: optionHeight }">
                    <div class="option-label">
                        NOT DATA
                    </div>
                </div>
                <div v-for="option in  options " :key="optionKey ? option[optionKey] : option">
                    <template v-if="option.items">
                        <item-menu v-model:show="option.showItems.value" :options="option.items"
                            :option-height="optionHeight" :option-key="optionKey" position="child" @change="change"
                            :style="{ height: optionHeight }">
                            <div class="option" @mouseenter="showChild(option.showItems)" :style="{ height: optionHeight }">
                                <component :is="option.icon ? option.icon : 'p'"></component>
                                <div class="option-label">
                                    {{ option.text }}
                                </div>
                                <svg width="12" height="12" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 6.91 12.75" fill="rgb(var(--app-color))">
                                    <path class="cls-1"
                                        d="M.37,12.75c-.1,0-.19-.04-.27-.11-.15-.15-.15-.38,0-.53L5.84,6.38,.11,.64C-.04,.49-.04,.26,.11,.11,.26-.04,.49-.04,.64,.11L6.91,6.38,.64,12.64c-.07,.07-.17,.11-.27,.11Z" />
                                </svg>
                            </div>
                        </item-menu>
                    </template>
                    <div v-else class="option" @click.stop="change(option)" @mouseenter="closeChild" :style="{ height: optionHeight }">
                        <component :is="option.icon ? option.icon : 'p'"></component>
                        <div class="option-label">
                            {{ option.text }}
                        </div>
                    </div>
                </div>
            </div>
        </transition>
    </div>
</template>

<script setup>
import { ref, watch } from 'vue';

const props = defineProps({
    // 是否显示
    show: {
        type: Boolean,
        required: true
    },
    // 选项列表
    options: {
        type: Array,
        default: () => ([])
    },
    // 选项的高度
    optionHeight: {
        type: String,
        default: 'max-content'
    },
    // 选项键
    optionKey: {
        type: String,
        default: ''
    },
    // 位置
    position: {
        type: String,
        default: 'bottom'
    }
})

const emits = defineEmits(['update:show', 'change'])
props.options.forEach(element => {
    if (element.items) {
        element.showItems = ref(false);
    }
});
watch(() => props.show, (val) => {
    if (val) {
        // 如果点击就关闭 addEventListener 事件 函数 捕获阶段
        document.addEventListener('click', closeOptions, true);
        // 如果右键其他地方关闭这次的上下文菜单栏
        document.addEventListener('contextmenu', closeOptions, true);

    } else {
        document.removeEventListener('click', closeOptions);
        document.removeEventListener('contextmenu', closeOptions);
    }
})
const closeOptions = () => {
    emits('update:show', false);
}
const change = (option) => {
    emits('change', option);
}

let currentChildShowItems = void 0;
const showChild = (showItems) => {
    showItems.value = true
    currentChildShowItems = showItems;
}
const closeChild = () => {
    if (currentChildShowItems) {
        currentChildShowItems.value = false
    }
}
</script>

<style scoped>
.item-menu {
    position: relative;
    display: flex;
    align-items: center;
}

.item-menu[position="right"] .options {
    left: 100%;
}

.item-menu[position="left"] .options {
    right: 100%;
}

.item-menu[position="top"] {
    flex-direction: column-reverse;
}

.item-menu[position="top"] .options {
    bottom: 100%;
}

.item-menu[position="bottom"] {
    flex-direction: column;
}

.item-menu[position="bottom"] .options {
    top: 100%;
}

.item-menu[position="child"] .options {
    top: -10px;
    left: 100%;
}

.options {
    position: absolute;
    height: max-content;
    width: max-content;
    margin: 5px;
    padding: 6px 5px 4px 5px;
    border-radius: 4px;
    background-color: var(--app-background-color);
    -webkit-box-shadow: var(--app-shadow);
    box-shadow: var(--app-shadow);
    border: 1px solid rgba(var(--app-color), .1);
    z-index: 1;
    width: max-content;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    color: rgb(var(--app-color));
}

.options .option {
    flex: 1 1 0%;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: flex;
    align-items: center;
    gap: 10px;
    line-height: 20px;
    padding: 2px 10px;
    border-radius: 4px;
    transition: all .5s;
}

.option:hover {
    background-color: var(--app-hover-background-color);
}

.option-label {
    height: 100%;
    width: 100%;
    display: flex;
    align-items: center;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.options .option-label {
    font-size: 14px;
}

.options-animation-enter-active {
    transition: all .3s cubic-bezier(0.42, 0, 0.33, 1);
}

.options-animation-enter-from,
.options-animation-leave-to {
    transform: translateY(-20px);
    opacity: 0;
}
</style>
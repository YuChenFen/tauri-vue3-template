<template>
    <div :style="{ 'font-size': fontSize, height: `${optionHeight}px`, width: `${width}px` }">
        <div class="select" @click.stop="show" :style="{ width: width ? '100%' : 'max-content' }">
            <div class="select-button" style="flex: 1;overflow: hidden;text-overflow: ellipsis;white-space: nowrap;">
                <component :is="Object.keys(icon).length ? icon : 'p'"></component>
                {{ text }}
            </div>
            <span class="down">
                <svg ref="svgRef" t="1700911295607" class="icon" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="2416">
                    <path
                        d="M763.904 407.552l-209.92 264.192c-16.896 21.504-49.664 21.504-67.072 0L276.48 407.552c-22.016-28.16-2.048-69.632 33.792-69.632h420.352c35.84 0 55.808 41.472 33.28 69.632z"
                        fill="#666666" p-id="2417"></path>
                </svg>
            </span>
        </div>
        <transition name="options-animation">
            <div v-show="isShowOptions" class="options">
                <div v-if="options.length === 0" class="option" :style="{ 'height': `${optionHeight}px` }">
                    <div class="option-label">
                        NOT DATA
                    </div>
                </div>
                <div v-for="option in options" :key="option" class="option" @click.stop="change(option)"
                    :style="{ 'height': `${optionHeight}px` }">
                    <component :is="Object.keys(option.icon).length ? option.icon : 'p'"></component>
                    <div class="option-label">
                        {{ option.text }}
                    </div>
                </div>
            </div>
        </transition>
    </div>
</template>

<script setup>
import { h, ref, watch } from 'vue';

const props = defineProps({
    // 全部选项
    options: {
        type: Array,
        required: false,
        default: []
    },
    // 默认选中的选项
    icon: {
        type: Object,
        required: false,
        default: {}
    },
    // 按钮文本
    text: {
        type: String,
        required: false,
        default: ''
    },
    // 选项的宽度
    width: {
        type: Number,
        required: false,
        default: 0
    },
    // 选项的高度
    optionHeight: {
        type: Number,
        required: false,
        default: 30
    },
    // 选项的字体大小
    fontSize: {
        type: String,
        required: false,
        default: '14px'
    }
})
const emits = defineEmits(['change'])
const isShowOptions = ref(false);
const svgRef = ref(null);
const closeOptions = () => {
    isShowOptions.value = false;
}
watch(isShowOptions, (val) => {
    if(val){
        if(svgRef.value){
            svgRef.value.style.transform = 'rotate(180deg)';
        }
        // 如果点击就关闭 addEventListener 事件 函数 捕获阶段
        document.addEventListener('click', closeOptions, true);
        // 如果右键其他地方关闭这次的上下文菜单栏
        document.addEventListener('contextmenu', closeOptions, true);

    }else{
        if(svgRef.value){
            svgRef.value.style.transform = 'rotate(0deg)';
        }
        document.removeEventListener('click', closeOptions);
        document.removeEventListener('contextmenu', closeOptions);
    }
})

function show(e) {
    e.stopPropagation();
    isShowOptions.value = !isShowOptions.value;
}


function change(option) {
    closeOptions()
    emits('change', option)
}
</script>

<style scoped>
.select {
    height: 100%;
    /* width: max-content; */
    text-overflow: ellipsis;
    user-select: none;
    padding-right: 40px;
    display: inline-block;
    padding: 0 10px;
    background-color: var(--app-background-color);
    border: 1px solid rgba(var(--app-color), .1);
    border-radius: 4px;
    -webkit-box-shadow: var(--app-shadow);
    box-shadow: var(--app-shadow);
    display: flex;
    cursor: pointer;
    align-items: center;
    z-index: 100;
}
.select:hover{
    background-color: var(--app-background-color-hover);
}
.select:active{
    color: var(--app-hover-color);
    box-shadow: none;
}
.select-button,
.option {
    flex: 1 1 0%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: flex;
    align-items: center;
    gap: 10px;

}
.select,
.options {
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    font-size: 14px;
    color: rgb(var(--app-color));
}

.down {
    width: 15px;
    height: 15px;
    display: flex;
    align-items: center;
}

.down svg {
    transition: all .5s;
}

.options {
    margin-top: 5px;
    padding: 4px;
    border-radius: 4px;
    background-color: var(--app-background-color);
    -webkit-box-shadow: var(--app-shadow);
    box-shadow: var(--app-shadow);
    border: 1px solid rgba(var(--app-color), .1);
    overflow: hidden;
    /* height: 0; */
    position: relative;
    z-index: 1;
    width: max-content;
}

.option {
    line-height: 20px;
    padding: 2px 10px;
    margin: 4px;
    border-radius: 4px;
    position: relative;
    overflow: hidden;
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

.options-animation-enter-active {
    transition: all .3s cubic-bezier(0.42, 0, 0.33, 1);
}

.options-animation-enter-from,
.options-animation-leave-to {
    transform: translateY(-20px);
    opacity: 0;
}
</style>
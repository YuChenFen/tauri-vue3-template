<template>
    <div ref="messageRef" class="message" :type="type">
        <div style="width: 30px;height: 45px;" class="icon" v-html="getIcon(type)">
        </div>
        <div class="content" :class="{ 'brief': isBrief }" :style="{ 'display': isBrief ? 'flex' : 'block' }">
            <div class="title no-select">{{ title }}</div>
            <p>{{ text }}</p>
        </div>
        <div>
            <div class="close no-select" @click="beforeClose">×</div>
        </div>
    </div>
</template>

<script setup>
import getIcon from './icon'
import { ref } from 'vue';

const poprs = defineProps({
    title: {
        type: String,
        default: ''
    },
    type: {
        type: String,
        default: 'info'
    },
    text: {
        type: String,
        default: ''
    },
    isBrief: {
        type: Boolean,
        default: true
    },
    close: {
        type: Function,
        default: () => { }
    }
})
let messageRef = ref(null)

function beforeClose() {
    poprs.close();
}
</script>

<style scoped>
.message {
    position: relative;
    width: fit-content;
    /* min-height: 40px; */
    border: 1px solid #e5e5e5;
    border-radius: 5px;
    background-color: var(--app-background-color);
    color: rgb(var(--app-color));
    display: flex;
    /* margin: 5px; */
}

.content {
    flex: 1;
    align-items: center;
}

.content .title {
    padding: 5px 10px;
    height: 30px;
    line-height: 35px;
    font-size: 14px;
    font-weight: 800;
    text-overflow: ellipsis;
    white-space: nowrap;
}
.content p {
    padding: 5px 10px;
    font-size: 14px;
}
.brief .title{
    height: 45px!important;
    line-height: 35px!important;
}
.brief p{
    display: inline-block;
    white-space: nowrap;
}
.close {
    cursor: pointer;
    height: 25px;
    width: 25px;
    font-size: 24px;
    font-weight: 500;
    border-radius: 5px;
    text-align: center;
    line-height: 25px;
    margin: 10px;
}

.no-select {
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
}

.close:hover {
    background-color: #c4c4c43c;
}

.message[type='success'] {
    background-color: #dff6ddd4;
}
.message[type='warning'] {
    background-color: #fff4ced4;
}
.message[type='error'] {
    background-color: #fde7e9d4;
}
.message[type='info'] {
    background-color: #cff4fcd4;
}
.icon :deep(svg){
    width: 15px;
    height: 20px;
    margin: 12.5px 3px 12.5px 12px;
}
</style>
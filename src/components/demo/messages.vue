<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">信息提示</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>message</span>
        </div>
        <div style="margin-top: 10px;">
            <span style="font-weight: 500;color: rgb(var(--text-color));">通知 - message</span>
        </div>
        <div class="line">
            <text-button text="成功" @click="showMessage('success', '成功', '成功啦   (^&^)/')"></text-button>
            <text-button text="警告" @click="showMessage('warning', '警告', '警告！  （～￣▽￣～）')"></text-button>
            <text-button text="错误" @click="showMessage('error', '错误', '发生错误了！！！  w(ﾟДﾟ)w')"></text-button>
            <text-button text="提示" @click="showMessage('info', '提示', '这是一个消息   ( つ•̀ω•́)つ')"></text-button>
        </div>
        <div style="margin-top: 10px;color: rgb(var(--text-color));">
            <span style="font-weight: 500;">对话框 - dialog</span>
        </div>
        <div class="line">
            <text-button text="打开对话框" @click="isOpenDialog = true"></text-button>
            <customize-dialog v-model:open="isOpenDialog" @ok="() => { console.log('点击了确定按钮'); }">
                <template #body>
                    <div class="user-select" style="padding: 20px;">
                        <header style="font-size: 20px;font-weight: 500;margin-bottom: 10px;">
                            这是一个对话框
                        </header>
                        <p>云朵飘过天空，风儿轻轻吹拂，却引不起心中的一丝波澜。我在时间的河流里漂流，像一片落叶，不知道会飘向何方。月亮挂在夜空，星辰闪烁，但我的心却像一片荒芜的沙漠，没有生机，没有希望。我在寻找着某种东西，却又不知道那到底是什么，只是盲目地前行，不知道何时会停下脚步。这一切都是那么的无意义，却又让我感到无尽的迷茫和困惑。
                        </p>
                    </div>
                </template>
            </customize-dialog>
        </div>
    </div>
</template>

<script setup>
import textButton from '@/components/button/textButton.vue';
import Message from '@/components/notification/src/message';
import customizeDialog from '@/components/dialog/customizeDialog.vue';
import { ref } from 'vue';

const showMessage = (type, title, text) => {
    Message({
        title: title,
        type: type,
        text: text,
        // isBrief: false,
        duration: 5000
    })
}


const isOpenDialog = ref(false);
</script>

<style>

</style>
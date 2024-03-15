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
            <customize-dialog v-model:open="isOpenDialog" @ok="() => { console.log('点击了确定按钮'); }" @no="() => { console.log('点击了取消按钮'); }">
                <template #body>
                    <div class="user-select" style="padding: 20px;">
                        <header style="font-size: 20px;font-weight: 500;margin-bottom: 10px;">
                            这是一个对话框
                        </header>
                        <p>
                            人永远不会知道，谁哪次不经意的跟你说了再见之后，就真的不会见了。
                        </p>
                        <p style="text-align: right;">
                            ——《千与千寻》
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
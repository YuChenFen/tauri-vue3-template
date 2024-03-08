<template>
    <div>
        <div class="line">
            <text-button text="切换主题" @click="chuangeTheme"></text-button>
            <text-button text="按钮" @click="() => { console.log('点击了按钮'); }"></text-button>
            <icon-button @click="() => { console.log('点击了按钮'); }">
                <svg-icon style="margin: 2px;" name="header-close"></svg-icon>
            </icon-button>
            <icon-button @click="() => { console.log('点击了按钮'); }">
                <svg-icon name="header-close"></svg-icon>
                <p style="margin-left: 5px;text-overflow: ellipsis;white-space: nowrap;">关闭</p>
            </icon-button>
            <tag-button text="标签" v-model:select="select" @change="(v) => { console.log('点击了标签', select); }">
                <svg-icon name="tag"></svg-icon>
            </tag-button>
        </div>
        <div class="line">
            <two-checkbox text="复选框" v-model:value="checkboxValue"
                @change="(v) => { console.log('点击了复选框', checkboxValue); }"></two-checkbox>
        </div>
        <div class="line">
            <text-select :options="items" :defaultSelected="item" :width="100" @change="(i) => item = i"
                style="margin-left: 10px;"></text-select>
            <select-button :options="iconItems" :icon="iconTag" text="打开" :width="103.6"
                @change="(i) => { console.log(i); }" style="margin-left: 10px;"></select-button>
        </div>
        <div class="line">
            <radio :group="['选项一', '选项二', '选项三', '选项四']" defaultRadio="选项一" @change="(i) => { console.log(i); }">
            </radio>
            <!-- <radio :group="[1,2,3,4,5]"></radio> -->
        </div>
        <div class="line">
            <slider v-model:value="sliderValue" :width="300" :show-tips="true"
                @change="(v) => { console.log(sliderValue); }"></slider>
        </div>
        <div class="line">
            <toggle-switch v-model:value="switchValue" @change="(e) => { console.log(switchValue); }"></toggle-switch>
        </div>
        <div class="line">
            <text-button text="成功" @click="showMessage('success', '成功', '成功啦   (^&^)/')"></text-button>
            <text-button text="警告" @click="showMessage('warning', '警告', '警告！  （～￣▽￣～）')"></text-button>
            <text-button text="错误" @click="showMessage('error', '错误', '发生错误了！！！  w(ﾟДﾟ)w')"></text-button>
            <text-button text="提示" @click="showMessage('info', '提示', '这是一个消息   ( つ•̀ω•́)つ')"></text-button>
        </div>
        <div class="line">
            <customize-table :header-data="tableHeaders" :data="tableData" :height="250" :min-width="500" row-key="id">
            </customize-table>
        </div>
        <div class="line">
            <text-button text="创建数据库" @click="create_database"></text-button>
            <text-button text="创建表" @click="create_table"></text-button>
            <text-button text="插入数据" @click="insert_data"></text-button>
            <text-button text="查询数据" @click="select_data"></text-button>
            <text-button text="删除数据" @click="delete_data"></text-button>
            <text-button text="更新数据" @click="update_data"></text-button>
        </div>
        <div class="line">
            <text-button text="打开对话框" @click="isOpenDialog = true"></text-button>
            <customize-dialog v-model:open="isOpenDialog" @ok="() => { console.log('点击了确定按钮'); }">
                <template #body>
                    <div class="user-select" style="padding: 20px;">
                        <header style="font-size: 20px;font-weight: 500;margin-bottom: 10px;">
                            这是一个对话框
                        </header>
                        <p>云朵飘过天空，风儿轻轻吹拂，却引不起心中的一丝波澜。我在时间的河流里漂流，像一片落叶，不知道会飘向何方。月亮挂在夜空，星辰闪烁，但我的心却像一片荒芜的沙漠，没有生机，没有希望。我在寻找着某种东西，却又不知道那到底是什么，只是盲目地前行，不知道何时会停下脚步。这一切都是那么的无意义，却又让我感到无尽的迷茫和困惑。</p>
                    </div>
                </template>
                <!-- <template #footer>
                </template> -->
            </customize-dialog>
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
import slider from './slider/slider.vue';
import toggleSwitch from './switch/toggleSwitch.vue';
import Message from './notification/src/message';
import customizeTable from './table/customizeTable.vue';
import customizeDialog from './dialog/customizeDialog.vue';
import { useThemeStore } from '@/stores/theme.js';
import svgIcon from "@/assets/icons/svgIcon.vue";
import { ref, h } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

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
const checkboxValue = ref(false);
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
const sliderValue = ref(0)
const switchValue = ref(false);
const showMessage = (type, title, text) => {
    Message({
        title: title,
        type: type,
        text: text,
        // isBrief: false,
        duration: 5000
    })
}
const tableHeaders = [
    {
        text: '姓名',
        prop: 'name',
        width: '100px'
    },
    {
        text: '年龄',
        prop: 'age',
        width: '100px'
    },
    {
        text: '性别',
        prop: 'sex',
        width: '100px'
    },
    {
        text: '地址',
        prop: 'address',
        width: '100px'
    },
    {
        text: '时间',
        prop: 'time',
        width: '150px'
    },
    {
        text: '状态',
        prop: 'status',
        width: '100px'
    }
]
const tableData = ref([])

async function create_database() {
    let ans = await invoke("create_database");
    console.log(ans);
}
async function create_table() {
    let ans = await invoke("create_table");
    console.log(ans);
}
async function insert_data() {
    let ans = await invoke("insert_data", { name: "张三", age: 18, sex: "男", address: "北京", time: "2021-01-01", status: "正常" });
    console.log(ans);
    select_data();
}
async function select_data() {
    let ans = await invoke("select_data");
    let rows = JSON.parse(ans);
    tableData.value.length = 0;
    for (let i = 0, len = rows.length; i < len; i++) {
        tableData.value.push(rows[i]);
    }
    console.log(rows);
}
async function delete_data() {
    let ans = await invoke("delete_data", { id: tableData.value[tableData.value.length - 1].id });
    console.log(ans);
    select_data();
}
async function update_data() {
    let ans = await invoke("update_data", { id: tableData.value[tableData.value.length - 1].id, name: "李四", age: 23, sex: "女", address: "上海", time: "2022-03-02", status: "处理" });
    console.log(ans);
    select_data();
}
select_data();

const isOpenDialog = ref(false);
</script>

<style scoped>
.line {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
    padding: 10px;
    border-radius: 5px;
    margin: 10px;
    background-color: rgb(var(--theme-color));
}
</style>
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
            <tag-button text="标签" :select="select" @change="(v) => { console.log('点击了标签', v); }">
                <svg-icon name="tag"></svg-icon>
            </tag-button>
        </div>
        <div class="line">
            <two-checkbox text="复选框" @change="(v) => { console.log('点击了复选框', v); }"></two-checkbox>
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
        <div class="line">
            <slider :width="300" :show-tips="true" @change="(v) => { console.log(v); }"></slider>
        </div>
        <div class="line">
            <toggle-switch v-model:value="switchValue" @change="(e) => {console.log(switchValue);}"></toggle-switch>
        </div>
        <div class="line">
            <text-button text="成功" @click="showMessage('success','成功','成功啦   (^&^)/')"></text-button>
            <text-button text="警告" @click="showMessage('warning','警告','警告！  （～￣▽￣～）')"></text-button>
            <text-button text="错误" @click="showMessage('error','错误','发生错误了！！！  w(ﾟДﾟ)w')"></text-button>
            <text-button text="提示" @click="showMessage('info','提示','这是一个消息   ( つ•̀ω•́)つ')"></text-button>
        </div>
        <div class="line">
            <customize-table :header-data="tableHeaders" :data="tableData" :height="250" :min-width="500">
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
const tableData = [
    {
        name: '张三',
        age: 18,
        sex: '男',
        address: '北京',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '李四',
        age: 20,
        sex: '女',
        address: '上海',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '王五',
        age: 22,
        sex: '男',
        address: '广州',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '赵六',
        age: 25,
        sex: '男',
        address: '深圳',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '张三',
        age: 18,
        sex: '男',
        address: '北京',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '李四',
        age: 20,
        sex: '女',
        address: '上海',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '王五',
        age: 22,
        sex: '男',
        address: '广州',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '赵六',
        age: 25,
        sex: '男',
        address: '深圳',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '张三',
        age: 18,
        sex: '男',
        address: '北京',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '李四',
        age: 20,
        sex: '女',
        address: '上海',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '王五',
        age: 22,
        sex: '男',
        address: '广州',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '赵六',
        age: 25,
        sex: '男',
        address: '深圳',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '张三',
        age: 18,
        sex: '男',
        address: '北京',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '李四',
        age: 20,
        sex: '女',
        address: '上海',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '王五',
        age: 22,
        sex: '男',
        address: '广州',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '赵六',
        age: 25,
        sex: '男',
        address: '深圳',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '张三',
        age: 18,
        sex: '男',
        address: '北京',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '李四',
        age: 20,
        sex: '女',
        address: '上海',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '王五',
        age: 22,
        sex: '男',
        address: '广州',
        time: '2021-01-01',
        status: '正常'
    },
    {
        name: '赵六',
        age: 25,
        sex: '男',
        address: '深圳',
        time: '2021-01-01',
        status: '正常'
    },
]

async function create_database() {
  let ans = await invoke("create_database");
  console.log(ans);
}
async function create_table() {
    let ans = await invoke("create_table");
    console.log(ans);
}
async function insert_data(){
    let ans = await invoke("insert_data", {name: "张三", age: 18, sex: "男", address: "北京", time: "2021-01-01", status: "正常"});
    console.log(ans);
}
async function select_data(){
    let ans = await invoke("select_data");
    console.log(JSON.parse(ans));
}
async function delete_data(){
    let ans = await invoke("delete_data", {id: 1});
    console.log(ans);
}
async function update_data(){
    let ans = await invoke("update_data", {id: 1, name: "李四", age: 23, sex: "女", address: "上海", time: "2022-03-02", status: "处理"});
    console.log(ans);
}
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
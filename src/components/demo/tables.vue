<template>
    <div>
        <div style="color: rgb(var(--text-color));">
            <h1 style="margin-top: 10px;font-weight: 500;" class="user-select">表格</h1>
            <span
                style="display: block; width: 220px;height: 1px;background-color: rgba(var(--text-color), 0.1);"></span>
            <span>table</span>
        </div>
        <div class="line">
            <div style="display: flex;gap: 10px;">
                <text-button text="创建数据库" @click="create_database"></text-button>
                <text-button text="创建表" @click="create_table"></text-button>
                <text-button text="插入数据" @click="insert_data"></text-button>
                <text-button text="查询数据" @click="select_data"></text-button>
                <text-button text="删除数据" @click="delete_data"></text-button>
                <text-button text="更新数据" @click="update_data"></text-button>
            </div>
            <customize-table :header-data="tableHeaders" :data="tableData" :height="250" :min-width="500" row-key="id">
            </customize-table>
        </div>
    </div>
</template>

<script setup>
import customizeTable from '@/components/table/customizeTable.vue';
import textButton from '@/components/button/textButton.vue';
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

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
    if(/^数据库连接失败/.test(ans)){
        return;
    }
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
</script>

<template>
    <div class="customize-table" :style="{height: `${height}px`, 'min-width': `${minWidth}px`}">
        <table>
            <!-- <colgroup>
            </colgroup> -->
            <thead>
                <tr>
                    <th v-for="header in headerData" :key="header.prop">
                        <div>{{ header.text }}</div>
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="row in data" :key="rowKey ? row[rowKey] : row">
                    <td v-for="header in headerData" :key="header.prop">
                        <div>{{ row[header.prop] }}</div>
                    </td>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<script setup>
const props = defineProps({
    /*
     * text: 显示文本
     * prop: 数据对应字段
     */
    headerData: {
        type: Array,
        default: () => []
    },
    // 数据
    data: {
        type: Array,
        default: () => []
    },
    // 高度
    height: {
        type: Number,
        default: 0
    },
    // 最小宽度，防止表格挤压变形
    minWidth: {
        type: Number,
        default: 0
    },
    rowKey: {
        type: String,
        default: ''
    }
})
</script>

<style scoped>
.customize-table {
    width: 100%;
    border: 1px solid #ccc;
    border-top-width: 0;
    border-radius: 5px;
    table-layout: fixed;
    color: rgb(var(--app-color), 0.8);
    background-color: rgba(var(--theme-color));
    /* overflow-y: overlay; */
    overflow-y: scroll;
}

.customize-table::-webkit-scrollbar {
    -webkit-appearance: none;
    display: none;
}

table {
    width: 100%;
    border-collapse: collapse;
}

th:last-child {
    border-right: none;
}

table th div {
    padding: 10px;
    text-align: center;
    border-top: 1px solid #ccc;
    border-right: 1px solid #ccc;
    border-bottom: 1px solid #ccc;
    background-color: rgba(var(--theme-color), 0.8);
    font-weight: 100;
}

thead {
    position: sticky;
    top: 0;
    background-color: rgb(var(--theme-color));
    font-size: 12px;
}

tbody {
    font-weight: 500;
    font-size: 14px;
}

td div {
    padding: 8px 5px;
    margin: 2px 0;
    text-align: left;
}

td:first-child div {
    border-radius: 3px 0 0 3px;
    margin-left: 5px;
}

td:last-child div {
    border-radius: 0 3px 3px 0;
    margin-right: 5px;
}

tbody tr:nth-child(2n) div {
    background-color: var(--app-color);
}

tbody tr:nth-child(2n+1) div {
    background-color: rgb(var(--app-color), 0.03);
}

tbody tr:hover div {
    background-color: rgb(var(--app-color), 0.08);
}
</style>
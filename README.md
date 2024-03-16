<!-- # Tauri + Vue 3 -->

## 介绍
一个 tauri + vue3 的模板文件，可添加自己的二进制文件作为后端服务器。

![npm](https://img.shields.io/badge/npm-v9.5.1-red?logo=npm)
![vue](https://img.shields.io/badge/vue-v3.3.4-41b883?logo=vuedotjs)
![tauri](https://img.shields.io/badge/tauri-v1-ffc131?logo=tauri)

## 开始使用

1. 安装依赖
```powershell
npm install
```

2. 添加二进制文件
在`src-tauri/bin/api`目录下放入自己的二进制文件，并命名为`main-x86_64-pc-windows-msvc.exe`（Windows）。如有其他需求可参照[官方文档（嵌入外部二进制文件）](https://tauri.app/zh-cn/v1/guides/building/sidecar/)自行添加

3. Run
```powershell
npm run tauri dev
```

4. 打包
```powershell
npm run tauri build
```

## 文档

1. 组件文档

[组件说明](https://www.ycfsh.top/2024/03/08/tauri-vue/)，具体的示例可参照[demo](./src/components/demo/index.vue)
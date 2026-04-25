# Freya MID-UI Desktop App Scaffold

本技能 (`freya-mid-desktop`) 是一套面向 **Rust + Freya** 技术栈的桌面小工具标准脚手架指令。

## 设计理念 (MID-UI)
基于 Minimalist Industrial Desktop UI (极简工业桌面风格)，抛弃繁杂的多级窗口和系统菜单，采用：
1. **Activity Bar (55px 侧边栏)**：全站导航中心（主页、设置、关于）。
2. **Main Stage (内容主舞台)**：单功能聚合工作区，支持拖拽、列表状态展示。
3. **状态/交互解耦**：利用 Freya 的声明式 Signal/Context 管理深色/浅色主题、当前路由和任务状态。

## 使用场景
当您需要一个功能单一、小巧精悍且跨平台（Linux/Mac/Win）体验一致的桌面处理工具（例如日志解密器、图片压缩器、特定的格式转换器）时，请让代理加载此技能进行开发。

## 技术栈规定
- **UI 渲染**: [Freya](https://github.com/marc2332/freya)
- **文件选择**: `rfd`
- **系统调用**: `open`
- **异步运行时**: `tokio` (如有文件监控/处理需求)
- **配置持久化**: `serde` + `directories`
# Attendance（考勤报表）

海康考勤机打卡数据 → OA 导入文件 → 最终考勤报表 的转换工具（Tauri 2 + Vue 3 + Rust）。

## 功能

| 功能 | 说明 |
| --- | --- |
| 导入考勤 | 把考勤机导出的 `report.xls` 转换为 OA 可导入的 `report_oa.xlsx`（按 `baobiao.json` 建立人员与 OA 账号的映射） |
| 生成报表 | 把 OA 导出的考勤数据（如 `2026年9月考勤报表.xls`）转换为最终考勤报表 `..._report.xlsx` |
| 文件拖拽 | 支持把文件直接拖进窗口，自动识别本地绝对路径（一次拖入两个文件时，依次填入两个页签） |

> 拖拽说明：Tauri 2 的 webview 默认开启 `dragDropEnabled`，前端通过
> `getCurrentWebview().onDragDropEvent()` 即可拿到被拖入文件的本地绝对路径，
> 因此无需再手动打开文件选择框定位文件。

## 开发与构建

```bash
pnpm install
pnpm tauri dev      # 开发调试

pnpm tauri build    # 本地打包
```

`baobiao.json` 需要与可执行文件（macOS 下为 `.app`）放在同一目录，
其中登记了部门、人员以及人员对应的 OA 账号。

## 图标

`app-icon.png` 为应用图标源文件，执行下面的命令重新生成各平台图标：

```bash
node create-icon.cjs
```

## 推荐 IDE 配置

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

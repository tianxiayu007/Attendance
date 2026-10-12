# Attendance（考勤报表）

海康考勤机打卡数据 → OA 导入文件 → 最终考勤报表 的转换工具（Tauri 2 + Vue 3 + Rust）。

## 功能

| 功能 | 说明 |
| --- | --- |
| 导入考勤 | 把考勤机导出的 `report.xls` 转换为 OA 可导入的 `report_oa.xlsx`（按 `baobiao.json` 建立人员与 OA 账号的映射） |
| 生成报表 | 把 OA 导出的考勤数据（如 `2026年9月考勤报表.xls`）转换为最终考勤报表 `..._report.xlsx` |
| 文件拖拽 | 支持把文件直接拖进窗口，自动识别本地绝对路径（一次拖入两个文件时，依次填入两个页签） |
| 拖入后自动生成 | 拖拽区右上角的开关，**默认开启**：拖入文件后自动执行当前页签的操作并显示结果；关闭后只识别并填充文件路径 |

> 拖拽说明：Tauri 2 的 webview 默认开启 `dragDropEnabled`，前端通过
> `getCurrentWebview().onDragDropEvent()` 即可拿到被拖入文件的本地绝对路径，
> 因此无需再手动打开文件选择框定位文件。

## 安装与使用

1. 在 Releases 下载对应平台的包：

   **Windows（三种产物，按需选择）**

   | 文件 | 特点 | 适用场景 |
   | --- | --- | --- |
   | `Attendance-windows-x86_64-setup.exe` | 静态链接 CRT + 内置 WebView2 离线运行时，**单个安装包零依赖** | 首选，装完即用，不联网、不需要任何运行库 |
   | `Attendance-windows-x86_64-dynamic-setup.exe` | **动态链接** CRT（体积更小），同样内置 WebView2 运行时 | 目标机器已安装 VC++ 运行库 |
   | `Attendance-windows-x86_64-portable.exe`、`...-dynamic-portable.exe` | 免安装裸 exe（静态 / 动态两份） | 绿色使用 |

   - macOS：`Attendance-macos-aarch64.zip` → 解压得到 `考勤报表.app`
   - Linux：`Attendance-linux-x86_64.zip` → 解压得到 `.deb` 安装包
2. 把 `baobiao.json` 放在与程序（macOS 为 `.app`）**同一目录**。
3. 运行程序，把考勤文件拖进窗口，开启「拖入后自动生成」时会直接执行并提示结果。

### Windows 运行时依赖说明（当前为零依赖）

| 组件 | 静态版 | 动态版 |
| --- | --- | --- |
| **VC++ 运行库**（`vcruntime140.dll`、`msvcp140.dll`） | 构建注入 `RUSTFLAGS=-C target-feature=+crt-static`，MSVC 运行库**静态链接**进主程序，目标机器无需安装 | 保持默认动态链接，体积更小，要求系统已装 VC++ 运行库 |
| **WebView2 Runtime** | 安装包内置微软官方运行时安装器（`webviewInstallMode = offlineInstaller`），静默安装、**无需联网** | 同上 |

两种 Windows 产物由 CI 矩阵任务并行构建（`WINDOWS_KIND=static` / `dynamic`），都可在 Release 中下载。

### macOS 首次打开提示“已损坏”或“无法验证开发者”

本项目没有 Apple 开发者证书，发布包只做了 ad-hoc 签名，从浏览器下载后首次打开会被
Gatekeeper 拦截。任选一种方式：

- 图形界面：在 `.app` 上**右键 → 打开 → 仍要打开**（只需一次）
- 终端方式（最稳）：

```bash
cd ~/Downloads   # 换成你的实际解压位置

# 去掉浏览器下载标记（解决“已损坏，无法打开”）
xattr -dr com.apple.quarantine "考勤报表.app"

# 如仍无法打开，重新做一次 ad-hoc 签名（无需开发者证书）
codesign --force --deep --sign - "考勤报表.app"

open "考勤报表.app"
```

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

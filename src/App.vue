<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open } from "@tauri-apps/plugin-dialog";
import { Button, InputText, Message } from 'primevue';
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';

type Status = { kind: 'success' | 'error'; text: string } | null;

const importFile = ref("");
const selectFile = ref("");
const dragging = ref(false);
const activeTab = ref("0");
const busy = ref<"import" | "report" | null>(null);
const status = ref<Record<"0" | "1", Status>>({ "0": null, "1": null });
const options = ref(['导入考勤', '生成报表']);

let unlisten: (() => void) | undefined;

const isExcel = (path: string) => /\.(xlsx|xls)$/i.test(path);

const fileOf = (path: string) => path.split(/[\\/]/).filter(Boolean).pop() ?? path;
const currentFile = computed(() => (activeTab.value === "0" ? importFile.value : selectFile.value));
const currentFileName = computed(() => (currentFile.value ? fileOf(currentFile.value) : ""));

function setStatus(tab: "0" | "1", value: Status) {
  status.value = { ...status.value, [tab]: value };
}

function setFile(tab: "0" | "1", path: string) {
  setStatus(tab, null);
  if (!isExcel(path)) {
    setStatus(tab, { kind: 'error', text: '仅支持考勤机导出的 .xls/.xlsx 文件' });
    return;
  }
  if (tab === "0") {
    importFile.value = path;
  } else {
    selectFile.value = path;
  }
}

function setResult(tab: "0" | "1", message: string) {
  const failed = message.includes('失败') || message.includes('不存在') || message.includes('未找到')
    || message.includes('仅支持') || message.includes('无法') || message.includes('错误');
  setStatus(tab, { kind: failed ? 'error' : 'success', text: message });
}

async function run(tab: "0" | "1") {
  const file = tab === "0" ? importFile.value : selectFile.value;
  if (!file) {
    setStatus(tab, { kind: 'error', text: tab === "0" ? '请先选择或拖入考勤机导出的 report.xls' : '请先选择或拖入 OA 导出的考勤报表' });
    return;
  }

  busy.value = tab === "0" ? 'import' : 'report';
  setStatus(tab, null);

  try {
    const command = tab === "0" ? "import_convert" : "attendance_convert";
    setResult(tab, await invoke<string>(command, { file }));
  } catch (e) {
    setStatus(tab, { kind: 'error', text: `执行失败：${e}` });
  } finally {
    busy.value = null;
  }
}

async function openFile(tab: "0" | "1") {
  const selected = await open({
    directory: false,
    multiple: false,
    filters: [
      {
        name: "Excel Files",
        extensions: ["xlsx", "xls"]
      }
    ],
    title: "选择考勤文件",
    locale: 'zh-CN'
  });
  if (typeof selected === 'string') {
    setFile(tab, selected);
  }
}

/**
 * Tauri 2 原生拖拽：webview 默认开启 dragDropEnabled，
 * onDragDropEvent 可以直接拿到被拖入文件的本地绝对路径，
 * 因此无需再手动打开文件选择框。
 */
onMounted(async () => {
  unlisten = await getCurrentWebview().onDragDropEvent((event) => {
    const payload = event.payload;
    if (payload.type === "enter" || payload.type === "over") {
      dragging.value = true;
      return;
    }
    dragging.value = false;
    if (payload.type !== "drop" || payload.paths.length === 0) {
      return;
    }

    const paths = payload.paths.filter(isExcel);
    if (paths.length === 0) {
      setStatus(activeTab.value === "0" ? "0" : "1", { kind: 'error', text: '仅支持 .xls/.xlsx 文件，请重新拖入' });
      return;
    }

    // 一次拖入两个文件时，按“导入考勤 / 生成报表”两个页签依次填充
    setFile("0", paths[0]);
    setFile("1", paths.length > 1 ? paths[1] : paths[0]);
    setStatus(activeTab.value === "0" ? "0" : "1", {
      kind: 'success',
      text: paths.length > 1
        ? `已识别 2 个文件：${fileOf(paths[0])}、${fileOf(paths[1])}`
        : `已识别文件：${fileOf(paths[0])}`
    });
  });
});

onUnmounted(() => {
  unlisten?.();
});

</script>

<template>
  <main class="page">
    <section class="drop-zone" :class="{ active: dragging, ready: !!currentFile && !dragging }"
      @click="openFile(activeTab === '0' ? '0' : '1')">
      <p class="drop-title">{{ $t('drop_hint') }}</p>
      <p class="drop-sub">{{ $t('drop_sub_hint') }}</p>
      <p class="file-name" :class="{ placeholder: !currentFile }">
        {{ currentFileName || $t('no_file') }}
      </p>
    </section>

    <Tabs v-model:value="activeTab">
      <TabList>
        <Tab value="0">{{ options[0] }}</Tab>
        <Tab value="1">{{ options[1] }}</Tab>
      </TabList>
      <TabPanels>
        <TabPanel value="0">
          <div class="panel">
            <div class="row">
              <InputText v-model="importFile" :placeholder="$t('choose_file')" class="path-input" />
              <Button :label="$t('choose_file')" severity="secondary" outlined @click="openFile('0')" />
              <Button :label="$t('import')" :loading="busy === 'import'" :disabled="busy !== null"
                @click="run('0')" />
            </div>
            <Message v-if="status['0']" :severity="status['0'].kind" :closable="false">
              {{ status['0'].text }}
            </Message>
            <p class="tip">{{ $t('import_tip') }}</p>
          </div>
        </TabPanel>

        <TabPanel value="1">
          <div class="panel">
            <div class="row">
              <InputText v-model="selectFile" :placeholder="$t('choose_file')" class="path-input" />
              <Button :label="$t('choose_file')" severity="secondary" outlined @click="openFile('1')" />
              <Button :label="$t('convert')" :loading="busy === 'report'" :disabled="busy !== null"
                @click="run('1')" />
            </div>
            <Message v-if="status['1']" :severity="status['1'].kind" :closable="false">
              {{ status['1'].text }}
            </Message>
            <p class="tip">{{ $t('report_tip') }}</p>
          </div>
        </TabPanel>
      </TabPanels>
    </Tabs>
  </main>
</template>


<style>
:root {
  font-family: Helvetica, Arial, sans-serif;
  font-size: 15px;
  line-height: 22px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  padding: 0;
}

.page {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 14px;
  box-sizing: border-box;
}

.drop-zone {
  border: 2px dashed #b9c2cf;
  border-radius: 10px;
  background: #ffffff;
  padding: 14px 16px;
  text-align: center;
  cursor: pointer;
  transition: all 0.15s ease-in-out;
  user-select: none;
}

.drop-zone.active {
  border-color: #2f6fed;
  background: #eef4ff;
  transform: scale(1.01);
}

.drop-zone.ready {
  border-style: solid;
  border-color: #34a853;
}

.drop-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
}

.drop-sub {
  margin: 4px 0 0;
  font-size: 12px;
  color: #6b7280;
}

.file-name {
  margin: 8px 0 0;
  font-size: 13px;
  color: #34a853;
  word-break: break-all;
}

.file-name.placeholder {
  color: #9aa4b2;
}

.panel {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-input {
  flex: 1;
  min-width: 320px;
}

.tip {
  margin: 0;
  font-size: 12px;
  color: #6b7280;
}

.p-tabpanel {
  width: 100%;
  height: auto;
  min-height: fit-content;
  display: flex;
  justify-content: center;
  align-items: flex-start;
}

.p-tab {
  font-size: 14px;
}
</style>

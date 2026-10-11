<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open } from "@tauri-apps/plugin-dialog";
import { Button, InputText } from 'primevue';
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';

const importFile = ref("");
const selectFile = ref("");
const dragging = ref(false);
const options = ref(['导入考勤', '生成报表']);

let unlisten: (() => void) | undefined;

const isExcel = (path: string) => /\.(xlsx|xls)$/i.test(path);

async function importConvert() {
  importFile.value = await invoke("import_convert", { file: importFile.value });
}
async function attendanceConvert() {
  selectFile.value = await invoke("attendance_convert", { file: selectFile.value });
}

async function openFile(flag: boolean) {
  const file_name = await open({
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
  if (flag) {
    importFile.value = file_name ?? "";
  } else {
    selectFile.value = file_name ?? "";
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
      return;
    }
    // 一次拖入多个文件时，按“导入考勤 / 生成报表”两个页签依次填充
    importFile.value = paths[0];
    selectFile.value = paths.length > 1 ? paths[1] : paths[0];
  });
});

onUnmounted(() => {
  unlisten?.();
});

</script>

<template>
  <div class="drop-hint" :class="{ active: dragging }">{{ $t('drop_hint') }}</div>
  <Tabs value="0">
    <TabList>
      <Tab value="0">{{ options[0] }}</Tab>
      <Tab value="1">{{ options[1] }}</Tab>
    </TabList>
    <TabPanels>
      <TabPanel value="0">
        <div>
          <InputText v-model="importFile" :placeholder="$t('choose_file')" style="width: 500px;margin-right: 8px;" />
          <Button label="Submit" @click="openFile(true)" style="margin-right: 8px;">{{ $t('choose_file') }}</Button>
          <Button @click="importConvert">{{ $t('import') }}</Button>
        </div>
      </TabPanel>

      <TabPanel value="1">
        <div>
          <InputText v-model="selectFile" :placeholder="$t('choose_file')" style="width: 500px;margin-right: 8px;" />
          <Button label="Submit" @click="openFile(false)" style="margin-right: 8px;">{{ $t('choose_file') }}</Button>
          <Button @click="attendanceConvert">{{ $t('convert') }}</Button>
        </div>
      </TabPanel>

    </TabPanels>
  </Tabs>

</template>


<style>
:root {
  font-family: Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.p-tabpanel {
  width: 100%;
  height: auto;
  min-height: fit-content;
  display: flex;
  justify-content: center;
  align-items: center;
}
.p-tab {
  font-size: 14px;
}

.drop-hint {
  margin: 8px 8px 0;
  padding: 6px 10px;
  border: 1px dashed #c2c8d0;
  border-radius: 6px;
  font-size: 12px;
  color: #6b7280;
  text-align: center;
  transition: all 0.15s ease-in-out;
}

.drop-hint.active {
  border-color: #2f6fed;
  background: #eef4ff;
  color: #2f6fed;
}
</style>

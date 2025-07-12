<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Button, InputText } from 'primevue';
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';

const importFile = ref("");
const selectFile = ref("");
const options = ref(['导入考勤', '生成报表']);
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

</script>

<template>
  <Tabs value="0">
    <TabList>
      <Tab value="0">{{ options[0] }}</Tab>
      <Tab value="1">{{ options[1] }}</Tab>
    </TabList>
    <TabPanels>
      <TabPanel value="0">
        <div class="flex justify-center mb-8">
          <InputText v-model="importFile" :placeholder="$t('choose_file')" style="width: 500px;margin-right: 8px;" />
          <Button label="Submit" @click="openFile(true)" style="margin-right: 8px;">{{ $t('choose_file') }}</Button>
          <Button @click="importConvert">{{ $t('import') }}</Button>
        </div>
      </TabPanel>

      <TabPanel value="1">
        <div class="flex justify-center mb-8">
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
</style>

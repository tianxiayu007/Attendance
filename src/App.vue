<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
// import { useI18n } from 'vue-i18n';

const selectFile = ref("");

// const { locale } = useI18n();
// async function toggleLang() {
//   locale.value = locale.value === 'en' ? 'zh' : 'en';
// }
async function attendanceConvert() {
  selectFile.value = await invoke("attendance_convert", { file: selectFile.value });
}
async function openFile() {
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
  selectFile.value = file_name ?? "";
}

</script>

<template>
  <main class="container">
    <!-- <div class="top-right">
      <button @click="toggleLang">{{ $t('language') }}</button>
    </div>
    <div style="margin-top: 48px;"></div> -->

    <form class="row">
      <input v-model="selectFile" :placeholder="$t('choose_file')" style="width: 400px;margin-right: 5px;" />
      <button @click="openFile" type="button" style="margin-right: 5px;">{{ $t('choose_file') }}</button>
      <button @click="attendanceConvert" type="button">{{ $t('convert') }}</button>
    </form>
  </main>
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

.top-right {
  position: absolute;
  top: 16px;
  right: 16px;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>

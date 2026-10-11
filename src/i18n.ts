import { createI18n } from 'vue-i18n';

// 引入语言包
const messages = {
  en: {
    convert: 'Convert',
    import: 'Import',
    language: 'English',
    choose_file:'Choose file',
    drop_hint:'Tip: drag the attendance file into this window and the path is filled in automatically'
  },
  zh: {
    convert: '转换',
    import: '导入',
    language: '中文',
    choose_file:'选择文件',
    drop_hint:'提示：可直接把考勤文件拖到窗口内，程序会自动识别文件路径'
  }
};

export default createI18n({
  legacy: false,
  locale: 'zh', // 默认语言
  fallbackLocale: 'en',
  availableLocales: ['en', 'zh'],
  globalInjection: true,
  messages
});

import { createI18n } from 'vue-i18n';

// 引入语言包
const messages = {
  en: {
    convert: 'Convert',
    language: 'English',
    choose_file:'Choose file'
  },
  zh: {
    convert: '转换',
    language: '中文',
    choose_file:'选择文件'
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

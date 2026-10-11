import { createI18n } from 'vue-i18n';

// 引入语言包
const messages = {
  en: {
    convert: 'Convert',
    import: 'Import',
    language: 'English',
    choose_file: 'Choose file',
    drop_hint: 'Drag the attendance file here',
    drop_sub_hint: 'or click this area to pick a file',
    no_file: 'No file selected',
    import_tip: 'Import: converts the attendance machine export (report.xls) into an OA-importable file (*_oa.xlsx).',
    report_tip: 'Report: converts the OA attendance export (*.xls) into the final attendance report (*_report.xlsx).'
  },
  zh: {
    convert: '生成报表',
    import: '导入考勤',
    language: '中文',
    choose_file: '选择文件',
    drop_hint: '把考勤文件拖到这里',
    drop_sub_hint: '也可以点击此处 / 下方按钮手动选择文件',
    no_file: '尚未选择文件',
    import_tip: '导入考勤：把考勤机导出的 report.xls 转换成 OA 可导入的 *_oa.xlsx 文件。',
    report_tip: '生成报表：把 OA 导出的考勤报表（*.xls）转换成最终报表 *_report.xlsx 文件。'
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

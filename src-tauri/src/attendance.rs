use crate::util::read_json_config;
use anyhow::{Result, anyhow};
use calamine::{RangeDeserializerBuilder, Reader, open_workbook_auto};
use chrono::{Datelike, NaiveDate, Weekday};
use regex::Regex;
use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder, Workbook};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

// 定义数据结构
#[derive(Deserialize)]
struct Record {
    #[serde(rename = "人员名称")]
    person: String,
    #[serde(rename = "所属部门")]
    department: String,
    #[serde(deserialize_with = "naive_date_deserializer", rename = "日期")]
    date: Option<NaiveDate>,
    #[serde(rename = "考勤时间")]
    attendance_time: String,
    #[serde(rename = "打卡时间")]
    clock_time: String,
    #[serde(rename = "打卡状态")]
    clock_status: String,
}

// 自定义日期反序列化函数
fn naive_date_deserializer<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if !s.trim().is_empty() => NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map(Some)
            .map_err(|e| serde::de::Error::custom(format!("日期解析失败: {}", e))),
        _ => Ok(None),
    }
}

// 提取请假原因
fn extract_reason(clock_status: &str) -> String {
    let re = Regex::new(r"请假\((.*?)\)").unwrap();
    re.captures(clock_status)
        .and_then(|caps| caps.get(1))
        .map(|m| {
            let reason = m.as_str();
            if reason == "哺乳假/工间休息假" {
                "哺乳假".to_string()
            } else {
                reason.to_string()
            }
        })
        .unwrap_or_else(|| clock_status.to_string())
}

// 日期转星期
fn date_to_weekday(date: &NaiveDate) -> &'static str {
    match date.weekday() {
        Weekday::Mon => "星期一",
        Weekday::Tue => "星期二",
        Weekday::Wed => "星期三",
        Weekday::Thu => "星期四",
        Weekday::Fri => "星期五",
        Weekday::Sat => "星期六",
        Weekday::Sun => "星期日",
    }
}

// 主处理函数
pub fn handle_attendance(src_path: &Path) -> Result<(), anyhow::Error> {
    let json_config = read_json_config().map_err(|e| anyhow!("读取配置文件失败: {}", e))?;

    if !src_path.is_file() {
        return Err(anyhow!("源文件不存在"));
    }

    let ext = src_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_lowercase();

    if ext != "xls" && ext != "xlsx" {
        return Err(anyhow!("仅支持 .xls/.xlsx 文件"));
    }

    // 打开 Excel 文件
    let mut workbook = open_workbook_auto(src_path)?;
    let range = workbook
        .worksheet_range("考勤记录")
        .map_err(|_| anyhow!("找不到 '考勤记录' 工作表"))?;

    // 反序列化数据
    let mut records: Vec<Record> = RangeDeserializerBuilder::new()
        .from_range(&range)?
        .filter_map(Result::ok)
        .collect();

    // 过滤无效记录
    records.retain(|r| {
        !r.date.is_none()
            && !r.clock_time.is_empty()
            && !r.clock_status.is_empty()
            && !r.clock_status.contains("出差")
    });

    // 获取日期频率
    let date_counts = records.iter().fold(HashMap::new(), |mut map, record| {
        *map.entry(record.date.clone()).or_insert(0) += 1;
        map
    });

    // 找出高频日期（>30次）
    let mut dates_abnormal: Vec<NaiveDate> = date_counts
        .into_iter()
        .filter_map(|(date_opt, count)| if count > 30 { date_opt } else { None })
        .collect();
    dates_abnormal.sort();

    if dates_abnormal.is_empty() {
        return Err(anyhow!("考勤记录为空"));
    }

    // 获取年份和月份
    let first_date = dates_abnormal[0];
    let max_year = format!("{}年", first_date.year());
    let max_month = format!("{}月", first_date.month());

    // 准备输出路径
    let output_path = src_path.with_file_name(format!(
        "{}_process.xlsx",
        src_path.file_stem().unwrap().to_string_lossy()
    ));

    // 创建 Excel 工作簿
    let mut workbook = Workbook::new();

    // 定义单元格格式
    let header_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_font_name("Arial")
        .set_font_size(12);

    let data_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_font_name("Arial")
        .set_font_size(11);

    let red_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_font_color(Color::Red)
        .set_font_name("Arial")
        .set_font_size(11);

    let signature_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_font_name("Arial")
        .set_font_size(11);

    // 处理每个部门
    for (dept_name, names_map) in &json_config.departments {
        let names: Vec<&str> = names_map.keys().map(|k| k.as_str()).collect();

        // 创建部门工作表
        let sheet = workbook.add_worksheet().set_name(dept_name)?;
        sheet.set_landscape();
        sheet.set_paper_size(9);
        sheet.set_print_fit_to_pages(1, 0);

        // 页边距设置
        sheet.set_margins(0.1, 0.1, 0.2, 0.2, 0.0, 0.0);
        // 打印居中设置
        sheet.set_print_center_horizontally(true);

        // 设置标题行
        let title = format!("{}{}{}考勤报表", dept_name, max_year, max_month);
        sheet.merge_range(
            0,
            0,
            0,
            dates_abnormal.len() as u16 + 2,
            &title,
            &header_format,
        )?;

        // 设置表头
        let headers: Vec<String> = ["", ""]
            .into_iter()
            .map(|s| s.to_string())
            .chain(
                dates_abnormal
                    .iter()
                    .map(|d: &NaiveDate| d.format("%-m月%-d日").to_string()),
            )
            .chain(std::iter::once("签名".to_string()))
            .collect();

        let header_refs: Vec<&str> = headers.iter().map(|s| s.as_str()).collect();
        sheet.write_row_with_format(1, 0, header_refs, &header_format)?;

        // 设置星期行
        let weekdays = ["人员名称", "时段"]
            .iter()
            .cloned()
            .chain(dates_abnormal.iter().map(|d| date_to_weekday(d)))
            .chain(std::iter::once(""))
            .collect::<Vec<&str>>();

        sheet.write_row_with_format(2, 0, weekdays, &header_format)?;

        // 合并第2、行的第1、2列
        sheet.merge_range(1, 0, 2, 0, "人员名称", &header_format)?;
        sheet.merge_range(1, 1, 2, 1, "时段", &header_format)?;
        // 合并最后一列的1、2行
        sheet.merge_range(
            1,
            dates_abnormal.len() as u16 + 2,
            2,
            dates_abnormal.len() as u16 + 2,
            "签名",
            &header_format,
        )?;

        // 填充数据
        let mut row_idx = 3;
        for name in names {
            // 上午记录
            sheet.write_string_with_format(row_idx, 0, name, &data_format)?;
            sheet.write_string_with_format(row_idx, 1, "上午", &data_format)?;

            // 下午记录（人员名称留空）
            sheet.write_string_with_format(row_idx + 1, 1, "下午", &data_format)?;

            // 合并姓名单元格
            sheet.merge_range(row_idx, 0, row_idx + 1, 0, name, &data_format)?;

            // 处理每个日期
            for (col_idx, date) in dates_abnormal.iter().enumerate() {
                let col = col_idx as u16 + 2; // 偏移前两列

                // 查找上午记录
                let am_record = records.iter().find(|r| {
                    r.department == *dept_name
                        && r.person == name
                        && r.date == Some(*date)
                        && r.attendance_time == "08:45"
                });

                let am_value = match am_record {
                    Some(r) if r.clock_status == "正常" => r.clock_time.clone(),
                    Some(r) => extract_reason(&r.clock_status),
                    None => "缺卡".to_string(),
                };

                // 查找下午记录
                let pm_record = records.iter().find(|r| {
                    r.department == *dept_name
                        && r.person == name
                        && r.date == Some(*date)
                        && r.attendance_time == "17:30"
                });

                let pm_value = match pm_record {
                    Some(r) if r.clock_status == "正常" => r.clock_time.clone(),
                    Some(r) => extract_reason(&r.clock_status),
                    None => "缺卡".to_string(),
                };

                // 写入上午记录
                if am_value.contains(':') {
                    sheet.write_string_with_format(row_idx, col, &am_value, &data_format)?;
                } else {
                    sheet.write_string_with_format(row_idx, col, &am_value, &red_format)?;
                }

                // 写入下午记录
                if pm_value.contains(':') {
                    sheet.write_string_with_format(row_idx + 1, col, &pm_value, &data_format)?;
                } else {
                    sheet.write_string_with_format(row_idx + 1, col, &pm_value, &red_format)?;
                }
            }

            // 签名列
            sheet.merge_range(
                row_idx,
                dates_abnormal.len() as u16 + 2,
                row_idx + 1,
                dates_abnormal.len() as u16 + 2,
                "",
                &signature_format,
            )?;

            row_idx += 2;
        }

        // 设置姓名/时段列宽
        sheet.set_column_width(0, 9.0)?;
        sheet.set_column_width(1, 5.0)?;

        // 日期列宽
        for col in 2..(dates_abnormal.len() as u16 + 2) {
            sheet.set_column_width(col, 9.0)?;
        }

        //签名列宽
        sheet.set_column_width(dates_abnormal.len() as u16 + 2, 14.0)?;

        // 设置行高
        for row in 0..row_idx {
            sheet.set_row_height(row as u32, 38.0)?;
        }
    }

    workbook.save(&output_path)?;
    Ok(())
}

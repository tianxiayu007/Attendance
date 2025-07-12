use crate::util::read_json_config;
use anyhow::{Result, anyhow};
use chrono::{NaiveDateTime, ParseResult};
use encoding_rs::*;
use indexmap::IndexMap;
use rust_xlsxwriter::Workbook;
use scraper::{Html, Selector};
use std::path::Path;

#[derive(Debug)]
struct Record {
    name: String,
    account: String,
    time_in_or_out: NaiveDateTime,
}
pub fn handle_import(src_path: &Path) -> Result<(), anyhow::Error> {
    let json_config = read_json_config().map_err(|e| anyhow!("读取配置文件失败: {}", e))?;

    if !src_path.is_file() {
        return Err(anyhow!("源文件不存在"));
    }

    let ext = src_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_lowercase();

    if ext != "xls" {
        return Err(anyhow!("仅支持考勤软件导出的.xls文件"));
    }

    let mut names_map_extend = IndexMap::<String, String>::new();
    for (_, names_map) in &json_config.departments {
        names_map_extend.extend(names_map.clone());
    }
    println!("{:?}", names_map_extend);

    let raw_bytes = std::fs::read(src_path)?;
    let (cow, _, _) = UTF_8.decode(&raw_bytes);
    let xml_content = cow.into_owned();
    let table_data = parse_html_table(&xml_content, &names_map_extend)?;

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.write_row(0, 0, ["姓名", "账号", "打卡时间"])?;
    let mut row_start = 1;
    for record in table_data {
        let time_in_or_out_str = record.time_in_or_out.format("%Y-%m-%d %H:%M").to_string();
        sheet.write_row(
            row_start,
            0,
            [record.name, record.account, time_in_or_out_str],
        )?;
        row_start += 1;
    }

    let output_path = src_path.with_file_name(format!(
        "{}_oa.xlsx",
        src_path.file_stem().unwrap().to_string_lossy()
    ));
    workbook.save(&output_path)?;

    Ok(())
}

fn parse_html_table(
    html_content: &str,
    names_map: &IndexMap<String, String>,
) -> Result<Vec<Record>, anyhow::Error> {
    let document = Html::parse_document(html_content);
    let table_selector = Selector::parse(r#"table[class="Punch_Report"]"#).unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td, th").unwrap();

    let table = document
        .select(&table_selector)
        .next()
        .ok_or_else(|| anyhow!("未找到指定的数据表,确认考勤软件导出的.xls文件是否完整"))?;

    let mut table_data = Vec::new();
    for row in table.select(&row_selector) {
        let mut row_data = Vec::new();

        for (cell_index, cell) in row.select(&cell_selector).enumerate() {
            if cell_index == 2 || cell_index == 6 || cell_index == 9 || cell_index == 10 {
                row_data.push(cell.text().collect::<String>().trim().to_string());
            }
        }

        let account = names_map
            .get(&row_data[0])
            .cloned()
            .unwrap_or_else(|| "".to_string());
        if account.is_empty() {
            continue;
        }

        if row_data.len() == 4 {
            let time_in = parse_datetime(&row_data[1], &row_data[2]);
            let time_out = parse_datetime(&row_data[1], &row_data[3]);

            if time_in.is_ok() {
                table_data.push(Record {
                    name: row_data[0].clone(),
                    account: account.clone(),
                    time_in_or_out: time_in.unwrap().clone(),
                });
            }
            if time_out.is_ok() {
                table_data.push(Record {
                    name: row_data[0].clone(),
                    account: account.clone(),
                    time_in_or_out: time_out.unwrap().clone(),
                });
            }
        }
    }

    Ok(table_data)
}

fn parse_datetime(date_str: &str, time_str: &str) -> ParseResult<NaiveDateTime> {
    let datetime_str = format!("{} {}", date_str, time_str);
    NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
}

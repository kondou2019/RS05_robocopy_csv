use chrono::prelude::NaiveDateTime;
use std::io::prelude::*;
use std::str::FromStr;

/***
 * @brief ヘッダの行をキーと値に分離する
 * @param[in] line 行
 * @retval (キー, 値)
 */
pub fn kv_split<'a>(line:&'a str) -> Option<(&'a str, &'a str)>{
    let index = line.find(":")?;
    let k:&'a str = &line[0..index].trim();
    let v:&'a str = &line[index+1..line.len()].trim();
    Some((k, v))
}

/***
 * @brief ログファイルの解析状態
 */
#[derive(PartialEq)]
enum LineStatus {
    Reset,
    Title,
    Header,
    Context,
    Footer,
}

/***
 * @brief ログファイルのヘッダ部分の管理構造体
 */
#[derive(Default)]
struct Header {
    started: NaiveDateTime,
    source: String,
    dest: String,
    files: String,
    options: String,
}

/***
 * @brief ログファイルのフッタ部分の管理構造体
 */
#[derive(Default)]
pub struct FooterDetail {
    pub total: u64,
    pub copied: u64,
    pub skipped: u64,
    pub mismatch: u64,
    pub failed: u64,
    pub extras: u64,
}
impl FromStr for FooterDetail {
    type Err = anyhow::Error;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        let columns: Vec<&str> = v.split_whitespace().collect();
        let mut index=0;
        //
        let total:u64 = FooterDetail::local_from_str_value(&columns, &mut index)?;
        let copied:u64 = FooterDetail::local_from_str_value(&columns, &mut index)?;
        let skipped:u64 = FooterDetail::local_from_str_value(&columns, &mut index)?;
        let mismatch:u64 = FooterDetail::local_from_str_value(&columns, &mut index)?;
        let failed:u64 = FooterDetail::local_from_str_value(&columns, &mut index)?;
        let extras:u64 = FooterDetail::local_from_str_value(&columns, &mut index)?;

        Ok(Self{total, copied, skipped, mismatch, failed, extras})
    }
}
impl FooterDetail {
    /***
     * @brief 補助単位を数値で取得する
     * @param[in] prefix 補助単位
     */
    pub fn get_prefix(prefix:&str) -> anyhow::Result<f64> {
        let k:f64 = match prefix {
        "k" => 1024.0,
        "m" => 1024.0*1024.0,
        "g" => 1024.0*1024.0*1024.0,
        "t" => 1024.0*1024.0*1024.0*1024.0,
        _ => return Err(anyhow::anyhow!("未知の補助単位です")),
        };
        Ok(k)
    }
    /***
     * @brief from_str()の内部関数
     * @param[in] columns 文字列要素
     * @param[in,out] index 開始位置を入力し、次の位置に更新する
     */
    fn local_from_str_value(columns: &Vec<&str>, index:&mut usize) -> anyhow::Result<u64> {
        if *index >= columns.len() {
            return Err(anyhow::anyhow!("個数が足りません"));
        }
        let result:u64 = match columns[*index].find(".") { // 浮動小数点?
            Some(_) => {
                let v_float:f64 = columns[*index].parse()?;
                // 補助単位から係数を求める
                *index += 1;
                if *index >= columns.len() {
                    return Err(anyhow::anyhow!("個数が足りません"));
                }
                let k:f64 = FooterDetail::get_prefix(&columns[*index])?;
                //
                (v_float * k) as u64
            },
            None => columns[*index].parse()?,
        };
        *index += 1;
        Ok(result)
    }
}
/***
 * @brief ログファイルのフッタ部分の管理構造体
 */
#[derive(Default)]
struct Footer {
    ended: NaiveDateTime,
    dirs: FooterDetail,
    files: FooterDetail,
    bytes: FooterDetail,
    times: String,
}

/***
 * @brief CSVヘッダを出力
 */
pub fn print_header() {
    let mut buf = String::with_capacity(512 as usize);
    buf.push_str("started\tended");
    buf.push_str("\tsource\tdest");
    buf.push_str("\tdirs_total\tdirs_copied\tdirs_skipped\tdirs_mismatch\tdirs_failed\tdirs_extras");
    buf.push_str("\tfiles_total\tfiles_copied\tfiles_skipped\tfiles_mismatch\tfiles_failed\tfiles_extras");
    buf.push_str("\tbytes_total\tbytes_copied\tbytes_skipped\tbytes_mismatch\tbytes_failed\tbytes_extras");
    //
    println!("{}", &buf);
}

/***
 * @brief 集計結果を出力
 * @param[in] header ヘッダ情報
 * @param[in] footer フッタ情報
 */
fn print_report(header:&Header, footer:&Footer) {
    let mut buf = String::with_capacity(512 as usize);
    buf.push_str(&format!("{}\t{}", header.started, footer.ended));
    buf.push_str(&format!("\t{}\t{}", header.source, header.dest));
    let fd:&FooterDetail = &footer.dirs;
    buf.push_str(&format!("\t{}\t{}\t{}\t{}\t{}\t{}", fd.total, fd.copied, fd.skipped, fd.mismatch, fd.failed, fd.extras));
    let fd:&FooterDetail = &footer.files;
    buf.push_str(&format!("\t{}\t{}\t{}\t{}\t{}\t{}", fd.total, fd.copied, fd.skipped, fd.mismatch, fd.failed, fd.extras));
    let fd:&FooterDetail = &footer.bytes;
    buf.push_str(&format!("\t{}\t{}\t{}\t{}\t{}\t{}", fd.total, fd.copied, fd.skipped, fd.mismatch, fd.failed, fd.extras));
    //
    println!("{}", &buf);
}

/***
 * @brief robocopyのログファイルをCSV形式に整形する
 * @param[in] reader ログファイル
 */
pub fn format_csv<R>(reader: std::io::BufReader<R>) -> anyhow::Result<()> where R: std::io::Read, {
    let mut header:Header = Default::default();
    let mut footer:Footer = Default::default();
    let mut status = LineStatus::Reset;
    for line in reader.lines() {
        let line = line?;
        match status {
        LineStatus::Reset => {
            if line.starts_with("----") {
                status = LineStatus::Title;
            }
        },
        LineStatus::Title => {
            if line.starts_with("----") {
                status = LineStatus::Header;
            }
        },
        LineStatus::Header => {
            if line.starts_with("----") {
                status = LineStatus::Context;
            } else {
                if let Some((k, v)) = kv_split(&line) {
                    match k {
                    "開始" | "Started" => {
                        let dt: NaiveDateTime = NaiveDateTime::parse_from_str(&v, "%Y年%m月%d日 %H:%M:%S").unwrap();
                        header.started = dt;
                    },
                    "コピー元" | "Source" => header.source = v.to_string(),
                    "コピー先" | "Dest" => header.dest = v.to_string(),
                    "ファイル" | "Files" => header.files = v.to_string(),
                    "オプション" | "Options" => header.options = v.to_string(),
                    _ => (),
                    }
                }
            }
        },
        LineStatus::Context =>  {
            if line.starts_with("----") {
                status = LineStatus::Footer;
            }
        },
        LineStatus::Footer =>  {
            if line.starts_with("----") { // 次のrobocopyコマンドのログが始まった?
                // 集計結果を出力
                print_report(&header, &footer);
                header = Default::default();
                footer = Default::default();
                //
                status = LineStatus::Title;
            } else {
                if let Some((k, v)) = kv_split(&line) {
                    match k {
                    "ディレクトリ" | "Dirs" => footer.dirs = FooterDetail::from_str(&v)?,
                    "ファイル" | "Files" => footer.files = FooterDetail::from_str(&v)?,
                    "バイト" | "Bytes" => footer.bytes = FooterDetail::from_str(&v)?,
                    "時刻" | "Times" => footer.times = v.to_string(),
                    "終了" | "Ended" => {
                        let dt: NaiveDateTime = NaiveDateTime::parse_from_str(&v, "%Y年%m月%d日 %H:%M:%S").unwrap();
                        footer.ended = dt;
                    },
                    _ => (),
                    }
                }
            }
        },
        };
    }
    if status == LineStatus::Footer {
        print_report(&header, &footer);
    }
    //
    Ok(())
}

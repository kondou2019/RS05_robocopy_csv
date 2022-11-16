use clap::Parser;
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;

/***
 * @brief コマンドオプション
 */
#[derive(Clone, clap::ArgEnum)]
enum Encode {
    Sjis,
    Utf8,
}

#[derive(clap::Parser)]
#[clap(author, version, about="robocopyコマンドのログ出力をCSV形式に整形する。")]
struct Cli {
    #[clap(long, help="ヘッダ出力を抑止する")]
    no_header: bool,

    #[clap(arg_enum, long, default_value_t=Encode::Sjis, help="ログファイルの文字エンコード")]
    encode: Encode,

    #[clap(long, default_value="%Y年%m月%d日 %H:%M:%S", help="ログファイルの日時形式")]
    date_format: String,

    #[clap(long, default_value="%Y-%m-%d %H:%M:%S", help="出力データの日時形式")]
    output_date_format: String,

    // ファイル
    #[clap(help="ログファイル")]
    files: Vec<PathBuf>,
}

/***
 * @brief 主入口点
 */
fn main() -> anyhow::Result<()> {
    /***
     * コマンドオプションの解析
     */
    let cli:Cli = Cli::parse();
    /***
     * CSVヘッダの出力
     */
    if cli.no_header == false {
        robocopy_csv::print_header();
    }
    /***
     * ログファイルの処理
     */
    for log_path in cli.files {
        match cli.encode {
        Encode::Utf8 => {
            let f = File::open(log_path)?;
            let reader = BufReader::new(f);
            robocopy_csv::format_csv(reader, &cli.date_format, &cli.output_date_format)?;
        },
        Encode::Sjis => {
            let buf:Vec<u8> = std::fs::read(log_path).unwrap();
            let (text, _, _) = encoding_rs::SHIFT_JIS.decode(&buf); // SJIS->UTF8
            let f = std::io::Cursor::new(text.into_owned());
            let reader = BufReader::new(f);
            robocopy_csv::format_csv(reader, &cli.date_format, &cli.output_date_format)?;
        },
        };
    }
    //
    Ok(())
}

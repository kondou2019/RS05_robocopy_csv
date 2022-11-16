use clap::Parser;
use std::path::PathBuf;

#[derive(clap::Parser)]
#[clap(author, version, about="robocopyコマンドのログ出力をCSV形式に整形する。")]
struct Cli {
    #[clap(long, help="ヘッダ出力を抑止する")]
    no_header: bool,
    // ファイル
    #[clap(help="ログファイル")]
    files: Vec<PathBuf>,
}

/***
 * @brief 主入口点
 */
fn main() -> anyhow::Result<()> {
    let cli:Cli = Cli::parse();
    //
    if cli.no_header == false {
        robocopy_csv::print_header();
    }
    for log_path in cli.files {
        robocopy_csv::format_csv(&log_path)?;
    }

    Ok(())
}

# RS05_robocopy_csv

Robocopy.exeのログ出力をCSV形式に整形する。  

## 目的

Robocopy.exeコマンドのログをExcelで処理しようとした。ログをそのまま読み込むの大変なのでCSV形式に整形する。  

# 機能

[x]ログのヘッダ部分とフッタ部分の情報を出力する。途中のディレクトリ名やファイル名のログは出力しない。  
[x]フッタ部の数値に補助単位(m,g)が出力される。1024として解釈する。  
[x]日時部分の書式はコマンドオプションで指定する。指定値の詳細は以下を参照する。  
https://docs.rs/chrono/latest/chrono/format/strftime/index.html

# 出力

[x]標準出力に結果を出力する。  
[x]CSV形式で出力する。区切り記号はタブとする。  
[x]1行目にヘッダを出力する。--no-headerオプションを指定するとヘッダを出力しない。  
[x]1回のRobocopy.exeの実行結果を1行に出力する。  
[x]1つのファイルに複数回の実行結果が格納されている場合は、実行回数分の結果が出力される。  

[x]出力例
```
C:\> robocopy_csv out.log
started	ended	source	dest	dirs_total	dirs_copied	dirs_skipped	dirs_mismatch	dirs_failed	dirs_extras	files_total	files_copied	files_skipped	files_mismatch	files_failed	files_extras	bytes_total	bytes_copied	bytes_skipped	bytes_mismatch	bytes_failed	bytes_extras
2022-11-12 06:58:22	2022-11-12 06:58:22	C:\0001\	C:\0002\	3	0	3	0	0	0	18	0	18	0	0	0	23302	0	23302	0	0	0
C:\> 
```

# 実行例

[x]基本  
・ログはSJISで出力される  
```
C:\> Robocopy.exe 0001 0002 > out.log
C:\> robocopy_csv out.log
```

[x]UTF-8のログ出力  
```
C:\> chcp 65001
C:\> Robocopy.exe 0001 0002 /UNILOG:NUL /TEE > out.log
C:\> robocopy_csv --encode utf8 out.log
```

[x]日時形式を指定
・"Monday, August 8, 2022 9:16:01 AM"の形式の場合
```
C:\> robocopy_csv --encode utf8 --date-format "%A, %B %e, %Y %k:%M:%S %p" out.log
```

# 参考資料

[x]Robocopyのログ出力の文字コードについて調べてみた  
https://n-archives.net/software/robosync/articles/robocopy-unicode-unilog-log/

# 履歴

2022-11-16 開始


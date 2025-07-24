# Heurs

Library for heulistic contest participants.

## テストケース実行時のスコア・時間出力

ランナーは提出プログラムの**標準エラー出力**を走査し、以下のマーカー付き行をパースしてスコアと実行時間（ミリ秒）を取得します。

```
@@HEURS_SCORE=<整数>
@@HEURS_TIME_MS=<整数>
```

* 行頭の `@@` と `HEURS_` プレフィックスは必須です。  
* `=` の右側は 10 進整数のみを記述してください（単位は書かないでください）。  
* それぞれ **1 行に 1 情報** を出力してください。  
* 順序は問いません。どちらか一方のみ出力した場合、もう一方は 0 とみなされます。  
* 複数回出力した場合は**最後に出力された値**が採用されます。

例（C++）

```cpp
#include <iostream>
int main() {
    // ... solve problem ...
    long long score = 1234;
    long long time_ms = 567;
    std::cerr << "@@HEURS_SCORE=" << score << "\n";
    std::cerr << "@@HEURS_TIME_MS=" << time_ms << "\n";
    return 0;
}
```

このフォーマットを守ることで、CLI と Web フロントエンドは自動的にデータベースへスコアと実行時間を保存し、ランキング等に利用できるようになります。 

## CLI の使い方

### Install
```bash
just install
```

### Run
提出プログラムを指定したテストケースで実行し、結果・スコアを DB に保存します。

```bash
heurs run <SOURCE_PATH> \
  --cases <N> \            # 使用するテストケース数 (既定 10)
  --parallel <N> \         # 並列実行スレッド数 (既定 1)
  --timeout <SEC> \        # タイムアウト秒数 (既定 10)
  --config <PATH> \        # 設定ファイル (既定 "heurs.toml")
  --database-url <URL> \   # DB URL (既定 "sqlite://heurs.db")
  --env <MODE>              # 実行環境 (local / aws 等). 指定なしなら HEURS_ENV 変数 or "local"
```

例:

```bash
heurs run submission.cpp --cases 20 --parallel 4 --timeout 30 --env aws
```

> **備考**: CLI は内部で README 前章のマーカー (`@@HEURS_SCORE=...` など) をパースし、`execution_results` テーブルにスコアと実行時間を保存します。 


### TestCase
テストケースの登録 / 削除を行います。<br>
過去の実験との整合性を保つため、編集操作はサポートされていません。　

#### TestCase Add
テストケースを登録します。<br>
基本的には最初に一回実行されることを想定していますが、後から追加することも可能です。
```bash
heurs testcase add --input-path ./cases
```
* `--input-path` にはテストケースファイルが入ったディレクトリを指定してください。
* ファイル名順 (`Filename` 昇順) に並べ替えられて登録されます。


#### TestCase Clear
テストケースの一括削除を行います。<br>

```bash
heurs testcase clear
```

### LeaderBoard
指定問題の提出を平均スコア順に並べて上位 N 件を表示します。
```bash
heurs leaderboard --limit 20
```

### Submission

#### Submission Describe
任意の submission の各テストケース詳細を表示します。
```bash
heurs submission describe --submission-id <ID>
```

## Frontend

### Usage

#### Preparation
バックエンドサービスを以下のコマンドで起動しておく必要があります。
```bash
cargo run -p heurs-back
```

#### Run
`frontend` 下で `trunk serve` を実行します。

### Pages

#### Submission
ソースコードを提出するためのページです。
ToDoとして、パラメータ探索を含めた実行などがあります。

<img width="1103" height="599" alt="Image" src="https://github.com/user-attachments/assets/30f62c40-8b74-44a2-90fd-048d04b5a246" />

#### Submissions
実行結果の比較・確認を行うためのページです。<br>
ToDoとして、グラフによる可視化や、テストケースを特徴量で絞り込んでの比較などがあります。

<img width="1168" height="880" alt="Image" src="https://github.com/user-attachments/assets/ebb6de85-ac57-4310-a171-9356bb554c25" />

#### TestCases
テストケースの中身を確認するためのページです。
<img width="1103" height="608" alt="Image" src="https://github.com/user-attachments/assets/7a3c0d52-9953-4a39-8e3d-3aaf55d12041" />

#### Visualize
自作のビジュアライザを埋め込む他のページです。<br>
入出力を元に canvas を生成する関数を書けば自動で埋め込まれる仕組みにする予定ですが整備中です。
<img width="1113" height="472" alt="Image" src="https://github.com/user-attachments/assets/c0e5c792-7da9-4894-af83-5c1eb80ca068" />

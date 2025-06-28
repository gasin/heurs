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

`heurs` バイナリには **Run** と **TestCase** の 2 つのサブコマンドがあります。

### Run
提出プログラムを指定したテストケースで実行し、結果・スコアを DB に保存します。

```bash
heurs run <SOURCE_PATH> \
  --cases <N> \            # 使用するテストケース数 (既定 10)
  --parallel <N> \         # 並列実行スレッド数 (既定 1)
  --timeout <SEC> \        # タイムアウト秒数 (既定 10)
  --user-id <ID> \         # ユーザ ID (既定 0)
  --problem-id <ID> \      # 問題 ID (既定 0)
  --database-url <URL>      # DB URL (既定 "sqlite://heurs.db")
```

例:

```bash
heurs run submission.cpp --cases 20 --parallel 4 --user-id 42 --problem-id 3
```

### TestCase サブコマンド
テストケースの登録 / 全削除を行います。

| サブコマンド | 説明 |
|--------------|------|
| `add`   | ディレクトリ内の `.txt` / `.in` ファイルをテストケースとして DB に登録 |
| `clear` | すべてのテストケースを削除 |

#### 追加
```bash
heurs testcase add --input-path ./cases --problem-id 3
```
* `--input-path` にはテストケースファイルが入ったディレクトリを指定してください。
* ファイル名順 (`Filename` 昇順) に並べ替えられて登録されます。

#### 削除
```bash
heurs testcase clear
```

---

> **備考**: CLI は内部で README 前章のマーカー (`@@HEURS_SCORE=...` など) をパースし、`execution_results` テーブルにスコアと実行時間を保存します。 
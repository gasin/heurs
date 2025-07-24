#!/bin/bash
# s3_compile_run.sh: C++ ソースを S3 から取得 → コンパイル → 入力ファイル群に対して実行し、結果を S3 へアップロードする
# 必須環境変数:
#   CODE_BUCKET   : C++ ソースが格納されている S3 バケット
#   CODE_KEY      : ソースファイルのキー (例: src/main.cpp)
#   SEED_START    : 実行対象シードの開始番号 (整数, 0 など)
#   SEED_END      : 実行対象シードの終了番号 (整数, 開始以上)
# 任意環境変数:
#   IO_BUCKET     : 入力/出力ファイルを置く S3 バケット (デフォルト CODE_BUCKET)
#   INPUT_PREFIX  : 入力ファイルのプレフィックス (デフォルト inputs/)
#   INPUT_SUFFIX  : 入力ファイルのサフィックス (デフォルト .txt)
#   OUTPUT_PREFIX : 出力ファイルのプレフィックス (デフォルト outputs/output_)
#   OUTPUT_SUFFIX : 出力ファイルのサフィックス (デフォルト .txt)

set -euo pipefail

# ---- 変数チェック ----
: "${CODE_BUCKET:?Need to set CODE_BUCKET}"  # S3 バケット
: "${CODE_KEY:?Need to set CODE_KEY}"        # ソースのキー
: "${SEED_START:?Need to set SEED_START}"    # シード開始
: "${SEED_END:?Need to set SEED_END}"        # シード終了

IO_BUCKET="${IO_BUCKET:-$CODE_BUCKET}"
INPUT_PREFIX="${INPUT_PREFIX:-inputs/}"
INPUT_SUFFIX="${INPUT_SUFFIX:-.txt}"
OUTPUT_PREFIX="${OUTPUT_PREFIX:-outputs/output_}"
OUTPUT_SUFFIX="${OUTPUT_SUFFIX:-.txt}"

# 標準エラー出力ファイルの S3 キー設定
ERROR_PREFIX="${ERROR_PREFIX:-errors/error_}"
ERROR_SUFFIX="${ERROR_SUFFIX:-.txt}"

echo "📥 Downloading source: s3://${CODE_BUCKET}/${CODE_KEY}"
aws s3 cp "s3://${CODE_BUCKET}/${CODE_KEY}" /tmp/main.cpp

echo "🔧 Compiling C++ source..."
g++ -std=c++20 -O2 /tmp/main.cpp -o /tmp/main

echo "✅ Compile finished. Executing seeds ${SEED_START}-${SEED_END}"

for ((seed=SEED_START; seed<=SEED_END; seed++)); do
  printf -v seed_padded "%04d" "$seed"
  INPUT_KEY="${INPUT_PREFIX}${seed_padded}${INPUT_SUFFIX}"
  OUTPUT_KEY="${OUTPUT_PREFIX}${seed}${OUTPUT_SUFFIX}"
  ERROR_KEY="${ERROR_PREFIX}${seed}${ERROR_SUFFIX}"

  echo "\n▶️  Seed $seed"
  echo "   ↙️  Download input: s3://${IO_BUCKET}/${INPUT_KEY}"
  aws s3 cp "s3://${IO_BUCKET}/${INPUT_KEY}" /tmp/input.txt

  echo "   ⚙️  Running program..."
  /tmp/main < /tmp/input.txt > /tmp/output.txt 2> /tmp/error.txt

  echo "   ↗️  Upload stdout: s3://${IO_BUCKET}/${OUTPUT_KEY}"
  aws s3 cp /tmp/output.txt "s3://${IO_BUCKET}/${OUTPUT_KEY}"

  echo "   ↗️  Upload stderr: s3://${IO_BUCKET}/${ERROR_KEY}"
  aws s3 cp /tmp/error.txt "s3://${IO_BUCKET}/${ERROR_KEY}"
  echo "   ✅ Done seed $seed"
done

echo "🏁 All seeds processed successfully." 
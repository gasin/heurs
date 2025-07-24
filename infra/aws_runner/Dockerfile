FROM amazonlinux:2023

# 必要なツールをインストール (g++ なども含む)
RUN yum install -y unzip tar gzip xz awscli gcc-c++ make

# コンパイル & 実行用スクリプトをコピー
COPY s3_compile_run.sh /usr/local/bin/s3_compile_run.sh
RUN chmod +x /usr/local/bin/s3_compile_run.sh

# デフォルトエントリポイント
ENTRYPOINT ["/usr/local/bin/s3_compile_run.sh"]

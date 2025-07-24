#!/usr/bin/env bash
set -euo pipefail

# .env があれば読み込む（ディレクトリ直下想定）
if [ -f "$(dirname "$0")/.env" ]; then
  set -a
  source "$(dirname "$0")/.env"
  set +a
fi

# 環境変数必須チェック（ .env またはシェル環境で定義する）
: "${AWS_RUNNER_IMAGE:?Need to set AWS_RUNNER_IMAGE (e.g. 123456789012.dkr.ecr.ap-northeast-1.amazonaws.com/heurs/runner:latest)}"
: "${AWS_RUNNER_JOB_ROLE_ARN:?Need to set AWS_RUNNER_JOB_ROLE_ARN (e.g. arn:aws:iam::123456789012:role/ecsTaskExecutionRole)}"

IMAGE_URI="$AWS_RUNNER_IMAGE"
JOB_ROLE_ARN="$AWS_RUNNER_JOB_ROLE_ARN"

read -r -d '' CONTAINER_JSON <<EOF
{
  "image": "${IMAGE_URI}",
  "vcpus": 1,
  "memory": 1024,
  "command": [],
  "environment": [],
  "jobRoleArn": "${JOB_ROLE_ARN}"
}
EOF

aws batch register-job-definition --job-definition-name aws-runner \
  --type container \
  --container-properties "${CONTAINER_JSON}"

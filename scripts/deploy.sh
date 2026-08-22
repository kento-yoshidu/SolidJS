#!/usr/bin/env bash
set -euo pipefail

# Usage: S3_BUCKET=my-bucket ./scripts/deploy.sh
# Optional: AWS_PROFILE, AWS_REGION can be set as usual for the AWS CLI.
# Values can also be placed in a .env file at the repo root (see .env.example).

cd "$(dirname "$0")/.."

if [ -f .env ]; then
  set -a
  source ./.env
  set +a
fi

: "${S3_BUCKET:?Set S3_BUCKET to the target bucket name, e.g. S3_BUCKET=my-bucket ./scripts/deploy.sh (or add it to .env)}"

cargo run --release

aws s3 sync dist/ "s3://${S3_BUCKET}" --delete

if [ -n "${CLOUDFRONT_DISTRIBUTION_ID:-}" ]; then
  aws cloudfront create-invalidation --distribution-id "${CLOUDFRONT_DISTRIBUTION_ID}" --paths "/*"
fi

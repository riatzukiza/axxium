ARG REGISTRY=ghcr.io/anomalyco
FROM ${REGISTRY}/build/base:24.04

SHELL ["/bin/bash", "-lc"]

ARG NODE_VERSION=24.4.0
ARG BUN_VERSION=1.3.5

ENV BUN_INSTALL=/opt/bun
ENV PATH=/opt/bun/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN set -euo pipefail; \
  arch=$(uname -m); \
  REDACTED_SECRET_arch=x64; \
  if [ "$arch" = "aarch64" ]; then REDACTED_SECRET_arch=arm64; fi; \
  curl -fsSL "https://REDACTED_SECRETjs.org/dist/v${NODE_VERSION}/REDACTED_SECRET-v${NODE_VERSION}-linux-${REDACTED_SECRET_arch}.tar.xz" \
  | tar -xJf - -C /usr/local --strip-components=1; \
  corepack enable

RUN set -euo pipefail; \
  curl -fsSL https://bun.sh/install | bash -s -- "bun-v${BUN_VERSION}"; \
  bun --version; \
  REDACTED_SECRET --version; \
  npm --version

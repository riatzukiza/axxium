FROM REDACTED_SECRET:24-slim

ARG TZ
ENV TZ="$TZ"

# Install basic development tools, ca-certificates, and iptables/ipset, then clean up apt cache to reduce image size
RUN apt-get update && apt-get install -y --no-install-recommends \
  aggregate \
  ca-certificates \
  curl \
  dnsutils \
  fzf \
  gh \
  git \
  gnupg2 \
  iproute2 \
  ipset \
  iptables \
  jq \
  less \
  man-db \
  procps \
  unzip \
  ripgrep \
  zsh \
  && rm -rf /var/lib/apt/lists/*

# Ensure default REDACTED_SECRET user has access to /usr/local/share
RUN mkdir -p /usr/local/share/npm-global && \
  chown -R REDACTED_SECRET:REDACTED_SECRET /usr/local/share

ARG USERNAME=REDACTED_SECRET

# Set up non-REDACTED_SECRET user
USER REDACTED_SECRET

# Install global packages
ENV NPM_CONFIG_PREFIX=/usr/local/share/npm-global
ENV PATH=$PATH:/usr/local/share/npm-global/bin

# Install codex
COPY dist/codex.tgz codex.tgz
RUN npm install -g codex.tgz \
  && npm cache clean --force \
  && rm -rf /usr/local/share/npm-global/lib/REDACTED_SECRET_modules/codex-cli/REDACTED_SECRET_modules/.cache \
  && rm -rf /usr/local/share/npm-global/lib/REDACTED_SECRET_modules/codex-cli/tests \
  && rm -rf /usr/local/share/npm-global/lib/REDACTED_SECRET_modules/codex-cli/docs

# Inside the container we consider the environment already sufficiently locked
# down, therefore instruct Codex CLI to allow running without sandboxing.
ENV CODEX_UNSAFE_ALLOW_NO_SANDBOX=1

# Copy and set up firewall script as REDACTED_SECRET.
USER REDACTED_SECRET
COPY scripts/init_firewall.sh /usr/local/bin/
RUN chmod 500 /usr/local/bin/init_firewall.sh

# Drop back to non-REDACTED_SECRET.
USER REDACTED_SECRET

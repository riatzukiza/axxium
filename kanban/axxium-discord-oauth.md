---
uuid: axxium-discord-oauth
title: "Add Discord OAuth to Axxium"
status: todo
priority: P0
labels: ["auth", "oauth", "discord", "axxium"]
created_at: "2026-06-02T00:00:00Z"
source: "axxium/kanban/axxium-discord-oauth.md"
points: 5
category: auth
---

# Add Discord OAuth to Axxium

## Goal
Enable users to create accounts and authenticate with Discord OAuth, alongside existing email/password auth.

## Requirements
- [ ] Discord OAuth2 callback route (`/api/auth/discord/callback`)
- [ ] Discord user info fetch (username, avatar, guild memberships)
- [ ] Link Discord identity to Axxium actor (create or link existing)
- [ ] Store Discord access token for API calls (guild role checks)
- [ ] UI: Discord login button on portal
- [ ] Policy: Derive capabilities from Discord guild roles

## Acceptance Criteria
- Users can sign up/login with Discord
- Discord guild membership is stored on actor
- Policy engine can check Discord roles for authorization decisions
- Bearer tokens work for API access after OAuth login

## Related
- Axxium auth routes: `src/cljs/axxium/routes/auth.cljs`
- Axxium session: `src/cljs/axxium/auth/session.cljs`
- Axxium config: OAuth client ID/secret in env/config

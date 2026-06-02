# Π fork-tax promotion snapshot — 20260602T163545Z

- Repository: riatzukiza/devel
- Branch: pi/fork-tax/20260529T022118Z-main-softreset-all-dirt
- Branch pre-merge head: d5c41bffda04fd4f3065c48b4010dfc18f7a2612
- Rewritten main head before promotion: 072dd5d6dac973a28288e2b4ba1566b06bac92b2
- Rewritten staging head merged with ours-strategy parent: db99be843ea94458ce73461f8b57de97da135b97
- Secret history gate: targeted filter-repo replacement removed redacted token/API-key/password values from main, staging, and this branch; two binary artifacts with retained old values were purged from reachable history.
- Merge policy: `git merge -s ours --no-ff origin/staging` to record staging ancestry while preserving the fork-tax all-dirt branch tree as the promotion source of truth.
- Verification: exact old-value scan for replacement values with length >=12 returned zero hits on rewritten main, staging, and branch histories before this merge; final scan must run after refs are promoted.
- Notes: raw secret values were not printed or committed; upstream credentials represented by the redacted values should still be rotated/revoked outside git.

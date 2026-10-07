# Current primary-source protocol facts

Browser verification during this planning session, not a provider request or
SDK execution. These are source facts informing proposed decisions; they do not
prove the actual app registration, credentials, redirects or live API consent.

- [Discord OAuth2](https://docs.discord.com/developers/topics/oauth2): authorization
  and token/revocation endpoints, form-encoded code/refresh/revoke operations,
  protected transaction state, grant scopes/expiry and refresh lifecycle.
- [Discord User resource](https://docs.discord.com/developers/resources/user):
  identify permits current-user identity; guilds yields partial guilds;
  guilds.members.read permits current-user member lookup for a selected guild.
- [Discord Guild resource](https://docs.discord.com/developers/resources/guild#guild-member-object):
  member roles are Snowflake role IDs. Profile names are not identity keys or
  local capabilities.
- [RFC 9700](https://www.rfc-editor.org/rfc/rfc9700.html): transaction, redirect,
  PKCE/application-class and mix-up controls guide the security review. Sending
  an unsupported challenge is not a verified PKCE defense; actual provider
  support and the chosen registered app contract remain review prerequisites.

The inspected general Discord OAuth2 page and Social SDK account-linking page
have no PKCE text match. That is a documentation observation, not a claim that
Discord universally lacks PKCE. No implicit grant or Social SDK integration is
adopted. Registered confidential server code flow, actual supported PKCE/S256
behavior, least-privilege three-scope consent and secret storage remain proposed
review decisions. Exact full future behavior is in the design/card; none was
verified with real tokens or external accounts here.

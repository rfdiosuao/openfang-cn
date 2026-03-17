---
name: oauth-expert
description: "OAuth 2.0 and OpenID Connect expert for authorization flows, PKCE, and token management"
---
# OAuth and OpenID Connect Expert

An identity and access management specialist with deep expertise in OAuth 2.0, OpenID Connect, and token-based authentication architectures. This skill provides guidance for implementing secure authorization flows, token lifecycle management, and identity federation patterns across web applications, mobile apps, SPAs, and machine-to-machine services.

## Key Principles

- Always use the Authorization Code flow with PKCE for public clients (SPAs, mobile apps, CLI tools); the implicit flow is deprecated and insecure
- Validate every JWT thoroughly: check the signature algorithm, issuer (iss), audience (aud), expiration (exp), and not-before (nbf) claims before trusting its contents
- Design scopes to represent specific permissions (read:documents, write:orders) rather than broad roles; fine-grained scopes enable least-privilege access
- Store tokens securely: HTTP-only secure cookies for web apps, secure storage APIs for mobile, and encrypted credential stores for server-side services
- Treat refresh tokens as highly sensitive credentials; bind them to the client, rotate on use, and set reasonable absolute expiration times

## Techniques

- Implement Authorization Code + PKCE: generate a random code_verifier, derive code_challenge via S256, send the challenge in the authorize request, and send the verifier in the token exchange
- Use Client Credentials flow for server-to-server authentication where no user context is needed; scope the resulting token narrowly
- Configure token refresh with sliding window expiration: issue short-lived access tokens (5-15 minutes) with longer refresh tokens (hours to days), rotating the refresh token on each use
- Implement OIDC by requesting the openid scope; validate the id_token signature and claims, then use the userinfo endpoint for additional profile data
- Set up the Backend-for-Frontend (BFF) pattern for SPAs: the BFF server handles the OAuth flow and stores tokens in HTTP-only cookies, keeping tokens out of JavaScript entirely
- Implement token revocation by calling the revocation endpoint on logout and maintaining a server-side deny list for JWTs that must be invalidated before expiration

## Common Patterns

- **Multi-tenant Identity**: Use the issuer and tenant claims to route token validation to the correct identity provider, supporting customers who bring their own IdP
- **Step-up Authentication**: Request additional authentication factors (MFA) when accessing sensitive operations by checking the acr claim and initiating a new auth flow if insufficient
- **Token Exchange**: Use the OAuth 2.0 Token Exchange (RFC 8693) for service-to-service delegation, allowing a backend to obtain a narrowly-scoped token on behalf of the original user
- **Device Authorization Flow**: For input-constrained devices (TVs, CLI tools), use the device code grant where the user authorizes on a separate device with a browser

## Pitfalls to Avoid

- Do not store access tokens or refresh tokens in localStorage; they are vulnerable to XSS attacks and accessible to any JavaScript on the page
- Do not skip the state parameter in authorization requests; it prevents CSRF attacks by binding the request to the user session
- Do not accept tokens without validating the audience claim; a token issued for one API should not be accepted by a different API
- Do not implement custom cryptographic token formats; use well-tested JWT libraries and standard OAuth/OIDC specifications

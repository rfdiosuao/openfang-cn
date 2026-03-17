---
name: openapi-expert
description: "OpenAPI/Swagger expert for API specification design, validation, and code generation"
---
# OpenAPI Expert

An API design architect with deep expertise in the OpenAPI Specification, RESTful API conventions, and the tooling ecosystem for validation, documentation, and code generation. This skill provides guidance for designing clear, consistent, and evolvable API contracts using OpenAPI 3.0 and 3.1, covering schema composition, security definitions, versioning strategies, and developer experience optimization.

## Key Principles

- Design the API specification before writing implementation code; the spec serves as the contract between frontend, backend, mobile, and third-party consumers
- Use $ref extensively to define reusable schemas, parameters, and responses in the components section; duplication across paths leads to inconsistency and maintenance burden
- Version your API explicitly through URL path prefixes (/v1/, /v2/) or custom headers; never break existing consumers by changing response shapes without a version boundary
- Write meaningful descriptions for every path, parameter, schema property, and response; the spec doubles as your API documentation and should be understandable without reading source code
- Validate the spec in CI using linting tools to catch breaking changes, missing descriptions, inconsistent naming, and schema errors before they reach production

## Techniques

- Structure the OpenAPI document with info (title, version, contact), servers (base URLs per environment), paths (endpoints), and components (schemas, securitySchemes, parameters, responses)
- Compose schemas using allOf for inheritance (base object + extension), oneOf for polymorphism (exactly one match), and anyOf for flexible unions (at least one match)
- Provide request and response examples at both the schema level and the media type level; tools like Swagger UI and Redoc render these prominently for developer reference
- Define security schemes (Bearer JWT, API key, OAuth2 flows) in components/securitySchemes and apply them globally or per-operation with the security field
- Distinguish path parameters (/users/{id}), query parameters (?page=2&limit=20), and header parameters for different use cases; path parameters identify resources, query parameters filter or paginate
- Implement consistent pagination with limit/offset or cursor-based patterns, documenting the pagination metadata schema (total, next_cursor, has_more) in a reusable component
- Generate server stubs and client SDKs using openapi-generator with language-specific templates; customize templates for your coding conventions

## Common Patterns

- **Error Response Schema**: Define a reusable error object with code (machine-readable string), message (human-readable), and details (array of field-level errors) applied consistently across all error responses
- **Polymorphic Responses**: Use discriminator with oneOf to model responses that can be different types (e.g., a notification that is either an EmailNotification or PushNotification) with a type field
- **Pagination Envelope**: Wrap list responses in a standard envelope with data (array of items), pagination (cursor or offset metadata), and optional meta (total count, timing)
- **Webhook Definitions**: Use the webhooks section (OpenAPI 3.1) to document callback payloads your API sends to consumers, specifying the event schema and expected acknowledgment

## Pitfalls to Avoid

- Do not use additionalProperties: true by default; it makes schemas permissive and hides unexpected fields that may cause client parsing issues
- Do not define inline schemas for every request and response body; extract them to components/schemas with descriptive names for reuse and clarity
- Do not mix naming conventions (camelCase and snake_case) within the same API; pick one convention and enforce it with a linter rule
- Do not skip providing enum descriptions; raw enum values like "PENDING", "ACTIVE", "SUSPENDED" need documentation explaining what each state means and what transitions are valid

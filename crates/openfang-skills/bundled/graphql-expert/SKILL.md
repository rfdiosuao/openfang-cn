---
name: graphql-expert
description: "GraphQL expert for schema design, resolvers, subscriptions, and performance optimization"
---
# GraphQL Expert

A backend API architect with deep expertise in GraphQL schema design, resolver implementation, real-time subscriptions, and query performance optimization. This skill provides guidance for building robust, well-typed GraphQL APIs that scale efficiently while maintaining an excellent developer experience for API consumers.

## Key Principles

- Design schemas around the domain model, not the database schema; GraphQL types should represent business concepts with clear relationships
- Use input types for mutations and keep query arguments minimal; complex filtering belongs in dedicated input types
- Prevent the N+1 query problem proactively by implementing DataLoader patterns for every resolver that accesses a data source
- Treat the schema as a contract; use deprecation directives before removing fields and version through additive changes rather than breaking ones
- Enforce query complexity limits and depth restrictions at the server level to prevent abusive or accidentally expensive queries

## Techniques

- Define types with clear nullability: non-null (String!) for required fields, nullable for fields that may genuinely be absent
- Implement resolvers that return promises and batch data access; use DataLoader to batch and cache database calls within a single request
- Set up subscriptions over WebSocket (graphql-ws protocol) with proper connection lifecycle handling (init, ack, keep-alive, terminate)
- Use fragments to share field selections across queries and reduce duplication in client-side code
- Apply custom directives (@auth, @deprecated, @cacheControl) for cross-cutting concerns like authorization and cache hints
- Implement cursor-based pagination following the Relay connection specification (edges, nodes, pageInfo with hasNextPage and endCursor)
- Structure error responses with extensions field for error codes and machine-readable metadata alongside human-readable messages

## Common Patterns

- **Schema Federation**: Split a monolithic schema into domain-specific subgraphs that compose into a unified supergraph via a gateway, enabling independent team ownership
- **Persisted Queries**: Hash and store approved queries server-side; clients send only the hash, reducing bandwidth and preventing arbitrary query execution
- **Optimistic UI Updates**: Design mutations to return the mutated object so clients can update their local cache immediately without a refetch
- **Batch Mutations**: Accept arrays in input types for bulk operations while returning per-item results with success/failure status for each entry

## Pitfalls to Avoid

- Do not expose raw database IDs as the primary identifier; use opaque, globally unique IDs (base64 encoded type:id) for Relay compatibility
- Do not nest resolvers deeply without complexity analysis; a query requesting 5 levels of nested connections can explode into millions of database rows
- Do not return generic error strings; structure errors with codes, paths, and extensions so clients can programmatically handle different failure modes
- Do not skip input validation in resolvers; even though the schema enforces types, business rules like max lengths and allowed values need explicit checks

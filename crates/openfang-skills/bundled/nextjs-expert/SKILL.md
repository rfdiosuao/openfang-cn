---
name: nextjs-expert
description: "Next.js expert for App Router, SSR/SSG, API routes, middleware, and deployment"
---
# Next.js Expert

A seasoned Next.js architect with deep expertise in the App Router paradigm, server-side rendering strategies, and production deployment patterns. This skill provides guidance on building performant, SEO-friendly web applications using Next.js 14+ conventions, including Server Components, Streaming, and the full spectrum of data fetching and caching mechanisms.

## Key Principles

- Prefer Server Components by default; only add "use client" when the component requires browser APIs, event handlers, or React state
- Leverage the app/ directory structure where each folder segment maps to a URL route, using layout.tsx for shared UI and page.tsx for unique content
- Design data fetching at the server layer using async Server Components and fetch with Next.js caching semantics
- Use generateStaticParams for static pre-rendering of dynamic routes at build time, falling back to on-demand ISR for long-tail pages
- Keep client bundles small by pushing logic into Server Components and using dynamic imports for heavy client-only libraries

## Techniques

- Structure routes with app/[segment]/page.tsx, using route groups (parentheses) to organize without affecting URL paths
- Implement loading.tsx and error.tsx boundaries at each route segment to provide instant loading states and graceful error recovery
- Use Route Handlers (app/api/.../route.ts) with exported GET, POST, PUT, DELETE functions for API endpoints
- Configure middleware in middleware.ts at the project root with a matcher config to intercept requests for auth, redirects, or header injection
- Optimize images with next/image (automatic srcSet, lazy loading, AVIF/WebP) and fonts with next/font (zero layout shift, self-hosted subsets)
- Enable ISR by returning revalidate values from fetch calls or using revalidatePath/revalidateTag for on-demand cache invalidation
- Set up next.config.js with redirects, rewrites, headers, and the experimental options appropriate to your deployment target

## Common Patterns

- **Parallel Routes**: Use @named slots in layouts to render multiple page-level components simultaneously, enabling dashboards and split views
- **Intercepting Routes**: Place (..) convention routes to show modals on navigation while preserving the direct URL as a full page
- **Server Actions**: Define async functions with "use server" for form submissions and mutations without building separate API routes
- **Streaming with Suspense**: Wrap slow data-fetching components in Suspense boundaries to stream HTML progressively and improve TTFB

## Pitfalls to Avoid

- Do not use useEffect for data fetching in Server Components; fetch directly in the component body or use server-side utilities
- Do not place "use client" at the layout level unless every child truly requires client interactivity, as this opts out the entire subtree from server rendering
- Do not confuse the Pages Router (pages/ directory) patterns with App Router conventions; they have different data fetching and routing models
- Do not skip setting proper cache headers and revalidation times, as stale data and unnecessary re-renders degrade both performance and user experience

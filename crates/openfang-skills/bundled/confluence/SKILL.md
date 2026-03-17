---
name: confluence
description: "Confluence wiki expert for page structure, spaces, macros, and content organization"
---
# Confluence Expert

A technical documentation specialist with deep experience organizing knowledge bases, team wikis, and project documentation in Confluence. This skill provides guidance for structuring spaces, designing page hierarchies, leveraging macros effectively, and using the Confluence REST API for automation, ensuring that documentation remains discoverable, maintainable, and useful.

## Key Principles

- Structure spaces around teams or projects, not individuals; each space should have a clear owner and a defined scope of content
- Design page hierarchies no more than 3-4 levels deep; deeply nested pages become difficult to navigate and are rarely discovered by readers
- Use labels consistently across spaces to create cross-cutting taxonomies; labels power search, reporting, and content-by-label macros
- Write for scanning: use headings, bullet points, status macros, and expand sections so readers can quickly find what they need without reading entire pages
- Maintain content hygiene with regular reviews; assign page owners and archive stale documentation to prevent knowledge rot

## Techniques

- Create space home pages with a clear navigation structure using the Children Display macro, Content by Label macro, and pinned links to key pages
- Use the Page Properties macro with Page Properties Report to build structured databases across pages (e.g., runbook registries, decision logs)
- Format content with Info, Warning, Note, and Tip panels to visually distinguish different types of information
- Build tables with the Table of Contents macro for long pages and the Excerpt Include macro to reuse content snippets across multiple pages
- Apply page templates at the space level for consistent formatting of recurring document types (meeting notes, ADRs, postmortems)
- Automate content management through the REST API: GET /rest/api/content for search, POST for page creation, and PUT for updates using storage format XHTML
- Set granular permissions at the space and page level; restrict sensitive pages (HR, security) while keeping general documentation open

## Common Patterns

- **Decision Log**: A parent page with a Page Properties Report that aggregates status, date, and decision summary from child pages, each created from an ADR template
- **Runbook Registry**: Use Page Properties on each runbook page with fields like service, severity, and last-reviewed-date, then aggregate with a Report macro on the index page
- **Meeting Notes Series**: Create a parent page per recurring meeting with child pages auto-titled by date, using a template that includes attendees, agenda, action items, and decisions
- **Knowledge Base Landing**: Design a dashboard page with column layouts, Content by Label macros for each category, and a search panel for self-service discovery

## Pitfalls to Avoid

- Do not create orphan pages without parent context; every page should be reachable through the space navigation hierarchy
- Do not embed large files (videos, binaries) directly in pages; link to external storage or use the Confluence file list with managed attachments
- Do not duplicate content across pages; use Excerpt Include or page links to maintain a single source of truth
- Do not skip setting page restrictions on sensitive content; Confluence defaults to space-level permissions, which may be too broad for certain documents

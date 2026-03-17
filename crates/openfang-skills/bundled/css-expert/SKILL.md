---
name: css-expert
description: "CSS expert for flexbox, grid, animations, responsive design, and modern layout techniques"
---
# CSS Expert

A front-end layout specialist with deep command of modern CSS, from flexbox and grid to container queries and cascade layers. This skill provides precise, standards-compliant guidance for building responsive, accessible, and maintainable user interfaces using the latest CSS specifications and best practices.

## Key Principles

- Use flexbox for one-dimensional layouts (rows or columns) and CSS Grid for two-dimensional layouts (rows and columns simultaneously)
- Embrace custom properties (CSS variables) for theming, spacing scales, and any value that repeats or needs runtime adjustment
- Design mobile-first with min-width media queries, layering complexity as viewport size increases
- Prefer logical properties (inline-start, block-end) over physical ones (left, bottom) for internationalization-ready layouts
- Leverage the cascade intentionally with @layer declarations to control specificity without resorting to !important

## Techniques

- Use flexbox justify-content and align-items for main-axis and cross-axis alignment; flex-wrap with gap for fluid card layouts
- Define CSS Grid layouts with grid-template-areas for named regions, and auto-fit/auto-fill with minmax() for responsive grids without media queries
- Create design tokens as custom properties on :root (--color-primary, --space-md) and override them in scoped selectors or media queries
- Use @container queries to style components based on their parent container size rather than the viewport
- Build animations with @keyframes and animation shorthand; prefer transform and opacity for GPU-accelerated, jank-free motion
- Apply transitions on interactive states (hover, focus-visible) with appropriate duration (150-300ms) and easing functions
- Use the :has() selector for parent-aware styling, :is()/:where() for grouping selectors with controlled specificity

## Common Patterns

- **Holy Grail Layout**: CSS Grid with grid-template-rows (auto 1fr auto) and grid-template-columns (sidebar content sidebar) for header/footer/sidebar page structures
- **Fluid Typography**: clamp(1rem, 2.5vw, 2rem) for font sizes that scale smoothly between minimum and maximum values without breakpoints
- **Aspect Ratio Boxes**: Use the aspect-ratio property directly instead of the legacy padding-bottom hack for responsive media containers
- **Dark Mode Toggle**: Define color tokens as custom properties, swap them inside a prefers-color-scheme media query or a data-theme attribute selector

## Pitfalls to Avoid

- Do not use fixed pixel widths for layout containers; prefer percentage, fr units, or min/max constraints for fluid responsiveness
- Do not stack z-index values arbitrarily; establish a z-index scale in custom properties and document each layer's purpose
- Do not rely on vendor prefixes without checking current browser support; tools like autoprefixer handle this systematically
- Do not nest selectors excessively in preprocessors, as the generated CSS becomes highly specific and difficult to maintain or override

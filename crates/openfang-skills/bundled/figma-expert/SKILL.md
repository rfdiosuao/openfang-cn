---
name: figma-expert
description: "Figma design expert for components, auto-layout, design systems, and developer handoff"
---
# Figma Expert

A product designer and design systems architect with deep expertise in Figma's component system, auto-layout, prototyping, and developer handoff workflows. This skill provides guidance for building scalable design systems, creating maintainable component libraries, and ensuring smooth collaboration between designers and engineers through precise specifications and token-driven design.

## Key Principles

- Build components with auto-layout from the start; it ensures consistent spacing, responsive resizing, and alignment with how CSS flexbox renders in production
- Use variants and component properties to reduce component sprawl; a single button component with size, state, and icon properties replaces dozens of separate frames
- Establish design tokens (colors, typography, spacing, radii) as Figma variables and reference them everywhere instead of hardcoding values
- Separate styles (visual appearance) from variables (semantic tokens); variables enable theming and mode switching (light/dark, brand A/brand B)
- Design with real content and edge cases; placeholder text hides layout issues that surface when actual data varies in length and complexity

## Techniques

- Configure auto-layout with padding (top, right, bottom, left), gap between items, and primary axis alignment (packed, space-between) for flexible container behavior
- Create component variants using the variant property panel: define axes like Size (sm, md, lg), State (default, hover, disabled), and Type (primary, secondary)
- Define a type scale using Figma text styles with consistent size, weight, and line-height ratios; map them to semantic names (heading-lg, body-md, caption)
- Build interactive prototypes with smart animate transitions between component variants for micro-interaction demonstrations
- Use the Figma Plugin API to automate repetitive tasks: batch-renaming layers, generating color palettes, or exporting design tokens to JSON
- Leverage Dev Mode for handoff: inspect spacing, export assets, and copy CSS/iOS/Android code snippets directly from the design
- Structure design system files with a cover page, a changelog page, and dedicated pages per component category (buttons, inputs, navigation, feedback)

## Common Patterns

- **Atomic Design Structure**: Organize the library into atoms (icons, colors, typography), molecules (inputs, badges), organisms (cards, headers), and templates (page layouts)
- **Theme Switching**: Use Figma variable modes to define light and dark color sets; components reference semantic variables that resolve differently per mode
- **Responsive Components**: Use auto-layout with fill-container width and min/max constraints to create components that adapt across breakpoints without separate mobile variants
- **Documentation Pages**: Embed component instances alongside usage guidelines, do/don't examples, and property tables directly in the Figma file for designer self-service

## Pitfalls to Avoid

- Do not use absolute positioning inside auto-layout frames unless the element genuinely needs to break out of flow; it defeats the purpose of responsive layout
- Do not create one-off detached instances when a variant or property would serve the use case; detached instances become stale when the source component updates
- Do not skip naming and organizing layers; engineers inspecting in Dev Mode rely on meaningful layer names to map designs to code components
- Do not embed raster images at full resolution without optimizing; large assets slow down Figma file performance and create unnecessarily heavy exports

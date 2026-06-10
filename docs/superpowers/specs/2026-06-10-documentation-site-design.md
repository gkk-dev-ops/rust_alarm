# Documentation Site Design

## Goal

Create a GitHub Pages documentation site for `clck` that combines a restrained,
branded landing page with clear CLI guides and reference material.

The site should feel at home beside a terminal application while remaining easy
to navigate and maintain inside the existing Rust repository.

## Technology

Use VitePress as the static site generator.

VitePress provides the required documentation navigation, built-in local
search, code highlighting, responsive layouts, and a customizable homepage
without requiring a bespoke frontend application. It introduces a small Node
toolchain isolated under `docs-site/`.

The site will be deployed to GitHub Pages through a dedicated GitHub Actions
workflow.

## Repository Layout

Keep public site sources separate from existing internal project documentation:

```text
docs-site/
  .vitepress/
    config.mts
    theme/
      index.ts
      custom.css
  development/
    releases.md
    testing.md
  guide/
    configuration.md
    installation.md
    scheduling.md
    usage.md
  reference/
    commands.md
    controls.md
    duration-formats.md
  index.md
  package.json
  package-lock.json
```

The existing `docs/` directory remains the source of internal design and
planning documents. Relevant information from `README.md`,
`docs/manual-testing.md`, and `docs/releases.md` will be adapted into the public
site rather than imported automatically.

## Information Architecture

The homepage introduces the tool and gives visitors a direct path to install or
start using it.

The documentation navigation is divided into three sections:

- **Guide:** installation, basic usage, scheduling, and configuration.
- **Reference:** commands, accepted duration formats, and runtime controls.
- **Development:** testing and release operations.

The header links to the guide, command reference, and GitHub repository. The
sidebar groups pages by the same three sections. Local search is enabled.

## Homepage

The homepage uses a custom VitePress home layout with:

1. A concise title and description.
2. Primary actions for quick start and GitHub.
3. A terminal-style example showing a PowerShell prompt and a typical `clck`
   command.
4. A compact feature grid covering flexible scheduling, terminal display,
   configurable alarms, and cross-platform support.
5. Installation examples for Cargo and prebuilt GitHub Release binaries.

The homepage remains documentation-first. It will not include testimonials,
large illustrations, pricing-style sections, or decorative marketing content.

## Visual Design

Use a restrained terminal-inspired visual system influenced by Ghostty's
minimalism and PowerShell's blue palette.

- Near-black and deep navy surfaces.
- Muted blue-gray text and borders.
- PowerShell blue as the primary accent and focus color.
- Monospace typography for headings, navigation labels, commands, and terminal
  examples, with a readable system sans-serif fallback for body copy.
- Square or subtly rounded corners.
- Minimal shadows and no decorative gradients.
- Little or no animation beyond standard interaction transitions.

The theme must maintain readable contrast in both dark and light modes, with
dark mode as the default presentation.

## Content Source and Maintenance

Initial documentation is derived from behavior already documented in the
repository and exposed by the CLI:

- Installation methods and platform support from `README.md`.
- Commands and flags from the CLI definitions.
- Scheduling and duration formats from CLI behavior and existing tests.
- Development instructions from existing manual testing and release documents.

Content is maintained manually in Markdown. Automatic command-reference
generation is out of scope for the first version because it would add release
and synchronization complexity disproportionate to the current CLI surface.

## Deployment

Add a GitHub Actions workflow that:

1. Runs for relevant changes pushed to `master` and by manual dispatch.
2. Installs the pinned Node dependencies with `npm ci`.
3. Builds the VitePress site.
4. Uploads the generated artifact.
5. Deploys it using GitHub Pages' official deployment actions.

The VitePress base path is `/clck/`, matching the project Pages URL. The
workflow receives the minimum Pages and identity-token permissions required for
deployment.

## Verification

CI should run the VitePress build alongside the existing Rust checks when site
sources or workflow files change. The implementation is complete when:

- `npm ci` and the documentation build succeed from `docs-site/`.
- All internal links resolve during the VitePress build.
- The homepage and documentation navigation work at the `/clck/` base path.
- The layout remains usable at desktop and mobile widths.
- The Pages workflow passes GitHub Actions syntax validation.

## Out of Scope

- A custom domain.
- Hosted analytics.
- Versioned documentation.
- Automated CLI reference generation.
- Internationalization.
- A blog or changelog section.

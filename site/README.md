# site

Source for the showcase site at `https://garage.udcio.com`, built with [Zola](https://www.getzola.org/).

The homepage shows a grid of project cards. Each card comes from the front matter of a page in
`content/projects/`: `title`, `description`, `weight` (card order), and `extra.standards` and
`extra.status`. To add a project, add a page there.

Each project page renders from `templates/project.html` with four sections:

- **What it is:** the `description`, `extra.standards` and `extra.status`.
- **Live demo:** a project with a demo sets `template` in its front matter to its own template,
  which extends `project.html` and overrides the `demo` block.
- **How it works and how it breaks:** the Markdown body of the page.
- **Code:** set `extra.code` to the app's path, such as `apps/aead`, once it exists.

A section with nothing to show yet says so.

The site loads nothing from other origins: no external scripts, fonts or stylesheets.

## Build

From this directory:

```sh
zola serve   # local preview at http://127.0.0.1:1111
zola build   # writes the site to public/
zola check   # checks templates, internal links and external links
```

Deployment to GitHub Pages is described in [docs/deployment-plan.md](../docs/deployment-plan.md).

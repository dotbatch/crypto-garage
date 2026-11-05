# site

Source for the showcase site at `https://garage.udcio.com`, built with [Zola](https://www.getzola.org/).

The homepage shows a grid of project cards. Each card comes from the front matter of a page in
`content/projects/`: `title`, `description`, `weight` (card order), and `extra.standards` and
`extra.status`. To add a project, add a page there.

The site loads nothing from other origins: no external scripts, fonts or stylesheets.

## Build

From this directory:

```sh
zola serve   # local preview at http://127.0.0.1:1111
zola build   # writes the site to public/
zola check   # checks templates, internal links and external links
```

Deployment to GitHub Pages is described in [docs/deployment-plan.md](../docs/deployment-plan.md).

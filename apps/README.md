# apps

Showcase apps, one folder per project. Each app is a crate in the root Cargo workspace and has its own README naming the standard it follows (FIPS, SP, RFC or ISO, with the version). Start each README from the [app README template](../docs/app-template.md).

Apps use vetted libraries only. No home-grown primitives here; those belong in [`learn/`](../learn/).

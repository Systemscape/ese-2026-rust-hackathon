---
title: Hardware CI
---

# Hardware CI

The [workflow](https://github.com/Systemscape/ese-2026-rust-hackathon/blob/main/.github/workflows/pcb.yaml) builds on pull requests, pushes to
`main`, and manual runs. It uses the same KiCad 10 / KiBot container as the OnMCU
hardware repository, with one board and one complete
[KiBot config](https://github.com/Systemscape/ese-2026-rust-hackathon/blob/main/hardware/config.kibot.yaml).

Add the board's matching `.kicad_pro`, `.kicad_sch` and `.kicad_pcb` files at
`hardware/ese-rust-board/ese-rust-board.*`. Commit any referenced sheets, custom
libraries, library tables and 3D models too, using project-relative paths.
If the board moves, update `PCB` in the workflow and the exclusion in `_config.yml`.
When all three sources are absent, CI publishes documentation only; a partially
added board fails the build.

KiBot runs ERC and DRC, then generates schematic and PCB PDFs, a CSV BOM,
interactive BOM, Gerbers and drills, pick-and-place files, PCB drawings, SVGs,
STEP, 3D-printable stencils and netlists. Errors fail the build and block
deployment. Available check reports and partial exports are uploaded even on
failure. The `pcb-outputs` artifact is retained for 90 days; `site-preview`
contains the rendered site for review on pull requests.

GitHub Pages uses Jekyll to render the README files and other Markdown. The
downloads page lists the generated files automatically. Only successful runs
on `main` deploy the public site; pull requests do not deploy.

In repository **Settings → Pages → Build and deployment**, set **Source** to
**GitHub Actions** before the first deployment. No additional secrets are needed.
The workflow follows GitHub's [Jekyll build](https://github.com/actions/jekyll-build-pages)
and [Pages deployment](https://github.com/actions/deploy-pages) actions.

To generate the same PCB exports locally with KiCad 10 and KiBot installed:

```sh
kibot -c hardware/config.kibot.yaml \
  -b hardware/ese-rust-board/ese-rust-board.kicad_pcb \
  -e hardware/ese-rust-board/ese-rust-board.kicad_sch -d outputs
python3 -c "import shutil; shutil.make_archive('outputs/gerbers', 'zip', 'outputs/gerber')"
```

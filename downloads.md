---
title: PCB downloads
permalink: /downloads/
---

# PCB downloads

[Hackathon](./README.md) · [Hardware](./hardware/README.md)

{% assign exports = site.static_files | where_exp: "file", "file.path contains '/downloads/'" | sort: 'path' %}
{% if exports.size > 0 %}
Exports from commit `{{ site.github.build_revision }}`.

The Gerber ZIP includes copper, mask, silkscreen, paste, board outline and drill files.

{% for file in exports %}
- [{{ file.path | remove_first: '/downloads/' }}]({{ file.path | relative_url }})
{% endfor %}
{% else %}
PCB downloads will appear here once the KiCad board sources are added and pass ERC/DRC.
{% endif %}

---
title: PCB downloads
permalink: /downloads/
---

# PCB downloads

[Hackathon](./README.md) · [Hardware](./hardware/README.md)

{% assign ibom = site.static_files | where: 'path', '/downloads/ibom/ese-rust-board-ibom.html' | first %}
{% if ibom %}
## Interactive BOM

Find components on the board and track sourcing, placement, and soldering.

<p><a href="{{ ibom.path | relative_url }}" target="_blank" rel="noopener noreferrer">Open interactive BOM (iBOM) in a new window</a></p>
{% endif %}

{% assign schematic = site.static_files | where: 'path', '/downloads/pdf/ese-rust-board-schematic.pdf' | first %}
{% if schematic %}
## Schematic

<p><a href="{{ schematic.path | relative_url }}" target="_blank" rel="noopener noreferrer">Open schematic (PDF) in a new window</a></p>

<object data="{{ schematic.path | relative_url }}" type="application/pdf" title="ESE Rust Devboard schematic" width="100%" height="700" style="height: 75vh; min-height: 400px;">
  <p>Your browser cannot display this PDF inline. <a href="{{ schematic.path | relative_url }}" target="_blank" rel="noopener noreferrer">Open the schematic in a new window</a>.</p>
</object>
{% endif %}

## All PCB exports

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

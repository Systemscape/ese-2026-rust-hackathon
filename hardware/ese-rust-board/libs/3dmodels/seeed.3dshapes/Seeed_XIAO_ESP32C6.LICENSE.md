# Seeed XIAO ESP32C6 3D model provenance and usage notice

Canonical file: `Seeed_XIAO_ESP32C6.step`.

Source: [GrabCAD Community Library — Seeed Studio XIAO ESP32-C6](https://grabcad.com/library/seeed-studio-xiao-esp32-c6-1), also linked by [Seeed's ESP32C6 documentation](https://wiki.seeedstudio.com/xiao_esp32c6_getting_started/#resources).

Original filename: `Seeed Studio XIAO ESP32-C6.step`. Supplied by the user from the downloaded `seeed-studio-xiao-esp32-c6-1.snapshot.3` folder on 2026-10-08. The STEP header records creation at `2024-11-06T17:15:41+01:00`, using Autodesk Translation Framework v13.20.0.188 / ST-DEVELOPER v20.

Creator: **Maurice Pannard**, as identified on the GrabCAD page and confirmed by the user on 2026-10-08. The STEP header itself has empty author and organization fields.

SHA-256 (original and canonical copy):
`2e1ce01f4497192485823ca9fffd68e323bafc84cd7b85c2906b5497550be8fc`

The file was copied under the canonical name without changing any bytes. Geometry, assembly structure, names, colors, and embedded metadata remain as supplied. The original download remains intact.

Alignment is stored in `XIAO-ESP32C6-Hybrid.kicad_mod`, not baked into the STEP file:

- Scale: `(1, 1, 1)`.
- KiCad model rotation: `(-90, 0, -90)` degrees.
- KiCad model offset: `(15.0013999554, 8.7593413144, 0.2499999851)` mm.

In model coordinates this maps `(x, y, z)` to `(z + 15.0013999554, x + 8.7593413144, y + 0.2499999851)`, with the underside mounting plane at Z=0. KiCad's STEP exporter adds the carrier board's mounting elevation. The attachment represents flush SMT assembly; a header/socket assembly requires the corresponding additional Z offset.

Validation: KiCad 10.0.6 exported the attached model successfully, retaining 37 solids. All 14 header-hole axes in the exported geometry match the footprint centers to 0.00001 mm. Model height above its mounting plane is 4.46 mm. This is an alignment check, not physical metrology. The footprint's underside pad locations were verified against Seeed's PCB and should remain authoritative over the visualization model.

Usage terms: see the [GrabCAD Terms of Use](https://blog.grabcad.com/terms-of-use/) and [GrabCAD model-use guidance](https://help.grabcad.com/article/246-how-can-models-be-used-and-shared). The guidance describes private use, requires original-creator attribution and a source link for public non-commercial use, and requires the designer's explicit permission for commercial public use. This notice does not grant additional rights or assert a separate open-source license. Seeed names and marks remain their owners' property.

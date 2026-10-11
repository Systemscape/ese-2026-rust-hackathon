KiCad 10 project libraries

The root fp-lib-table and sym-lib-table each include the matching table
under libs/ using type Table. Maintain the local library lists here.

Footprints: Connector_THT_Wurth, seeed.
Symbols: Sensors_ESE, Connector_Wurth_WR-WST, seeed.
Standard KiCad libraries continue to come from the global library table.
3D-model directories are referenced by footprints, not library-table rows.
Existing Wurth footprints use WE_3DMODEL_DIR; configure that path to this
project's libs/3dmodels directory when using their 3D models.

Use seeed:XIAO-ESP32C6 with seeed:XIAO-ESP32C6-Hybrid. The footprint
supports THT headers and hand-soldered SMT castellations, with plated
underside solder-access holes. No stencil paste apertures are provided.
The 3D model is Seeed_XIAO_ESP32C6.step, attributed to Maurice Pannard;
see the adjacent LICENSE.md for source, checksum and alignment details.
Model alignment represents flush SMT assembly. Add the actual header or
socket height to the model Z offset when using a raised assembly.

Paths use ${KIPRJMOD}/libs, so retain libs/ in the project root.
Adding a new library requires adding its entry to these nested tables.

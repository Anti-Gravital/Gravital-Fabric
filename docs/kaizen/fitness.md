# Architectural fitness functions / Funciones de aptitud arquitectonica

Verifiable rules for the sweep loop. Reglas verificables para el loop sweep.

- No source file exceeds 600 lines. Ningun archivo de codigo supera 600 lineas.
- No crate named `fabric-*` depends on a crate named `ag-*`.
  Ningun crate `fabric-*` depende de un crate `ag-*`.
- No unsafe code without an RFC and a code comment.
  Ningun `unsafe` sin RFC y comentario de codigo.
- Rust only, except the approved governance tooling.
  Solo Rust, salvo la herramienta de gobernanza aprobada.
- No cycles between modules of the same crate.
  Sin ciclos entre modulos del mismo crate.
- Startup time < ___ ms; p95 of operation X < ___ ms.
  Tiempo de arranque < ___ ms; p95 de la operacion X < ___ ms.

Metric thresholds are set by a person after the first benchmark.
Los umbrales los fija una persona tras el primer benchmark.

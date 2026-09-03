# Zeroth Compiler

Zeroth es un lenguaje de programación de alto rendimiento enfocado en sistemas, desarrollado como un fork especializado de rustc. Constituye el núcleo de ejecución e infraestructura de compilación para el monorepo Holtha-Space.

## Propósito

Zeroth extiende y adapta la infraestructura de rustc para integrarse nativamente con el ecosistema Herrat, ofreciendo un entorno controlado para el desarrollo de software, motores y librerías de bajo nivel.

## Arquitectura en Holtha-Space

Dentro del monorepo, zeroth actúa como el motor principal:

* Compilador / Core: zeroth/ (basado en la cadena de herramientas de Rust)
* Librería Estándar & Runtime: herrat/ (core, sys, gfx, math, input, net, audio)

## Construcción y Desarrollo

Para compilar el compilador y sus herramientas asociadas desde el monorepo:

cd zeroth
python3 x.py build

Nota: Los artefactos de compilación se generan en zeroth/build/ y están excluidos del control de versiones.

## Licencia

Al ser un derivado de rustc, Zeroth mantiene las licencias originales Apache 2.0 y MIT.
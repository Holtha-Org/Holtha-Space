# Holtha-Space

Monorrepositorio central y **Cargo Workspace** oficial de **Holtha Org**. Una arquitectura de librerías atómicas, modulares y de alto rendimiento escritas en Rust para infraestructura, gráficos, matemáticas y sistemas.

---

## 🚀 Arquitectura del Proyecto

`Holtha-Space` utiliza un diseño atómico (*micro-crates*). Cada módulo principal se divide en sub-librerías independientes para optimizar los tiempos de compilación y permitir un consumo quirúrgico de componentes.

```text
Holtha-Space/
├── core/      # Funcionalidades base, utilidades y gestión de errores
├── sys/       # Manejo de archivos, memoria y abstracciones del SO
├── gfx/       # Ventanas, renderizado y gestión de shaders
├── math/      # Álgebra lineal, vectores y matrices
├── input/     # Entrada de teclado, ratón y mandos
├── net/       # Redes, sockets y protocolos
├── audio/     # Procesamiento y mezcla de sonido
├── tools/     # Herramientas internas e infraestructura de desarrollo
├── docs/      # Documentación técnica unificada
└── examples/  # Ejemplos prácticos de integración

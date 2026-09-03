# Holtha-Space

`Holtha-Space` es el monorepo principal de **Holtha Org**. Reúne las librerías,
herramientas, documentación y componentes de infraestructura que forman el
ecosistema Holtha.

El proyecto está escrito principalmente en Rust y sigue una arquitectura modular:
cada dominio se divide en crates pequeños e independientes para facilitar el
desarrollo, las pruebas, el mantenimiento y la reutilización de código.

## Qué contiene este repositorio

```text
Holtha-Space/
├── herrat/    # Workspace y futuro gestor de paquetes de Holtha
│   ├── core/  # Tipos, errores y utilidades fundamentales
│   ├── sys/   # Archivos, información del sistema y memoria
│   ├── gfx/   # Color, ventanas y renderizado
│   ├── math/  # Geometría, vectores y matrices
│   ├── input/ # Teclado, ratón y mandos
│   ├── net/   # Comunicación TCP y UDP
│   └── audio/ # Decodificación, mezcla y reproducción de audio
├── tools/     # Herramientas internas
├── docs/      # Documentación técnica
├── examples/  # Ejemplos de uso
└── zeroth/    # Copia independiente del código fuente de Rust
```

Los directorios dentro de `herrat` contienen crates especializados. Por ejemplo,
`herrat/core/holtha-core-errors` y `herrat/math/holtha-math-vec` son componentes
separados que pueden evolucionar de forma independiente.

## Herrat

**Herrat** es el nombre del futuro gestor de paquetes, dependencias y proyectos
de Holtha. Su propósito es ofrecer para el ecosistema Holtha una experiencia
similar a la que Cargo ofrece para Rust, pero adaptada a nuestras necesidades y
convenciones.

Herrat está organizado inicialmente por dominios y sirve como base para:

- descubrir y administrar los crates de Holtha;
- resolver dependencias entre módulos;
- crear y configurar proyectos nuevos;
- coordinar versiones, compilación y publicación de paquetes;
- mantener una estructura coherente dentro del monorepo.

Herrat se encuentra en desarrollo. Actualmente, el workspace de Cargo se
configura mediante el `Cargo.toml` raíz y descubre los crates ubicados dentro de
`herrat`, mientras se define la interfaz y el funcionamiento propio de Herrat.

## Relación con Cargo

Cargo continúa siendo la herramienta de compilación y gestión de dependencias
para el código Rust durante esta etapa. Herrat no reemplaza todavía a Cargo:
funciona como la dirección futura del ecosistema Holtha y podrá integrarse con
las herramientas existentes mientras madura.

## `zeroth`

La carpeta `zeroth` contiene una copia independiente del código fuente de Rust,
incluyendo componentes del compilador, la biblioteca estándar y sus herramientas.
Los derechos de autor de ese código pertenecen a sus autores originales,
principalmente a los contribuyentes del Proyecto Rust.

La copia conserva sus licencias Apache 2.0, MIT y las licencias adicionales de
terceros. Consulta [ZEROTH-NOTICE.md](ZEROTH-NOTICE.md) y los avisos completos en
[`zeroth/COPYRIGHT`](zeroth/COPYRIGHT), [`zeroth/LICENSE-APACHE`](zeroth/LICENSE-APACHE),
[`zeroth/LICENSE-MIT`](zeroth/LICENSE-MIT) y [`zeroth/LICENSES/`](zeroth/LICENSES/).

## Estado del proyecto

Holtha-Space se encuentra en desarrollo activo. Las APIs, la organización de los
crates y el diseño de Herrat pueden cambiar mientras se construye la base del
ecosistema.

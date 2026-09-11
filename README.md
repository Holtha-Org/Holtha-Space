# Holtha-Space

`Holtha-Space` es el monorepo principal de **Holtha Org**. Reúne el núcleo del lenguaje, las herramientas, la documentación y los componentes de infraestructura que forman el ecosistema Holtha.

El proyecto sigue una arquitectura modular enfocada en la independencia, el rendimiento y una filosofía abierta. Cada dominio y componente evoluciona de forma separada para facilitar el desarrollo y el mantenimiento.

---

## Qué contiene este repositorio

    Holtha-Space/
    ├── herrat/    # Futuro gestor de paquetes de Holtha (base adaptada de Cargo)
    ├── zeroth/    # Base y núcleo del lenguaje de programación Zeroth
    ├── tools/     # Herramientas internas de desarrollo
    ├── docs/      # Documentación técnica y avisos legales
    └── examples/  # Ejemplos de uso del ecosistema

---

## Herrat

**Herrat** es el nombre del futuro gestor de paquetes, dependencias y proyectos del ecosistema. Su propósito es ofrecer una experiencia robusta de gestión, coordinar versiones y mantener la estructura de nuestros módulos, pero adaptada estrictamente a nuestras necesidades y convenciones.

Para no reinventar la rueda, el núcleo de Herrat está siendo inicializado tomando como base la arquitectura probada de **Cargo**.

Herrat se encuentra en **desarrollo activo**. Actualmente se está definiendo su estructura interna, su integración con los módulos y su futura interfaz de uso, por lo que aún no reemplaza los flujos de trabajo estándar.

---

## Somer

**Somer** es el asistente local inteligente del ecosistema. Estará integrado dentro de la arquitectura de Herrat para proporcionar diagnósticos y asistencia directa en el desarrollo de forma privada y local. Al igual que el resto de las herramientas, Somer se encuentra en fase de diseño e integración conceptual.

---

## Zeroth

**Zeroth** es nuestro lenguaje de programación independiente en construcción. La carpeta `zeroth` contiene la base para su compilador, núcleo y herramientas de desarrollo.

El proyecto está sentando las bases de su sintaxis y su propia biblioteca estándar para operar de manera autónoma dentro del ecosistema Holtha.

---

## Licencias y Atribución

Dado que estamos utilizando y adaptando bases robustas de código abierto para acelerar nuestro desarrollo, respetamos rigurosamente sus licencias originales (**MIT / Apache 2.0**):

- **Herrat:** La atribución correspondiente a la base de Cargo y las decisiones de arquitectura se encuentran documentadas en `docs/HERRAT-NOTICE.md`.
- **Zeroth:** El código base inicial conserva sus derechos y licencias de origen, detallados en la carpeta `zeroth/` y en sus respectivos archivos de aviso.

---

## Estado del proyecto

**Holtha-Space** se encuentra en **desarrollo activo**. Las APIs, la organización de los directorios y el diseño conceptual de Herrat, Zeroth y Somer cambiarán constantemente mientras se construye la base definitiva del ecosistema.
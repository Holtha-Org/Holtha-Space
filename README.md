# Holtha-Space

`Holtha-Space` es el monorepo principal de **Holtha Org**. Reúne el
núcleo del lenguaje, las herramientas, la documentación y los
componentes de infraestructura que forman el ecosistema Holtha.

El proyecto sigue una arquitectura modular enfocada en la independencia,
el rendimiento y una filosofía abierta. Cada dominio y componente
evoluciona de forma separada para facilitar el desarrollo y el
mantenimiento.

---

## Qué contiene este repositorio

    Holtha-Space/
    ├── herrat/    # Futuro gestor de paquetes de Holtha (base adaptada de Cargo)
    ├── zeroth/    # Base y núcleo del lenguaje de programación Zeroth
    ├── merath/    # Plataforma de asistente local (base adaptada de N.E.K.O.)
    ├── tools/     # Herramientas internas de desarrollo
    ├── docs/      # Documentación técnica y avisos legales
    └── examples/  # Ejemplos de uso del ecosistema

---

## Herrat

**Herrat** es el nombre del futuro gestor de paquetes, dependencias y
proyectos del ecosistema. Su propósito es ofrecer una experiencia
robusta de gestión, coordinar versiones y mantener la estructura de
nuestros módulos, pero adaptada estrictamente a nuestras necesidades
y convenciones.

Para no reinventar la rueda, el núcleo de Herrat está siendo
inicializado tomando como base la arquitectura probada de **Cargo**.

Herrat se encuentra en **desarrollo activo**.

---

## MERATH

**MERATH** es la plataforma de asistente local de Holtha Org. Hereda
la arquitectura de **N.E.K.O.** (Apache 2.0) y funciona como una
aplicación de escritorio con interfaz gráfica, avatar, chat, control
del sistema y sistema de plugins.

MERATH incorpora por defecto a **Somer**, el asistente y avatar
predeterminado del ecosistema, equivalente a la gata Yui en N.E.K.O.

MERATH se encuentra en **desarrollo activo**. Su estructura interna,
interfaz y capacidades evolucionarán conforme se adapte al ecosistema
Holtha.

---

## Somer

**Somer** es el asistente y avatar predeterminado de **MERATH**. Vive
dentro de la plataforma y es lo primero que ve el usuario al abrirla.
Somer tiene su propia personalidad, voz y conjunto de herramientas,
y puede ser adaptado o sustituido por otros asistentes conforme
evolucione el ecosistema.

---

## Zeroth

**Zeroth** es nuestro lenguaje de programación independiente en
construcción. La carpeta `zeroth` contiene la base para su compilador,
núcleo y herramientas de desarrollo.

El proyecto está sentando las bases de su sintaxis y su propia
biblioteca estándar para operar de manera autónoma dentro del
ecosistema Holtha.

---

## Licencias y Atribución

Dado que estamos utilizando y adaptando bases robustas de código
abierto para acelerar nuestro desarrollo, respetamos rigurosamente
sus licencias originales (**MIT / Apache 2.0**):

- **Herrat:** La atribución correspondiente a la base de Cargo y las
  decisiones de arquitectura se encuentran documentadas en
  `docs/HERRAT-NOTICE.md`.
- **MERATH:** La atribución correspondiente a la base de N.E.K.O. y
  las decisiones de arquitectura se encuentran documentadas en
  `merath/docs/MERATH-NOTICE.md`.
- **Zeroth:** El código base inicial conserva sus derechos y licencias
  de origen, detallados en la carpeta `zeroth/` y en sus respectivos
  archivos de aviso.

---

## Estado del proyecto

**Holtha-Space** se encuentra en **desarrollo activo**. Las APIs, la
organización de los directorios y el diseño conceptual de Herrat,
MERATH, Somer y Zeroth cambiarán constantemente mientras se construye
la base definitiva del ecosistema.
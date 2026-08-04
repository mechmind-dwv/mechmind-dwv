# 🦾 MechROS2

Capa de integración de **MechMind** con **ROS2 (Robot Operating System 2)**. Este módulo permite la comunicación asíncrona entre el hardware y los nodos de ROS2.

## 🛠️ Requisitos

- Rust (estable)
- ROS2 Humble (recomendado)
- Dependencias del sistema: `build-essential`, `pkg-config`, `libssl-dev`

## 🚀 Uso

Compilar el nodo de integración:

```bash
cargo build -p mechros2
```

Ejecutar el nodo:

```bash
cargo run -p mechros2
```

## 📋 Funcionalidades

- Procesamiento de visión en tiempo real.
- Planificación de navegación.
- Gestión de sensores y actuadores.
- Publicación de estado del sistema vía tópicos ROS2.

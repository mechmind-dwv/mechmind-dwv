# 🔌 Especificación Técnica: Puente WebSocket para `mechros2`

## 1. Resumen Ejecutivo
Para conectar el centro de mando cyberpunk (`projects/frontend`) con el nodo ROS2 (`projects/mechros2`) en tiempo real, el dashboard utiliza el protocolo **rosbridge v2** sobre WebSocket. El navegador no habla DDS directamente; `rosbridge_server` expone los tópicos ROS2 mediante mensajes JSON con un campo `op`, por ejemplo `subscribe` y `publish` [1] [2].

La implementación actual del frontend mantiene un **fallback local**: intenta conectar tres veces al puente y, si no está disponible, conserva la telemetría visual local y marca la interfaz como `UPLINK SIMULATED`. Cuando el puente vuelve a responder, el dashboard cambia automáticamente a `UPLINK LIVE`.

> **Nota de rendimiento:** para flujos de visualización de alta frecuencia o mensajes grandes, la documentación de Foxglove recomienda considerar `foxglove_bridge` como alternativa de mayor rendimiento. El cliente implementado aquí usa rosbridge v2 porque encaja directamente con los tópicos `std_msgs/msg/String` ya presentes en `mechros2` [1].

---

## 2. Contrato de Datos

### 2.1. Telemetría ROS2 ➔ Navegador

`mechros2` publica el estado serializado en `/mechros2/system_state` como `std_msgs/msg/String`. El frontend envía la operación rosbridge siguiente al abrir el socket:

```json
{
  "op": "subscribe",
  "topic": "/mechros2/system_state",
  "type": "std_msgs/msg/String"
}
```

El contenido de `msg.data` debe ser JSON con esta forma:

```json
{
  "timestamp": "2026-08-12T14:31:16Z",
  "position": { "x": 0.0, "y": 0.0, "z": 0.0 },
  "velocity": { "x": 0.42, "y": 0.0, "z": 0.0 },
  "orientation": { "roll": 0.0, "pitch": 0.0, "yaw": 0.0 },
  "battery_level": 78.5,
  "system_status": "Active",
  "core_load": 59.0,
  "signal_integrity": 92.0,
  "ai_confidence": 96.4,
  "temperature": 41.8,
  "active_nodes": 24
}
```

El dashboard acepta también los alias `load`, `signal`, `thermal`, `battery`, `nodes` y `confidence` para facilitar una migración gradual desde otros publicadores.

### 2.2. Navegador ➔ ROS2

Los controles de diagnóstico y autonomía publican comandos JSON en `/mechros2/remote_commands`:

```json
{
  "op": "publish",
  "topic": "/mechros2/remote_commands",
  "type": "std_msgs/msg/String",
  "msg": {
    "data": "{\"target_node\":\"mechros2_hub\",\"command_type\":\"DIAGNOSTIC_RUN\",\"timestamp\":1755000000}"
  }
}
```

Los comandos reconocidos inicialmente por el dashboard son `DIAGNOSTIC_RUN` y `RESUME_AUTONOMY`. El nodo ROS2 debe validar `target_node`, `command_type` y el origen antes de ejecutar cualquier acción física.

---

## 3. Arquitectura

```text
┌──────────────────┐       WebSocket / rosbridge v2       ┌────────────────────────┐
│  Cyber Dashboard │ ◄──────────────────────────────────► │ rosbridge_server       │
│  React + Vite    │       ws://robot-host:9090           │ ROS2 Humble / port 9090│
└──────────────────┘                                      └───────────┬────────────┘
                                                                      │ DDS / r2r
                                                         ┌────────────▼───────────┐
                                                         │      mechros2 node      │
                                                         │ system_state + commands │
                                                         └────────────────────────┘
```

En desarrollo local, el cliente usa `VITE_ROS_BRIDGE_WS_URL` si está definida y, si no, `ws://localhost:9090`. En un despliegue HTTPS se debe usar un endpoint `wss://` con TLS y no un socket `ws://` de texto plano.

---

## 4. Instalación en ROS2 Humble

El paquete `rosbridge_server` está publicado para ROS2 Humble. En la máquina que ejecuta ROS2:

```bash
sudo apt update
sudo apt install ros-humble-rosbridge-server
source /opt/ros/humble/setup.bash
ros2 launch rosbridge_server rosbridge_websocket_launch.xml port:=9090
```

Para iniciar el dashboard apuntando a un robot remoto durante el desarrollo:

```bash
export VITE_ROS_BRIDGE_WS_URL=ws://ROBOT_HOST:9090
pnpm dev
```

Para producción, configurar una URL `wss://` detrás de un proxy TLS y restringir el acceso al puerto del puente mediante red privada, VPN o una lista de clientes autorizados. No se deben publicar puertos ROS2 sin autenticación en Internet.

---

## 5. Verificación manual

1. Arrancar `rosbridge_server` y confirmar que el indicador pasa de `UPLINK SIMULATED` a `UPLINK LIVE`.
2. Publicar un mensaje de prueba en `/mechros2/system_state` con `std_msgs/msg/String` y comprobar que cambian el gráfico, batería, nodos y temperatura.
3. Pulsar **Run diagnostics** y verificar que el bridge recibe un `op: publish` para `/mechros2/remote_commands`.
4. Detener el puente y comprobar que el dashboard vuelve a `UPLINK SIMULATED` sin perder la navegación ni bloquear la interfaz.

---

## Referencias

[1]: https://foxglove.dev/blog/using-rosbridge-with-ros2 "Using Rosbridge with ROS 2 — Foxglove"
[2]: https://github.com/RobotWebTools/rosbridge_suite/blob/ros2/ROSBRIDGE_PROTOCOL.md "rosbridge v2.1.0 Protocol Specification — RobotWebTools"
[3]: https://index.ros.org/p/rosbridge_server/ "rosbridge_server package — ROS Index"

# 🗺️ Especificación Técnica: Editor de Rutas y Waypoints para MechBot-2X

## 1. Resumen Ejecutivo
El editor de rutas de navegación permite al operador interactuar con un mapa satelital/híbrido (vía la integración de Google Maps de Manus) o una cuadrícula táctica de respaldo para definir, reordenar y transmitir trayectorias autónomas (`geometry_msgs/PoseStamped` o `nav_msgs/Path`) al nodo ROS2 `mechros2` mediante el protocolo rosbridge WebSocket [1] [2].

---

## 2. Contrato de Datos ROS2

### 2.1. Publicación de Waypoints (Navegador ➔ ROS2)
Cuando el operador hace clic en "Transmitir ruta a MechBot", el cliente empaqueta los waypoints en una trama rosbridge `publish` dirigida a `/mechros2/navigation_goals`:

```json
{
  "op": "publish",
  "topic": "/mechros2/navigation_goals",
  "type": "std_msgs/msg/String",
  "msg": {
    "data": "{\"sequence_id\":\"route_1755000000\",\"waypoints\":[{\"index\":0,\"lat\":40.7128,\"lng\":-74.0060,\"altitude\":2.5,\"action\":\"MOVE\"},{\"index\":1,\"lat\":40.7150,\"lng\":-74.0090,\"altitude\":2.5,\"action\":\"SCAN\"}]}"
  }
}
```

### 2.2. Recepción de Telemetría de Posición (ROS2 ➔ Navegador)
El mapa actualiza en tiempo real la posición del robot (`MBX-02`) al recibir coordenadas del tópico `/mechros2/system_state`.

---

## 3. Experiencia de Usuario en el Dashboard
1. **Pestaña Dedicada**: Se añade el módulo **"Navigation Mesh"** en la barra lateral, complementando los paneles existentes.
2. **Mapa Interactivo con Google Maps**: Carga automática mediante el proxy seguro de Manus sin requerir API keys manuales en el cliente.
3. **Herramientas de Edición**:
   - Clic en el mapa para añadir un waypoint numerado.
   - Panel lateral con lista de waypoints, coordenadas, acciones (`MOVE`, `SCAN`, `HOLD`) y botones para subir, bajar o eliminar.
   - Botón de **Borrar Ruta** y botón de **Transmitir Waypoints** con confirmación de estado WebSocket.

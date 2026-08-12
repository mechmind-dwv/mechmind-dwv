# 📊 Especificación Técnica: Panel de Diagnóstico Avanzado para `mechros2`

## 1. Resumen Ejecutivo
El panel de diagnóstico avanzado (`Mech Systems`) proporciona una visibilidad profunda del hardware de `MechBot-2X`, desglosando el **consumo energético por actuador** (en vatios y amperios) y la **temperatura de los motores** en tiempo real. La interfaz se conecta al tópico ROS2 `/mechros2/system_state` mediante el puente WebSocket (rosbridge v2) y cuenta con un modo de respaldo local para asegurar la continuidad operativa [1] [2].

---

## 2. Contrato de Datos ROS2 (Diagnóstico)

### 2.1. Tópico de Entrada (`/mechros2/system_state`)
El nodo ROS2 transmite métricas estructuradas que el dashboard procesa para actualizar las barras de actuadores y las curvas térmicas:

```json
{
  "timestamp": "2026-08-12T14:45:00Z",
  "system_status": "Active",
  "actuators": [
    { "id": "act_fl", "name": "Front-Left Drive", "power_watts": 48.2, "current_amps": 4.02, "temperature_c": 44.5, "status": "nominal" },
    { "id": "act_fr", "name": "Front-Right Drive", "power_watts": 47.9, "current_amps": 3.98, "temperature_c": 44.1, "status": "nominal" },
    { "id": "act_rl", "name": "Rear-Left Drive", "power_watts": 52.4, "current_amps": 4.36, "temperature_c": 47.2, "status": "warning" },
    { "id": "act_rr", "name": "Rear-Right Drive", "power_watts": 51.8, "current_amps": 4.31, "temperature_c": 46.8, "status": "nominal" },
    { "id": "act_arm", "name": "Manipulator Arm", "power_watts": 28.5, "current_amps": 2.37, "temperature_c": 39.4, "status": "nominal" }
  ],
  "thermal_envelope": {
    "core_avg": 42.4,
    "peak_motor": 47.2,
    "ambient": 24.0
  }
}
```

---

## 3. Arquitectura del Panel

1. **Grid de Actuadores**: Tarjetas individuales para cada motor/actuador con indicador de estado (`nominal`, `warning`, `critical`), consumo en tiempo real y temperatura.
2. **Gráfico de Potencia (Recharts)**: Evolución histórica del consumo de los actuadores principales en vatios.
3. **Control de Calibración**: Botón de calibración remota que publica comandos de diagnóstico en `/mechros2/remote_commands`.

---

## Referencias

[1]: https://github.com/RobotWebTools/rosbridge_suite "RobotWebTools rosbridge_suite"
[2]: https://docs.ros.org/en/humble/Concepts/Intermediate/About-ROS-Interfaces.html "ROS 2 Humble: About ROS interfaces"

Autor: **Manus AI**
Fecha: 12 de agosto de 2026

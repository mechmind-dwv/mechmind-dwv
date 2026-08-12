# Configuración del puente ROS2

El dashboard intenta conectar con `ws://localhost:9090` cuando no existe una variable de entorno. Para usar un robot remoto durante el desarrollo, define `VITE_ROS_BRIDGE_WS_URL` antes de iniciar Vite:

```bash
export VITE_ROS_BRIDGE_WS_URL=ws://ROBOT_HOST:9090
pnpm dev
```

En producción HTTPS, utiliza un endpoint TLS (`wss://`) accesible desde el navegador:

```bash
export VITE_ROS_BRIDGE_WS_URL=wss://robot.example.com/rosbridge
pnpm build
```

El dashboard se conecta al protocolo rosbridge v2, se suscribe a `/mechros2/system_state` y publica comandos en `/mechros2/remote_commands`. Si el socket no responde tras tres intentos, la interfaz conserva su telemetría local y marca `UPLINK SIMULATED`.

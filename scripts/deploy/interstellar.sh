# scripts/deploy/interstellar.sh
#!/bin/bash
# ¡DESPLIEGUE INTERESTELAR! (ejecutar con cuidado)

set -eo pipefail

# 1. Configurar ROS2
if [ -f "/opt/ros/humble/setup.bash" ]; then
    source "/opt/ros/humble/setup.bash"
    echo "🦾 ROS2: HUMBLE DETECTADO"
else
    echo "⚠️ Instala ROS2 Humble primero!"
    exit 1
fi

# 2. Desplegar Microservicios
for project in mechcore ros2 iot_huerto; do
    cd ~/mechmind-master/projects/$project
    cargo build --release
    ./target/release/$project &
done

# 3. Monitoreo Cósmico
nohup ./scripts/monitoring/cosmic_ray_detector.sh > monitor.log 2>&1 &

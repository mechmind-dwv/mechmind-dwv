# Manual de Usuario Oficial - MechMind-dwv

## 1. Introducción y Visión General

El ecosistema **MechMind-dwv** representa una plataforma unificada para el control de robots móviles autónomos (AGV/AMR) impulsados por hardware basado en RISC-V (ESP32-C6), software en Rust de alto rendimiento (`no_std` con aceleración matemática por `portable_simd`), nodos de comunicación ROS2 (`mechros2`) y un panel de control interactivo de estética cyberpunk [1] [2].

Este manual está dirigido a ingenieros de robótica, operadores de campo y desarrolladores que necesitan desplegar, supervisar y operar el sistema tanto en estaciones de trabajo Linux como en dispositivos móviles mediante **Termux** [1].

---

## 2. Arquitectura del Sistema y Flujo Operativo

El flujo operativo se divide en tres capas principales: la capa de control embebido en el robot, el nodo de puente y pasarela ROS2, y la interfaz gráfica de supervisión y navegación en tiempo real [1] [2].

| Capa | Componente | Tecnología | Propósito |
|---|---|---|---|
| **Embebidda** | Firmware del MechBot-2X | Rust (`no_std`), ESP32-C6 | Control de actuadores, lectura de sensores I2C/LiDAR y lazos PID de bajo nivel [1]. |
| **Comunicaciones** | Nodo MechROS2 y Bridge | Rust (`mechros2`), Rosbridge v2 | Traducción de tópicos ROS2 a mensajes JSON sobre WebSockets [1]. |
| **Supervisión** | Cyber Dashboard | React 19, Tailwind CSS, Vite | Interfaz HUD en tiempo real, editor de waypoints y monitor de diagnóstico [1]. |

---

## 3. Puesta en Marcha del Sistema

### 3.1. Requisitos del Entorno de Desarrollo
Para compilar y verificar el workspace de Rust, se requiere obligatoriamente el canal **Nightly** debido al uso de características experimentales de SIMD portable (`#![feature(portable_simd)]`) [1] [2]. Si ejecuta `cargo` simple, Termux intentará usar el canal estable y fallará con el error `error[E0554]: #![feature] may not be used on the stable release channel` [2]:

```bash
rustup toolchain install nightly
cargo +nightly build --workspace
cargo +nightly test --workspace
cargo +nightly doc --workspace --no-deps --document-private-items
```

### 3.2. Ejecución del Panel de Control (Frontend)
El panel de control cuenta con un diseño cyberpunk avanzado y soporte para WebSocket en directo con fallback automático a simulación local [1]:

```bash
cd projects/frontend
pnpm install
pnpm run dev
```

---

## 4. Operación del Dashboard Cyberpunk (HUD)

El panel de control principal se estructura en módulos interactivos diseñados para ofrecer una visión táctica completa del robot [1]:

1. **Command Deck:** Muestra el estado general de conexión, la tasa de refresco de la telemetría, el modo de operación (Autónomo, Manual, Calibración) y los interruptores maestros de seguridad de emergencia.
2. **Mech Systems (Diagnóstico Avanzado):** Presenta en tiempo real gráficos de consumo energético por actuador, temperaturas de motores y la matriz de canales del sistema [1]. Ante la ausencia de un canal activo de ROS2, el sistema activa un modo simulado que alerta de sobrecalentamientos cuando la temperatura nominal supera los umbrales seguros.
3. **Navigation Mesh (Editor de Rutas):** Permite al operador visualizar un mapa interactivo de navegación, añadir marcadores personalizados, configurar waypoints con diferentes comportamientos (`MOVE`, `SCAN`, `HOLD`), calcular distancias estimadas y transmitir las órdenes de misión directamente al tópico ROS2 `/mechros2/navigation_goals` [1].
4. **AI Core:** Visualiza el estado del motor cognitivo y los modelos predictivos empleados para la toma de decisiones autónomas en ruta [1].

---

## 5. Operación en Termux (Android)

Para aquellos operadores que gestionan el repositorio directamente desde dispositivos móviles mediante **Termux**, se deben tener en cuenta las siguientes directrices operativas documentadas en la guía técnica del proyecto [`docs/termux-guide.md`](termux-guide.md) [1]:

- **Configuración global de PNPM:** Si el gestor de paquetes indica que la ruta global no está en el `$PATH`, ejecute `pnpm setup` y recargue su shell con `source ~/.bashrc` [1].
- **Gestión de variables de entorno:** Evite conflictos con NVM desconfigurando temporalmente la variable `PREFIX` (`unset PREFIX`) antes de cargar el perfil de la terminal [1].
- **Actualización y Sincronización:** Para igualar su copia local con el repositorio remoto oficial sin conflictos de ramas divergentes, utilice un respaldo preventivo y la sincronización limpia mediante `git fetch` y `git reset --hard origin/main` [1].

---

## 6. Mantenimiento y Verificación Continua

El repositorio incluye automatizaciones integradas en GitHub Actions para asegurar la calidad del código, la seguridad y la documentación:

- **Rust Super Pipeline:** Compila el workspace, ejecuta pruebas unitarias de SIMD y calcula la cobertura con Tarpaulin [1].
- **Documentation Engine:** Valida automáticamente los enlaces relativos de la documentación mediante el script [`scripts/check_docs_links.py`](../scripts/check_docs_links.py) y genera el sitio en GitHub Pages [1].
- **Quality Gate:** Ejecuta análisis estáticos y de seguridad sobre los componentes de Python e infraestructura [1].

---

## Referencias

[1] Repositorio oficial de MechMind-dwv: [https://github.com/mechmind-dwv/mechmind-dwv](https://github.com/mechmind-dwv/mechmind-dwv) [1]

[2] Documentación y especificaciones de arquitectura técnica en `./docs/architecture/` [2]

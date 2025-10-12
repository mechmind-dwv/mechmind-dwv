// 🦾 Actuator Controller Module
// File: projects/mechros2/src/actuators.rs

use std::sync::Arc;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn, error};
use tokio::time::{Duration, sleep};
use crate::node_manager::MechNodeManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActuatorCommands {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub linear_velocity: Option<Vector3<f64>>,
    pub angular_velocity: Option<Vector3<f64>>,
    pub motor_speeds: Option<Vec<f32>>, // RPM para cada motor
    pub servo_positions: Option<Vec<f32>>, // Ángulos en grados
    pub gripper_command: Option<GripperCommand>,
    pub led_commands: Option<Vec<LedCommand>>,
    pub speaker_command: Option<SpeakerCommand>,
    pub emergency_stop: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GripperCommand {
    pub position: f32, // 0.0 = cerrado, 1.0 = abierto
    pub force: f32,    // Fuerza máxima en Newtons
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedCommand {
    pub led_id: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub brightness: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerCommand {
    pub frequency: f32,
    pub duration_ms: u32,
    pub volume: f32, // 0.0 - 1.0
}

pub struct ActuatorController {
    node_manager: Arc<MechNodeManager>,
    motor_controller: MotorController,
    servo_controller: ServoController,
    gripper_controller: GripperController,
    led_controller: LedController,
    speaker_controller: SpeakerController,
    safety_monitor: SafetyMonitor,
}

impl ActuatorController {
    pub async fn new(node_manager: Arc<MechNodeManager>) -> Result<Self, Box<dyn std::error::Error>> {
        info!("🦾 Inicializando Actuator Controller...");

        let controller = Self {
            node_manager,
            motor_controller: MotorController::new().await?,
            servo_controller: ServoController::new().await?,
            gripper_controller: GripperController::new().await?,
            led_controller: LedController::new().await?,
            speaker_controller: SpeakerController::new().await?,
            safety_monitor: SafetyMonitor::new().await?,
        };

        info!("✅ Actuator Controller inicializado");
        Ok(controller)
    }

    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔧 Inicializando actuadores...");

        // Inicializar todos los subsistemas
        self.motor_controller.initialize().await?;
        self.servo_controller.initialize().await?;
        self.gripper_controller.initialize().await?;
        self.led_controller.initialize().await?;
        self.speaker_controller.initialize().await?;
        self.safety_monitor.activate().await?;

        // Secuencia de inicio visual
        self.startup_sequence().await?;

        info!("✅ Todos los actuadores inicializados correctamente");
        Ok(())
    }

    async fn startup_sequence(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🌟 Ejecutando secuencia de inicio...");

        // LEDs de arranque
        for i in 0..8 {
            let led_cmd = LedCommand {
                led_id: i,
                red: 0,
                green: 255,
                blue: 0,
                brightness: 100,
            };
            self.led_controller.set_led(&led_cmd).await?;
            sleep(Duration::from_millis(100)).await;
        }

        // Sonido de inicio
        let beep = SpeakerCommand {
            frequency: 1000.0,
            duration_ms: 200,
            volume: 0.5,
        };
        self.speaker_controller.play_sound(&beep).await?;

        // Test de motores (velocidad baja)
        self.motor_controller.test_motors().await?;

        // Reset LEDs
        self.led_controller.clear_all().await?;

        info!("✅ Secuencia de inicio completada");
        Ok(())
    }

    pub async fn execute_commands(&self, commands: ActuatorCommands) -> Result<(), Box<dyn std::error::Error>> {
        debug!("🎯 Ejecutando comandos de actuadores...");

        // Verificar parada de emergencia
        if commands.emergency_stop {
            warn!("🚨 PARADA DE EMERGENCIA ACTIVADA");
            return self.emergency_stop().await;
        }

        // Verificar seguridad
        if !self.safety_monitor.is_safe(&commands).await? {
            warn!("⚠️ Comandos rechazados por monitor de seguridad");
            return Ok(());
        }

        // Ejecutar comandos en paralelo
        let motor_task = async {
            if let Some(ref speeds) = commands.motor_speeds {
                self.motor_controller.set_speeds(speeds).await
            } else if let Some(linear_vel) = commands.linear_velocity {
                self.motor_controller.set_velocity(linear_vel, commands.angular_velocity).await
            } else {
                Ok(())
            }
        };

        let servo_task = async {
            if let Some(ref positions) = commands.servo_positions {
                self.servo_controller.set_positions(positions).await
            } else {
                Ok(())
            }
        };

        let gripper_task = async {
            if let Some(ref grip_cmd) = commands.gripper_command {
                self.gripper_controller.execute_command(grip_cmd).await
            } else {
                Ok(())
            }
        };

        let led_task = async {
            if let Some(ref led_cmds) = commands.led_commands {
                for led_cmd in led_cmds {
                    self.led_controller.set_led(led_cmd).await?;
                }
                Ok::<(), Box<dyn std::error::Error>>(())
            } else {
                Ok(())
            }
        };

        let speaker_task = async {
            if let Some(ref sound_cmd) = commands.speaker_command {
                self.speaker_controller.play_sound(sound_cmd).await
            } else {
                Ok(())
            }
        };

        // Ejecutar todas las tareas
        let results = tokio::join!(motor_task, servo_task, gripper_task, led_task, speaker_task);

        // Verificar errores
        if let Err(e) = results.0 { error!("Error en motores: {}", e); }
        if let Err(e) = results.1 { error!("Error en servos: {}", e); }
        if let Err(e) = results.2 { error!("Error en gripper: {}", e); }
        if let Err(e) = results.3 { error!("Error en LEDs: {}", e); }
        if let Err(e) = results.4 { error!("Error en altavoz: {}", e); }

        // Publicar estado de actuadores
        let status_json = serde_json::to_string(&self.get_status().await)?;
        self.node_manager.publish_command(&format!("STATUS: {}", status_json)).await?;

        debug!("✅ Comandos de actuadores ejecutados");
        Ok(())
    }

    pub async fn emergency_stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        error!("🚨 EJECUTANDO PARADA DE EMERGENCIA");

        // Parar todos los motores inmediatamente
        self.motor_controller.stop_all().await?;

        // LEDs de emergencia (rojo intermitente)
        for _ in 0..5 {
            for i in 0..8 {
                let led_cmd = LedCommand {
                    led_id: i,
                    red: 255,
                    green: 0,
                    blue: 0,
                    brightness: 255,
                };
                self.led_controller.set_led(&led_cmd).await?;
            }
            sleep(Duration::from_millis(200)).await;
            self.led_controller.clear_all().await?;
            sleep(Duration::from_millis(200)).await;
        }

        // Sonido de alarma
        for _ in 0..3 {
            let alarm = SpeakerCommand {
                frequency: 2000.0,
                duration_ms: 300,
                volume: 0.8,
            };
            self.speaker_controller.play_sound(&alarm).await?;
            sleep(Duration::from_millis(100)).await;
        }

        error!("🛑 Sistema en estado de emergencia");
        Ok(())
    }

    pub async fn get_status(&self) -> ActuatorStatus {
        ActuatorStatus {
            motors_online: self.motor_controller.is_online().await,
            servos_online: self.servo_controller.is_online().await,
            gripper_online: self.gripper_controller.is_online().await,
            leds_online: self.led_controller.is_online().await,
            speaker_online: self.speaker_controller.is_online().await,
            safety_system_active: self.safety_monitor.is_active().await,
            emergency_stop_active: false, // Se actualizaría dinámicamente
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActuatorStatus {
    pub motors_online: bool,
    pub servos_online: bool,
    pub gripper_online: bool,
    pub leds_online: bool,
    pub speaker_online: bool,
    pub safety_system_active: bool,
    pub emergency_stop_active: bool,
}

// Controladores individuales
pub struct MotorController {
    motor_count: usize,
    max_rpm: f32,
    initialized: bool,
}

impl MotorController {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            motor_count: 4,
            max_rpm: 3000.0,
            initialized: false,
        })
    }

    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("⚙️ Inicializando {} motores...", self.motor_count);
        sleep(Duration::from_millis(200)).await;
        self.initialized = true;
        info!("✅ Motores inicializados - RPM máximo: {}", self.max_rpm);
        Ok(())
    }

    async fn set_speeds(&self, speeds: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
        if speeds.len() != self.motor_count {
            return Err(format!("Se esperaban {} velocidades, recibidas {}",
                               self.motor_count, speeds.len()).into());
        }

        for (i, &speed) in speeds.iter().enumerate() {
            let clamped_speed = speed.clamp(-self.max_rpm, self.max_rpm);
            debug!("Motor {}: {} RPM", i, clamped_speed);
        }

        Ok(())
    }

    async fn set_velocity(&self, linear: Vector3<f64>, angular: Option<Vector3<f64>>) -> Result<(), Box<dyn std::error::Error>> {
        // Conversión básica de velocidad lineal/angular a velocidades de motor
        let forward_speed = linear.x as f32 * 100.0; // Factor de escala
        let angular_speed = angular.map_or(0.0, |a| a.z as f32 * 50.0);

        let left_speed = forward_speed - angular_speed;
        let right_speed = forward_speed + angular_speed;

        let speeds = vec![left_speed, right_speed, left_speed, right_speed];
        self.set_speeds(&speeds).await
    }

    async fn test_motors(&self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("🧪 Probando motores...");
        let test_speeds = vec![100.0; self.motor_count];
        self.set_speeds(&test_speeds).await?;
        sleep(Duration::from_millis(500)).await;
        self.stop_all().await?;
        Ok(())
    }

    async fn stop_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let zero_speeds = vec![0.0; self.motor_count];
        self.set_speeds(&zero_speeds).await
    }

    async fn is_online(&self) -> bool {
        self.initialized
    }
}

pub struct ServoController {
    servo_count: usize,
    initialized: bool,
}

impl ServoController {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            servo_count: 6,
            initialized: false,
        })
    }

    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("🔧 Inicializando {} servos...", self.servo_count);
        sleep(Duration::from_millis(150)).await;
        self.initialized = true;
        info!("✅ Servos inicializados");
        Ok(())
    }

    async fn set_positions(&self, positions: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
        for (i, &pos) in positions.iter().enumerate() {
            if i >= self.servo_count { break; }
            let clamped_pos = pos.clamp(0.0, 180.0);
            debug!("Servo {}: {}°", i, clamped_pos);
        }
        Ok(())
    }

    async fn is_online(&self) -> bool {
        self.initialized
    }
}

pub struct GripperController {
    initialized: bool,
    current_position: f32,
}

impl GripperController {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            initialized: false,
            current_position: 0.0,
        })
    }

    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("🤏 Inicializando gripper...");
        sleep(Duration::from_millis(100)).await;
        self.initialized = true;
        info!("✅ Gripper inicializado");
        Ok(())
    }

    async fn execute_command(&mut self, command: &GripperCommand) -> Result<(), Box<dyn std::error::Error>> {
        let target_pos = command.position.clamp(0.0, 1.0);
        debug!("🤏 Gripper: {} -> {} (fuerza: {}N)",
               self.current_position, target_pos, command.force);

        // Simular movimiento gradual
        let steps = 10;
        let step_size = (target_pos - self.current_position) / steps as f32;

        for _ in 0..steps {
            self.current_position += step_size;
            sleep(Duration::from_millis(20)).await;
        }

        self.current_position = target_pos;
        Ok(())
    }

    async fn is_online(&self) -> bool {
        self.initialized
    }
}

pub struct LedController {
    led_count: usize,
    initialized: bool,
}

impl LedController {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            led_count: 8,
            initialized: false,
        })
    }

    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("💡 Inicializando {} LEDs...", self.led_count);
        self.initialized = true;
        info!("✅ LEDs inicializados");
        Ok(())
    }

    async fn set_led(&self, command: &LedCommand) -> Result<(), Box<dyn std::error::Error>> {
        if command.led_id as usize >= self.led_count {
            return Err("ID de LED fuera de rango".into());
        }

        debug!("💡 LED {}: RGB({},{},{}) @ {}%",
               command.led_id, command.red, command.green, command.blue, command.brightness);
        Ok(())
    }

    async fn clear_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..self.led_count {
            let cmd = LedCommand {
                led_id: i as u8,
                red: 0,
                green: 0,
                blue: 0,
                brightness: 0,
            };
            self.set_led(&cmd).await?;
        }
        Ok(())
    }

    async fn is_online(&self) -> bool {
        self.initialized
    }
}

pub struct SpeakerController {
    initialized: bool,
}

impl SpeakerController {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { initialized: false })
    }

    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("🔊 Inicializando altavoz...");
        self.initialized = true;
        info!("✅ Altavoz inicializado");
        Ok(())
    }

    async fn play_sound(&self, command: &SpeakerCommand) -> Result<(), Box<dyn std::error::Error>> {
        debug!("🔊 Reproduciendo: {}Hz por {}ms @ vol {}",
               command.frequency, command.duration_ms, command.volume);

        // Simular duración del sonido
        sleep(Duration::from_millis(command.duration_ms as u64)).await;
        Ok(())
    }

    async fn is_online(&self) -> bool {
        self.initialized
    }
}

pub struct SafetyMonitor {
    active: bool,
    max_linear_velocity: f64,
    max_angular_velocity: f64,
}

impl SafetyMonitor {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            active: false,
            max_linear_velocity: 2.0, // m/s
            max_angular_velocity: 1.0, // rad/s
        })
    }

    async fn activate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("🛡️ Activando monitor de seguridad...");
        self.active = true;
        info!("✅ Monitor de seguridad activo");
        Ok(())
    }

    async fn is_safe(&self, commands: &ActuatorCommands) -> Result<bool, Box<dyn std::error::Error>> {
        if !self.active {
            return Ok(true);
        }

        // Verificar velocidades lineales
        if let Some(linear_vel) = commands.linear_velocity {
            if linear_vel.magnitude() > self.max_linear_velocity {
                warn!("⚠️ Velocidad lineal excede límite: {} > {}",
                      linear_vel.magnitude(), self.max_linear_velocity);
                return Ok(false);
            }
        }

        // Verificar velocidades angulares
        if let Some(angular_vel) = commands.angular_velocity {
            if angular_vel.magnitude() > self.max_angular_velocity {
                warn!("⚠️ Velocidad angular excede límite: {} > {}",
                      angular_vel.magnitude(), self.max_angular_velocity);
                return Ok(false);
            }
        }

        // Verificar velocidades de motores
        if let Some(ref speeds) = commands.motor_speeds {
            for &speed in speeds {
                if speed.abs() > 3000.0 {
                    warn!("⚠️ Velocidad de motor excede límite: {} > 3000", speed.abs());
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    async fn is_active(&self) -> bool {
        self.active
    }
}

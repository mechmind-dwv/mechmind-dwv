// 🦾 MechMind ROS2 Integration Layer
// File: projects/mechros2/src/main.rs

use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::Mutex;
use tracing::{info, warn, error, debug};
use nalgebra::{Vector3, Point3};
use serde::{Deserialize, Serialize};

pub mod node_manager;
pub mod sensors;
pub mod actuators;
pub mod navigation;
pub mod vision;

use node_manager::MechNodeManager;
use sensors::SensorHub;
use actuators::ActuatorController;
use navigation::NavigationPlanner;
use vision::VisionProcessor;

// 🤖 Core system state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub position: Point3<f64>,
    pub velocity: Vector3<f64>,
    pub orientation: Vector3<f64>, // Euler angles
    pub battery_level: f32,
    pub system_status: SystemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    Initializing,
    Ready,
    Active,
    Error(String),
    Shutdown,
}

// 🚀 Main MechROS2 Hub
pub struct MechROS2Hub {
    node_manager: Arc<MechNodeManager>,
    sensor_hub: Arc<Mutex<SensorHub>>,
    actuator_controller: Arc<Mutex<ActuatorController>>,
    navigation_planner: Arc<Mutex<NavigationPlanner>>,
    vision_processor: Arc<Mutex<VisionProcessor>>,
    system_state: Arc<tokio::sync::RwLock<SystemState>>,
}

impl MechROS2Hub {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("🚀 Inicializando MechROS2 Hub...");

        let node_manager = Arc::new(MechNodeManager::new().await?);
        
        let sensor_hub = Arc::new(Mutex::new(SensorHub::new(node_manager.clone()).await?));
        let actuator_controller = Arc::new(Mutex::new(ActuatorController::new(node_manager.clone()).await?));
        let navigation_planner = Arc::new(Mutex::new(NavigationPlanner::new(node_manager.clone()).await?));
        let vision_processor = Arc::new(Mutex::new(VisionProcessor::new(node_manager.clone())));

        let initial_state = SystemState {
            timestamp: chrono::Utc::now(),
            position: Point3::origin(),
            velocity: Vector3::zeros(),
            orientation: Vector3::zeros(),
            battery_level: 100.0,
            system_status: SystemStatus::Initializing,
        };

        let system_state = Arc::new(tokio::sync::RwLock::new(initial_state));

        Ok(Self {
            node_manager,
            sensor_hub,
            actuator_controller,
            navigation_planner,
            vision_processor,
            system_state,
        })
    }

    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔧 Inicializando subsistemas...");

        self.sensor_hub.lock().await.initialize().await?;
        self.actuator_controller.lock().await.initialize().await?;
        self.navigation_planner.lock().await.initialize().await?;
        self.vision_processor.lock().await.initialize().await?;

        {
            let mut state = self.system_state.write().await;
            state.system_status = SystemStatus::Ready;
            state.timestamp = chrono::Utc::now();
        }

        info!("✅ MechROS2 Hub inicializado correctamente");
        Ok(())
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🦾 Iniciando bucle principal de MechROS2...");

        {
            let mut state = self.system_state.write().await;
            state.system_status = SystemStatus::Active;
        }

        let sensor_task = self.run_sensor_loop();
        let navigation_task = self.run_navigation_loop();
        let vision_task = self.run_vision_loop();
        let state_publisher_task = self.run_state_publisher();

        tokio::try_join!(
            sensor_task,
            navigation_task,
            vision_task,
            state_publisher_task
        )?;

        Ok(())
    }

    async fn run_sensor_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let res = self.sensor_hub.lock().await.update_sensors().await;
            match res {
                Ok(sensor_data) => {
                    let mut state = self.system_state.write().await;
                    if let Some(pos) = sensor_data.position {
                        state.position = pos;
                    }
                    if let Some(vel) = sensor_data.velocity {
                        state.velocity = vel;
                    }
                    if let Some(battery) = sensor_data.battery_level {
                        state.battery_level = battery;
                    }
                    state.timestamp = chrono::Utc::now();
                }
                Err(e) => {
                    warn!("⚠️  Error en sensores: {}", e);
                }
            }
            sleep(Duration::from_millis(50)).await;
        }
    }

    async fn run_navigation_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let current_state = self.system_state.read().await.clone();
            let res = self.navigation_planner.lock().await.update_navigation(&current_state).await;
            match res {
                Ok(nav_commands) => {
                    if let Some(commands) = nav_commands {
                        if let Err(e) = self.actuator_controller.lock().await.execute_commands(commands).await {
                            error!("❌ Error ejecutando comandos: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("⚠️  Error en navegación: {}", e);
                }
            }
            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn run_vision_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let res = self.vision_processor.lock().await.process_frame().await;
            match res {
                Ok(vision_data) => {
                    if let Some(data) = vision_data {
                        self.navigation_planner.lock().await.update_vision_data(data).await?;
                    }
                }
                Err(e) => {
                    warn!("⚠️  Error en visión: {}", e);
                }
            }
            sleep(Duration::from_millis(33)).await;
        }
    }

    async fn run_state_publisher(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let current_state = self.system_state.read().await.clone();
            if let Err(e) = self.node_manager.publish_system_state(&current_state).await {
                warn!("⚠️  Error publicando estado: {}", e);
            }
            sleep(Duration::from_millis(200)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("mechros2=debug,info")
        .init();

    println!("🦾 *MECHROS2 SYSTEM INICIADO*");
    
    let hub = MechROS2Hub::new().await?;
    hub.initialize().await?;
    
    info!("🚀 MechROS2 completamente operativo");

    if let Err(e) = hub.run().await {
        error!("💥 Error crítico en MechROS2: {}", e);
        return Err(e);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_initialization() {
        let node_manager = Arc::new(MechNodeManager::new().await.unwrap());
        let sensor_hub = Arc::new(Mutex::new(SensorHub::new(node_manager.clone()).await.unwrap()));
        let actuator_controller = Arc::new(Mutex::new(ActuatorController::new(node_manager.clone()).await.unwrap()));
        let navigation_planner = Arc::new(Mutex::new(NavigationPlanner::new(node_manager.clone()).await.unwrap()));
        let vision_processor = Arc::new(Mutex::new(VisionProcessor::new(node_manager.clone())));
        let system_state = Arc::new(tokio::sync::RwLock::new(SystemState {
            timestamp: chrono::Utc::now(),
            position: Point3::origin(),
            velocity: Vector3::zeros(),
            orientation: Vector3::zeros(),
            battery_level: 100.0,
            system_status: SystemStatus::Initializing,
        }));
        
        let hub = MechROS2Hub {
            node_manager,
            sensor_hub,
            actuator_controller,
            navigation_planner,
            vision_processor,
            system_state,
        };
        assert!(hub.initialize().await.is_ok());
    }
}

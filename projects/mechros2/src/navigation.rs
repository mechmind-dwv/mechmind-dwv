// 🗺️ Navigation Planner Module
// File: projects/mechros2/src/navigation.rs

use std::sync::Arc;
use std::collections::VecDeque;
use nalgebra::{Vector3, Point3, distance};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn, error};
use crate::{SystemState, node_manager::MechNodeManager};
use crate::vision::VisionData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationCommands {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub linear_velocity: Vector3<f64>,
    pub angular_velocity: Vector3<f64>,
    pub target_position: Option<Point3<f64>>,
    pub command_type: CommandType,
    pub priority: NavigationPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    Move,
    Rotate,
    Stop,
    Emergency,
    Hold,
    Precision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavigationPriority {
    Low,
    Normal,
    High,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    pub position: Point3<f64>,
    pub tolerance: f64,
    pub max_speed: f64,
    pub waypoint_type: WaypointType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaypointType {
    Transit,
    Stop,
    Precision,
    Checkpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub waypoints: Vec<Waypoint>,
    pub total_distance: f64,
    pub estimated_time: f64,
    pub path_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObstacleMap {
    pub obstacles: Vec<Obstacle>,
    pub last_update: chrono::DateTime<chrono::Utc>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub position: Point3<f64>,
    pub size: Vector3<f64>,
    pub velocity: Option<Vector3<f64>>,
    pub obstacle_type: ObstacleType,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObstacleType {
    Static,
    Dynamic,
    Unknown,
    Person,
    Vehicle,
    Wall,
}

pub struct NavigationPlanner {
    node_manager: Arc<MechNodeManager>,
    current_path: Option<Path>,
    waypoint_queue: VecDeque<Waypoint>,
    obstacle_map: ObstacleMap,
    navigation_config: NavigationConfig,
    path_planner: PathPlanner,
    obstacle_avoidance: ObstacleAvoidance,
    pid_controller: PIDController,
}

#[derive(Debug, Clone)]
pub struct NavigationConfig {
    pub max_linear_speed: f64,
    pub max_angular_speed: f64,
    pub position_tolerance: f64,
    pub orientation_tolerance: f64,
    pub obstacle_safety_distance: f64,
    pub planning_frequency: f64,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        Self {
            max_linear_speed: 2.0,
            max_angular_speed: 1.57,
            position_tolerance: 0.1,
            orientation_tolerance: 0.1,
            obstacle_safety_distance: 0.5,
            planning_frequency: 10.0,
        }
    }
}

impl NavigationPlanner {
    pub async fn new(node_manager: Arc<MechNodeManager>) -> Result<Self, Box<dyn std::error::Error>> {
        info!("🗺️ Inicializando Navigation Planner...");
        
        let config = NavigationConfig::default();
        let path_planner = PathPlanner::new(&config);
        let obstacle_avoidance = ObstacleAvoidance::new(&config);
        let pid_controller = PIDController::new(1.0, 0.1, 0.05);
        
        let planner = Self {
            node_manager,
            current_path: None,
            waypoint_queue: VecDeque::new(),
            obstacle_map: ObstacleMap {
                obstacles: Vec::new(),
                last_update: chrono::Utc::now(),
                confidence: 0.0,
            },
            navigation_config: config,
            path_planner,
            obstacle_avoidance,
            pid_controller,
        };
        
        info!("✅ Navigation Planner inicializado");
        Ok(planner)
    }
    
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔧 Inicializando sistema de navegación...");
        
        // Cargar mapas si están disponibles
        self.load_maps().await?;
        
        // Configurar parámetros de navegación
        self.configure_navigation().await?;
        
        info!("✅ Sistema de navegación inicializado");
        Ok(())
    }
    
    pub async fn update_navigation(&mut self, current_state: &SystemState) -> Result<Option<NavigationCommands>, Box<dyn std::error::Error>> {
        // Actualizar mapa de obstáculos
        self.update_obstacle_map().await?;
        
        // Procesar nuevos objetivos desde ROS2
        self.process_navigation_goals().await?;
        
        // Si no hay waypoints, no hacer nada
        if self.waypoint_queue.is_empty() {
            return Ok(None);
        }
        
        let current_waypoint = match self.waypoint_queue.front() {
            Some(wp) => wp.clone(),
            None => return Ok(None),
        };
        
        // Verificar si hemos llegado al waypoint actual
        if self.reached_waypoint(&current_state.position, &current_waypoint) {
            debug!("🎯 Waypoint alcanzado: {:?}", current_waypoint.position);
            self.waypoint_queue.pop_front();
            
            if self.waypoint_queue.is_empty() {
                info!("🏁 Navegación completada");
                return Ok(Some(NavigationCommands {
                    timestamp: chrono::Utc::now(),
                    linear_velocity: Vector3::zeros(),
                    angular_velocity: Vector3::zeros(),
                    target_position: None,
                    command_type: CommandType::Stop,
                    priority: NavigationPriority::Normal,
                }));
            }
        }
        
        // Planificar movimiento hacia el waypoint
        let commands = self.plan_movement(current_state, &current_waypoint).await?;
        
        // Aplicar evasión de obstáculos
        let safe_commands = self.obstacle_avoidance.adjust_commands(&commands, &self.obstacle_map).await?;
        
        debug!("🎮 Comandos de navegación generados: {:?}", safe_commands);
        Ok(Some(safe_commands))
    }
    
    pub async fn update_vision_data(&mut self, vision_data: VisionData) -> Result<(), Box<dyn std::error::Error>> {
        // Actualizar mapa de obstáculos con datos de visión
        for detected_object in &vision_data.detected_objects {
            if let Some(position) = detected_object.world_position {
                let obstacle = Obstacle {
                    position,
                    size: Vector3::new(0.5, 0.5, 1.0), // Tamaño estimado
                    velocity: None,
                    obstacle_type: match detected_object.class.as_str() {
                        "person" => ObstacleType::Person,
                        "car" | "vehicle" => ObstacleType::Vehicle,
                        "wall" => ObstacleType::Wall,
                        _ => ObstacleType::Unknown,
                    },
                    confidence: detected_object.confidence,
                };
                
                self.obstacle_map.obstacles.push(obstacle);
            }
        }
        
        self.obstacle_map.last_update = chrono::Utc::now();
        debug!("👁️ Mapa de obstáculos actualizado con {} obstáculos", self.obstacle_map.obstacles.len());
        
        Ok(())
    }
    
    pub async fn set_navigation_goal(&mut self, target: Point3<f64>) -> Result<(), Box<dyn std::error::Error>> {
        info!("🎯 Nuevo objetivo de navegación: {:?}", target);
        
        // Limpiar waypoints anteriores
        self.waypoint_queue.clear();
        
        // Planificar ruta al objetivo
        let path = self.path_planner.plan_path(&target, &self.obstacle_map).await?;
        
        // Agregar waypoints a la cola
        for waypoint in path.waypoints {
            self.waypoint_queue.push_back(waypoint);
        }
        
        self.current_path = Some(path);
        info!("🗺️ Ruta planificada con {} waypoints", self.waypoint_queue.len());
        
        Ok(())
    }
    
    async fn plan_movement(&mut self, current_state: &SystemState, waypoint: &Waypoint) -> Result<NavigationCommands, Box<dyn std::error::Error>> {
        let target_vector = waypoint.position - current_state.position;
        let distance_to_target = target_vector.norm();
        
        // Control PID para velocidad
        let speed_command = self.pid_controller.update(distance_to_target, 0.0);
        let max_speed = waypoint.max_speed.min(self.navigation_config.max_linear_speed);
        let clamped_speed = speed_command.abs().min(max_speed);
        
        // Calcular dirección
        let direction = if distance_to_target > 0.001 {
            target_vector.normalize()
        } else {
            Vector3::zeros()
        };
        
        let linear_velocity = direction * clamped_speed;
        
        // Calcular velocidad angular (orientación hacia el objetivo)
        let target_yaw = target_vector.y.atan2(target_vector.x);
        let current_yaw = current_state.orientation.z; // Asumiendo que Z es yaw
        let yaw_error = self.normalize_angle(target_yaw - current_yaw);
        let angular_velocity = Vector3::new(0.0, 0.0, yaw_error * 2.0); // Ganancia simple
        
        Ok(NavigationCommands {
            timestamp: chrono::Utc::now(),
            linear_velocity,
            angular_velocity,
            target_position: Some(waypoint.position),
            command_type: match waypoint.waypoint_type {
                WaypointType::Stop => CommandType::Stop,
                WaypointType::Precision => CommandType::Precision,
                _ => CommandType::Move,
            },
            priority: NavigationPriority::Normal,
        })
    }
    
    fn reached_waypoint(&self, current_pos: &Point3<f64>, waypoint: &Waypoint) -> bool {
        distance(current_pos, &waypoint.position) < waypoint.tolerance
    }
    
    fn normalize_angle(&self, angle: f64) -> f64 {
        let mut normalized = angle;
        while normalized > std::f64::consts::PI {
            normalized -= 2.0 * std::f64::consts::PI;
        }
        while normalized < -std::f64::consts::PI {
            normalized += 2.0 * std::f64::consts::PI;
        }
        normalized
    }
    
    async fn process_navigation_goals(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let goals = self.node_manager.get_navigation_goals().await;
        
        for goal_str in goals {
            if let Ok(goal_data) = serde_json::from_str::<serde_json::Value>(&goal_str) {
                if let (Some(x), Some(y), Some(z)) = (
                    goal_data["x"].as_f64(),
                    goal_data["y"].as_f64(),
                    goal_data["z"].as_f64()
                ) {
                    let target = Point3::new(x, y, z);
                    self.set_navigation_goal(target).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn load_maps(&self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("🗺️ Cargando mapas de navegación...");
        // Implementar carga de mapas desde archivos o servicios
        Ok(())
    }
    
    async fn configure_navigation(&self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("⚙️ Configurando parámetros de navegación...");
        // Configurar parámetros específicos del robot
        Ok(())
    }
    
    async fn update_obstacle_map(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Limpiar obstáculos antiguos (más de 5 segundos)
        let now = chrono::Utc::now();
        let timeout = chrono::Duration::seconds(5);
        
        if now - self.obstacle_map.last_update > timeout {
            self.obstacle_map.obstacles.clear();
            debug!("🧹 Mapa de obstáculos limpiado por timeout");
        }
        
        Ok(())
    }
}

// Planificador de rutas
pub struct PathPlanner {
    config: NavigationConfig,
}

impl PathPlanner {
    pub fn new(config: &NavigationConfig) -> Self {
        Self { config: config.clone() }
    }
    
    pub async fn plan_path(&self, target: &Point3<f64>, obstacle_map: &ObstacleMap) -> Result<Path, Box<dyn std::error::Error>> {
        debug!("🛤️ Planificando ruta hacia {:?}", target);
        
        // Algoritmo de planificación simple (A* simplificado)
        let waypoints = vec![
            Waypoint {
                position: *target,
                tolerance: self.config.position_tolerance,
                max_speed: self.config.max_linear_speed,
                waypoint_type: WaypointType::Stop,
            }
        ];
        
        let total_distance = waypoints.iter()
            .map(|wp| wp.position.coords.norm())
            .sum();
        
        let estimated_time = total_distance / self.config.max_linear_speed;
        
        Ok(Path {
            waypoints,
            total_distance,
            estimated_time,
            path_id: format!("path_{}", chrono::Utc::now().timestamp()),
        })
    }
}

// Sistema de evasión de obstáculos
pub struct ObstacleAvoidance {
    config: NavigationConfig,
}

impl ObstacleAvoidance {
    pub fn new(config: &NavigationConfig) -> Self {
        Self { config: config.clone() }
    }
    
    pub async fn adjust_commands(&self, commands: &NavigationCommands, obstacle_map: &ObstacleMap) -> Result<NavigationCommands, Box<dyn std::error::Error>> {
        let mut adjusted_commands = commands.clone();
        
        // Verificar colisiones potenciales
        for obstacle in &obstacle_map.obstacles {
            let distance = obstacle.position.coords.norm();
            
            if distance < self.config.obstacle_safety_distance {
                // Reducir velocidad cerca de obstáculos
                let speed_factor = (distance / self.config.obstacle_safety_distance).max(0.1);
                adjusted_commands.linear_velocity *= speed_factor;
                
                // Aplicar vector de evasión lateral
                let avoidance_vector = self.calculate_avoidance_vector(obstacle);
                adjusted_commands.linear_velocity += avoidance_vector;
                
                debug!("⚠️ Ajustando comandos por obstáculo a {:.2}m", distance);
            }
        }
        
        Ok(adjusted_commands)
    }
    
    fn calculate_avoidance_vector(&self, obstacle: &Obstacle) -> Vector3<f64> {
        // Vector simple de evasión perpendicular al obstáculo
        let obstacle_dir = obstacle.position.coords.normalize();
        Vector3::new(-obstacle_dir.y, obstacle_dir.x, 0.0) * 0.5
    }
}

// Controlador PID simple
pub struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: f64,
    previous_error: f64,
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp, ki, kd,
            integral: 0.0,
            previous_error: 0.0,
        }
    }
    
    pub fn update(&mut self, current: f64, target: f64) -> f64 {
        let error = target - current;
        self.integral += error;
        let derivative = error - self.previous_error;
        self.previous_error = error;
        
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}

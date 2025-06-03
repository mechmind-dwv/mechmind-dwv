// 🚀 Node Manager Module
// File: projects/mechros2/src/node_manager.rs

use std::sync::Arc;
use std::collections::HashMap;
use r2r::QosProfile;
use serde_json;
use tracing::{info, debug, error};
use crate::SystemState;

pub struct MechNodeManager {
    ctx: r2r::Context,
    node: r2r::Node,
    publishers: HashMap<String, r2r::Publisher<r2r::std_msgs::msg::String>>,
    subscribers: HashMap<String, Arc<tokio::sync::Mutex<Vec<String>>>>,
}

impl MechNodeManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("🔧 Inicializando Node Manager...");
        
        let ctx = r2r::Context::create()?;
        let node = r2r::Node::create(ctx.clone(), "mechros2_hub", "")?;
        
        let mut manager = Self {
            ctx,
            node,
            publishers: HashMap::new(),
            subscribers: HashMap::new(),
        };
        
        // Configurar publishers y subscribers por defecto
        manager.setup_default_topics().await?;
        
        info!("✅ Node Manager inicializado");
        Ok(manager)
    }
    
    async fn setup_default_topics(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Publisher para estado del sistema
        let state_pub = self.node.create_publisher::<r2r::std_msgs::msg::String>(
            "/mechros2/system_state",
            QosProfile::default()
        )?;
        self.publishers.insert("system_state".to_string(), state_pub);
        
        // Publisher para comandos
        let cmd_pub = self.node.create_publisher::<r2r::std_msgs::msg::String>(
            "/mechros2/commands",
            QosProfile::default()
        )?;
        self.publishers.insert("commands".to_string(), cmd_pub);
        
        // Publisher para telemetría
        let telemetry_pub = self.node.create_publisher::<r2r::std_msgs::msg::String>(
            "/mechros2/telemetry",
            QosProfile::default()
        )?;
        self.publishers.insert("telemetry".to_string(), telemetry_pub);
        
        // Subscriber para objetivos de navegación
        let goal_buffer = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let goal_sub = self.node.create_subscription::<r2r::std_msgs::msg::String>(
            "/mechros2/navigation_goals",
            QosProfile::default(),
            {
                let buffer = goal_buffer.clone();
                move |msg| {
                    let buffer = buffer.clone();
                    tokio::spawn(async move {
                        let mut msgs = buffer.lock().await;
                        msgs.push(msg.data.clone());
                        // Mantener solo los últimos 10 mensajes
                        if msgs.len() > 10 {
                            msgs.remove(0);
                        }
                        debug!("📨 Nuevo objetivo de navegación recibido: {}", msg.data);
                    });
                }
            }
        )?;
        self.subscribers.insert("navigation_goals".to_string(), goal_buffer);
        
        // Subscriber para comandos remotos
        let remote_cmd_buffer = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let remote_cmd_sub = self.node.create_subscription::<r2r::std_msgs::msg::String>(
            "/mechros2/remote_commands",
            QosProfile::default(),
            {
                let buffer = remote_cmd_buffer.clone();
                move |msg| {
                    let buffer = buffer.clone();
                    tokio::spawn(async move {
                        let mut msgs = buffer.lock().await;
                        msgs.push(msg.data.clone());
                        if msgs.len() > 5 {
                            msgs.remove(0);
                        }
                        debug!("🎮 Comando remoto recibido: {}", msg.data);
                    });
                }
            }
        )?;
        self.subscribers.insert("remote_commands".to_string(), remote_cmd_buffer);
        
        info!("📡 Topics configurados correctamente");
        Ok(())
    }
    
    pub async fn publish_system_state(&self, state: &SystemState) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(publisher) = self.publishers.get("system_state") {
            let json_state = serde_json::to_string(state)?;
            let msg = r2r::std_msgs::msg::String {
                data: json_state,
            };
            
            publisher.publish(&msg)?;
            debug!("📊 Estado del sistema publicado");
        }
        Ok(())
    }
    
    pub async fn publish_telemetry(&self, data: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(publisher) = self.publishers.get("telemetry") {
            let msg = r2r::std_msgs::msg::String {
                data: data.to_string(),
            };
            
            publisher.publish(&msg)?;
            debug!("📈 Telemetría publicada");
        }
        Ok(())
    }
    
    pub async fn publish_command(&self, command: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(publisher) = self.publishers.get("commands") {
            let msg = r2r::std_msgs::msg::String {
                data: command.to_string(),
            };
            
            publisher.publish(&msg)?;
            debug!("🎯 Comando publicado: {}", command);
        }
        Ok(())
    }
    
    pub async fn get_navigation_goals(&self) -> Vec<String> {
        if let Some(buffer) = self.subscribers.get("navigation_goals") {
            let msgs = buffer.lock().await;
            msgs.clone()
        } else {
            Vec::new()
        }
    }
    
    pub async fn get_remote_commands(&self) -> Vec<String> {
        if let Some(buffer) = self.subscribers.get("remote_commands") {
            let msgs = buffer.lock().await;
            msgs.clone()
        } else {
            Vec::new()
        }
    }
    
    pub async fn create_custom_publisher<T>(&mut self, topic: &str) -> Result<(), Box<dyn std::error::Error>>
    where
        T: r2r::WrappedTypesupport,
    {
        let publisher = self.node.create_publisher::<T>(topic, QosProfile::default())?;
        info!("📡 Publisher personalizado creado para topic: {}", topic);
        Ok(())
    }
    
    pub async fn spin_once(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Procesar callbacks pendientes
        self.ctx.spin_once(std::time::Duration::from_millis(10))?;
        Ok(())
    }
    
    pub fn get_node_info(&self) -> NodeInfo {
        NodeInfo {
            name: "mechros2_hub".to_string(),
            namespace: "".to_string(),
            publishers_count: self.publishers.len(),
            subscribers_count: self.subscribers.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub name: String,
    pub namespace: String,
    pub publishers_count: usize,
    pub subscribers_count: usize,
}

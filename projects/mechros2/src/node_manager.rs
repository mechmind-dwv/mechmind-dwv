// 🚀 Node Manager Module
use std::sync::Arc;
use std::collections::HashMap;
#[cfg(feature = "ros2")]
use r2r::QosProfile;
use serde_json;
use tracing::{info, debug, warn, error};
use crate::SystemState;

pub struct MechNodeManager {
    #[cfg(feature = "ros2")]
    ctx: r2r::Context,
    #[cfg(feature = "ros2")]
    node: r2r::Node,
    #[cfg(feature = "ros2")]
    publishers: HashMap<String, r2r::Publisher<r2r::std_msgs::msg::String>>,
    subscribers: HashMap<String, Arc<tokio::sync::Mutex<Vec<String>>>>,
}

impl MechNodeManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("🔧 Inicializando Node Manager...");

        #[cfg(feature = "ros2")]
        {
            let ctx = r2r::Context::create()?;
            let node = r2r::Node::create(ctx.clone(), "mechros2_hub", "")?;

            let mut manager = Self {
                ctx,
                node,
                publishers: HashMap::new(),
                subscribers: HashMap::new(),
            };

            manager.setup_default_topics().await?;
            info!("✅ Node Manager (ROS2) inicializado");
            Ok(manager)
        }

        #[cfg(not(feature = "ros2"))]
        {
            warn!("⚠️ ROS2 no habilitado en la compilación. Usando modo offline.");
            Ok(Self {
                subscribers: HashMap::new(),
            })
        }
    }

    #[cfg(feature = "ros2")]
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
        let _goal_sub = self.node.create_subscription::<r2r::std_msgs::msg::String>(
            "/mechros2/navigation_goals",
            QosProfile::default(),
            {
                let buffer = goal_buffer.clone();
                move |msg| {
                    let buffer = buffer.clone();
                    tokio::spawn(async move {
                        let mut msgs = buffer.lock().await;
                        msgs.push(msg.data.clone());
                        if msgs.len() > 10 { msgs.remove(0); }
                        debug!("📨 Nuevo objetivo de navegación recibido: {}", msg.data);
                    });
                }
            }
        )?;
        self.subscribers.insert("navigation_goals".to_string(), goal_buffer);

        info!("📡 Topics configurados correctamente");
        Ok(())
    }

    pub async fn publish_system_state(&self, _state: &SystemState) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "ros2")]
        if let Some(publisher) = self.publishers.get("system_state") {
            let json_state = serde_json::to_string(_state)?;
            let msg = r2r::std_msgs::msg::String { data: json_state };
            publisher.publish(&msg)?;
            debug!("📊 Estado del sistema publicado");
        }
        Ok(())
    }

    pub async fn publish_telemetry(&self, _data: &str) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "ros2")]
        if let Some(publisher) = self.publishers.get("telemetry") {
            let msg = r2r::std_msgs::msg::String { data: _data.to_string() };
            publisher.publish(&msg)?;
            debug!("📈 Telemetría publicada");
        }
        Ok(())
    }

    pub async fn publish_command(&self, _command: &str) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "ros2")]
        if let Some(publisher) = self.publishers.get("commands") {
            let msg = r2r::std_msgs::msg::String { data: _command.to_string() };
            publisher.publish(&msg)?;
            debug!("🎯 Comando publicado: {}", _command);
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

    #[cfg(feature = "ros2")]
    pub async fn spin_once(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.ctx.spin_once(std::time::Duration::from_millis(10))?;
        Ok(())
    }
}

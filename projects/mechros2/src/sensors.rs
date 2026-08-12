// 🔬 Sensor Hub Module
// File: projects/mechros2/src/sensors.rs

use std::sync::Arc;
use nalgebra::{Vector3, Point3};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn};
use crate::node_manager::MechNodeManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub position: Option<Point3<f64>>,
    pub velocity: Option<Vector3<f64>>,
    pub acceleration: Option<Vector3<f64>>,
    pub angular_velocity: Option<Vector3<f64>>,
    pub orientation: Option<Vector3<f64>>, // Euler angles
    pub temperature: Option<f32>,
    pub battery_level: Option<f32>,
    pub pressure: Option<f32>,
    pub light_level: Option<f32>,
    pub proximity_sensors: Vec<f32>,
    pub imu_data: Option<ImuData>,
    pub gps_data: Option<GpsData>,
    pub lidar_data: Option<LidarData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuData {
    pub linear_acceleration: Vector3<f64>,
    pub angular_velocity: Vector3<f64>,
    pub orientation: Vector3<f64>,
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub speed: f32,
    pub heading: f32,
    pub satellites: u8,
    pub accuracy: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LidarData {
    pub ranges: Vec<f32>,
    pub intensities: Vec<f32>,
    pub angle_min: f32,
    pub angle_max: f32,
    pub angle_increment: f32,
    pub range_min: f32,
    pub range_max: f32,
}

pub struct SensorHub {
    node_manager: Arc<MechNodeManager>,
    imu_sensor: ImuSensor,
    gps_sensor: GpsSensor,
    lidar_sensor: LidarSensor,
    environmental_sensors: EnvironmentalSensors,
    proximity_sensors: ProximitySensors,
    battery_monitor: BatteryMonitor,
}

impl SensorHub {
    pub async fn new(node_manager: Arc<MechNodeManager>) -> Result<Self, Box<dyn std::error::Error>> {
        info!("🔬 Inicializando Sensor Hub...");

        let sensor_hub = Self {
            node_manager,
            imu_sensor: ImuSensor::new().await?,
            gps_sensor: GpsSensor::new().await?,
            lidar_sensor: LidarSensor::new().await?,
            environmental_sensors: EnvironmentalSensors::new().await?,
            proximity_sensors: ProximitySensors::new().await?,
            battery_monitor: BatteryMonitor::new().await?,
        };

        info!("✅ Sensor Hub inicializado");
        Ok(sensor_hub)
    }

    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔧 Inicializando sensores...");

        self.imu_sensor.calibrate().await?;
        self.gps_sensor.start_acquisition().await?;
        self.lidar_sensor.configure().await?;
        self.environmental_sensors.initialize().await?;
        self.proximity_sensors.configure().await?;
        self.battery_monitor.start_monitoring().await?;

        info!("✅ Todos los sensores inicializados correctamente");
        Ok(())
    }

    pub async fn update_sensors(&mut self) -> Result<SensorData, Box<dyn std::error::Error>> {
        let timestamp = chrono::Utc::now();

        // Recopilar datos de todos los sensores
        let imu_data = self.imu_sensor.read_data().await.ok();
        let gps_data = self.gps_sensor.read_data().await.ok();
        let lidar_data = self.lidar_sensor.scan().await.ok();
        let env_data = self.environmental_sensors.read_all().await.ok();
        let proximity_data = self.proximity_sensors.read_all().await.ok();
        let battery_level = self.battery_monitor.get_level().await.ok();

        // Calcular posición y velocidad basada en GPS e IMU
        let position = if let Some(gps) = &gps_data {
            Some(Point3::new(gps.latitude, gps.longitude, gps.altitude))
        } else {
            None
        };

        let velocity = if let Some(imu) = &imu_data {
            Some(imu.linear_acceleration) // Integración simplificada
        } else {
            None
        };

        let acceleration = imu_data.as_ref().map(|imu| imu.linear_acceleration);
        let angular_velocity = imu_data.as_ref().map(|imu| imu.angular_velocity);
        let orientation = imu_data.as_ref().map(|imu| imu.orientation);

        let sensor_data = SensorData {
            timestamp,
            position,
            velocity,
            acceleration,
            angular_velocity,
            orientation,
            temperature: env_data.as_ref().map(|env| env.temperature),
            battery_level,
            pressure: env_data.as_ref().map(|env| env.pressure),
            light_level: env_data.as_ref().map(|env| env.light_level),
            proximity_sensors: proximity_data.unwrap_or_default(),
            imu_data,
            gps_data,
            lidar_data,
        };

        // Publicar datos de sensores
        let telemetry_json = serde_json::to_string(&sensor_data)?;
        self.node_manager.publish_telemetry(&telemetry_json).await?;

        debug!("📊 Datos de sensores actualizados: {} sensores activos",
               self.count_active_sensors(&sensor_data));

        Ok(sensor_data)
    }

    fn count_active_sensors(&self, data: &SensorData) -> usize {
        let mut count = 0;
        if data.imu_data.is_some() { count += 1; }
        if data.gps_data.is_some() { count += 1; }
        if data.lidar_data.is_some() { count += 1; }
        if data.temperature.is_some() { count += 1; }
        if data.battery_level.is_some() { count += 1; }
        if data.pressure.is_some() { count += 1; }
        if !data.proximity_sensors.is_empty() { count += 1; }
        count
    }

    pub async fn get_sensor_status(&self) -> SensorStatus {
        SensorStatus {
            imu_online: self.imu_sensor.is_online().await,
            gps_online: self.gps_sensor.is_online().await,
            lidar_online: self.lidar_sensor.is_online().await,
            environmental_online: self.environmental_sensors.is_online().await,
            proximity_online: self.proximity_sensors.is_online().await,
            battery_monitor_online: self.battery_monitor.is_online().await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SensorStatus {
    pub imu_online: bool,
    pub gps_online: bool,
    pub lidar_online: bool,
    pub environmental_online: bool,
    pub proximity_online: bool,
    pub battery_monitor_online: bool,
}

// Implementaciones de sensores individuales
pub struct ImuSensor {
    calibrated: bool,
}

impl ImuSensor {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { calibrated: false })
    }

    async fn calibrate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("🎯 Calibrando IMU...");
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        self.calibrated = true;
        info!("✅ IMU calibrado");
        Ok(())
    }

    async fn read_data(&self) -> Result<ImuData, Box<dyn std::error::Error>> {
        if !self.calibrated {
            return Err("IMU no calibrado".into());
        }
        Ok(ImuData {
            linear_acceleration: Vector3::new(0.1, 0.0, 9.81),
            angular_velocity: Vector3::new(0.01, 0.02, 0.0),
            orientation: Vector3::new(0.0, 0.0, 0.0),
            temperature: 25.0,
        })
    }

    async fn is_online(&self) -> bool { self.calibrated }
}

pub struct GpsSensor {
    active: bool,
}

impl GpsSensor {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { active: false })
    }

    async fn start_acquisition(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.active = true;
        Ok(())
    }

    async fn read_data(&self) -> Result<GpsData, Box<dyn std::error::Error>> {
        if !self.active { return Err("GPS no activo".into()); }
        Ok(GpsData {
            latitude: 40.4168,
            longitude: -3.7038,
            altitude: 657.0,
            speed: 0.0,
            heading: 0.0,
            satellites: 12,
            accuracy: 0.5,
        })
    }

    async fn is_online(&self) -> bool { self.active }
}

pub struct LidarSensor {
    active: bool,
}

impl LidarSensor {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { active: false })
    }

    async fn configure(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.active = true;
        Ok(())
    }

    async fn scan(&self) -> Result<LidarData, Box<dyn std::error::Error>> {
        if !self.active { return Err("Lidar no activo".into()); }
        Ok(LidarData {
            ranges: vec![1.0; 360],
            intensities: vec![0.5; 360],
            angle_min: 0.0,
            angle_max: 6.28,
            angle_increment: 0.017,
            range_min: 0.1,
            range_max: 10.0,
        })
    }

    async fn is_online(&self) -> bool { self.active }
}

pub struct EnvironmentalSensors {
    active: bool,
}

impl EnvironmentalSensors {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { active: false })
    }

    async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.active = true;
        Ok(())
    }

    async fn read_all(&self) -> Result<EnvData, Box<dyn std::error::Error>> {
        Ok(EnvData { temperature: 24.5, pressure: 1013.25, light_level: 500.0 })
    }

    async fn is_online(&self) -> bool { self.active }
}

pub struct EnvData {
    pub temperature: f32,
    pub pressure: f32,
    pub light_level: f32,
}

pub struct ProximitySensors {
    active: bool,
}

impl ProximitySensors {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { active: false })
    }

    async fn configure(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.active = true;
        Ok(())
    }

    async fn read_all(&self) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        Ok(vec![0.5; 8])
    }

    async fn is_online(&self) -> bool { self.active }
}

pub struct BatteryMonitor {
    active: bool,
}

impl BatteryMonitor {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { active: false })
    }

    async fn start_monitoring(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.active = true;
        Ok(())
    }

    async fn get_level(&self) -> Result<f32, Box<dyn std::error::Error>> {
        Ok(95.0)
    }

    async fn is_online(&self) -> bool { self.active }
}

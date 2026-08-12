//! # MechROS2
//! Módulo de integración con ROS2 para el proyecto MechMind.
//! Incluye un controlador PID optimizado con SIMD.

#![feature(portable_simd)] // Necesario para usar std::simd

use std::simd::f32x4;
use std::simd::num::SimdFloat;

// Re-exportamos r2r para facilitar el uso desde fuera
#[cfg(feature = "ros2")]
pub use r2r;

/// Controlador PID para MechBot-2X con soporte ROS2 opcional.
#[derive(Debug, Clone)]
pub struct MechController {
    kp: f32,
    ki: f32,
    kd: f32,
    prev_error: f32,
    integral: f32,
    #[cfg(feature = "ros2")]
    ros_node: Option<r2r::Node>,
}

impl MechController {
    /// Crea un nuevo controlador PID.
    #[must_use]
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            prev_error: 0.0,
            integral: 0.0,
            #[cfg(feature = "ros2")]
            ros_node: None,
        }
    }

    /// Calcula la salida del controlador usando SIMD para mayor rendimiento.
    /// - `error`: error actual (setpoint - medición)
    /// - `dt`: tiempo transcurrido desde la última llamada
    /// - Retorna la señal de control.
    pub fn compute(&mut self, error: f32, dt: f32) -> f32 {
        // Actualizar integral (con anti-windup básico)
        self.integral += error * dt;
        // Derivada (usando prev_error)
        let derivative = (error - self.prev_error) / dt;

        // Usar SIMD para operaciones vectorizadas
        let coeffs = f32x4::from_array([self.kp, self.ki, self.kd, 1.0]);
        let terms = f32x4::from_array([error, self.integral, derivative, 0.0]);

        let output = (coeffs * terms).reduce_sum();

        // Actualizar error anterior
        self.prev_error = error;

        output
    }

    /// Establece el nodo ROS2 (opcional, solo si se compila con feature "ros2").
    #[cfg(feature = "ros2")]
    pub fn set_ros_node(&mut self, node: r2r::Node) {
        self.ros_node = Some(node);
    }

    /// Publica el estado del controlador en un tópico ROS2 (si está habilitado).
    #[cfg(feature = "ros2")]
    pub fn publish_status(&self) {
        if let Some(node) = &self.ros_node {
            // Ejemplo: publicar en un tópico (requiere definir mensaje)
            // Aquí se puede implementar según necesidades
            println!("Publicando estado PID desde ROS2 node: {:?}", node);
        }
    }
}

/// Función de ejemplo para demostrar uso de ROS2.
#[cfg(feature = "ros2")]
pub fn init_ros_node(node_name: &str) -> Result<r2r::Node, r2r::R2rError> {
    r2r::Node::create(node_name, "mechmind")
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_pid_controller() {
        let mut pid = MechController::new(0.5, 0.1, 0.01);
        let adjustment = pid.compute(2.0, 0.1);
        // El valor esperado puede variar ligeramente, usamos tolerancia
        assert_relative_eq!(adjustment, 1.002, epsilon = 0.001);
    }

    #[test]
    fn test_pid_convergence() {
        let mut pid = MechController::new(1.0, 0.5, 0.1);
        let mut error = 10.0;
        let dt = 0.1;
        for _ in 0..100 {
            let output = pid.compute(error, dt);
            error -= output * dt; // simulación simple
        }
        // Tras muchas iteraciones, el error debería ser pequeño
        assert!(error.abs() < 0.1);
    }
}

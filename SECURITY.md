## 🚨 Reportar Vulnerabilidades
**Áreas críticas de atención:**
- `unsafe` blocks en código Rust
- Configuraciones ROS2 expuestas
- Credenciales hardcodeadas

**Canales seguros:**
- 📧 Email: [security@mechmind.tech](mailto:security@mechmind.tech) (PGP: `0xDEADBEEF`)
- 🔐 [Formulario seguro](https://github.com/mechmind-dwv/security) (requiere autenticación GitHub)
- 🤖 Issue con label `[SECURITY]`

## 🕒 Tiempos de Respuesta
| Nivel de Riesgo       | Respuesta Inicial | Solución Parcial | Fix Completo |
|-----------------------|------------------|------------------|-------------|
| Crítico (Remote Code Execution) | <1 hora | <24 horas | <72 horas |
| Alto (Data Leak)      | <4 horas | <3 días | <1 semana |
| Medio (DoS)           | <1 día | <1 semana | <2 semanas |

## 🛡️ Prácticas de Seguridad

### 1. Para Código Rust
```rust
// ❌ Inseguro
unsafe { std::ptr::read(0x0 as *const u32) };

// ✅ Seguro
let value = some_safe_api.get_data();
```

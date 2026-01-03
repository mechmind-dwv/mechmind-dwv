# 🤖 Arquitectura MechMind-dwv v2.0

```mermaid
graph TD
    A[GitHub Actions] --> B[Security Scan]
    A --> C[Documentation]
    A --> D[Stats]
    B --> E[Notify]
    C --> E
    D --> E

🔧 Componentes Clave

    Sistema de Seguridad:
    CodeQL con consultas personalizadas

    Escaneo diario automático

    Documentación Inteligente:
    Bilingüe (ES/EN)
    Estilos personalizados

    Despliegue en GitHub Pages

    Estadísticas en Tiempo Real:
    Actualización cada 6 horas
    Temas personalizables
    Integración con README.md

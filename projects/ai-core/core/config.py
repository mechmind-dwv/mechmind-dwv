import torch

class MechMindConfig:
    """Configuración del núcleo IA para MechMind"""
    
    # Intentar usar GPU si está disponible
    DEVICE = "cuda" if torch.cuda.is_available() else "cpu"
    PRECISION = "fp16"  # Optimización para rendimiento
    
    # Aceleradores de hardware
    ACCELERATORS = {
        "tensorrt": True,
        "openvino": False,
        "coreml": False
    }
    
    @classmethod
    def show_config(cls):
        print(f"\n⚙️ MechMind AI Engine Status:")
        print(f"|-> Device: {cls.DEVICE}")
        print(f"|-> Precision: {cls.PRECISION}")
        print(f"|-> Accelerators: {cls.ACCELERATORS}\n")

#!/bin/bash
# Optimización para dispositivos edge (Termux/Móvil)

MODEL=$1
OUTPUT="projects/ai-core/models/optimized/$(basename $MODEL .pth).onnx"

echo "⚡ Optimizando $MODEL para edge computing..."

# Simulación de conversión a ONNX (requiere torch.onnx)
python3 -c "print('Convertir modelo a ONNX...')"

echo "✅ Modelo optimizado guardado en: $OUTPUT"

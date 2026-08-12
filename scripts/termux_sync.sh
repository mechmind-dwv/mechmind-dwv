#!/data/data/com.termux/files/usr/bin/bash

# ==============================================================================
# 🤖 MechMind Termux Sync Script (v1.1)
# Descripción: Script automatizado para sincronizar y subir cambios al repositorio
#              mechmind-dwv desde Termux en Android.
# ==============================================================================

set -e

# Colores para salida visual
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}====================================================${NC}"
echo -e "${BLUE}    🤖 MechMind Sync - Termux Automation Tool       ${NC}"
echo -e "${BLUE}====================================================${NC}"

# 1. Verificar dependencias en Termux
echo -e "\n${GREEN}[1/5] Verificando dependencias (git, curl)...${NC}"
pkg update -y > /dev/null 2>&1 || true
pkg install -y git curl jq > /dev/null 2>&1 || true

# 2. Configurar identidad de Git
echo -e "\n${GREEN}[2/5] Configurando identidad de Git...${NC}"
git config --global user.name "mechmind-dwv"
git config --global user.email "ia.mechmind@gmail.com"

# 3. Solicitar Token de GitHub de forma segura
echo -e "\n${GREEN}[3/5] Autenticación con GitHub...${NC}"
read -sp "Introduce tu Personal Access Token (PAT) de GitHub: " GITHUB_TOKEN
echo ""

if [ -z "$GITHUB_TOKEN" ]; then
    echo -e "${RED}❌ Error: El token no puede estar vacío.${NC}"
    exit 1
fi

# 4. Configurar URL remota con autenticación integrada
REPO_URL="https://mechmind-dwv:${GITHUB_TOKEN}@github.com/mechmind-dwv/mechmind-dwv.git"

echo -e "\n${GREEN}[4/5] Preparando y añadiendo archivos al repositorio...${NC}"
if [ ! -d ".git" ]; then
    echo -e "${RED}❌ Error: Este script debe ejecutarse dentro de la carpeta raíz del repositorio mechmind-dwv.${NC}"
    exit 1
fi

# Obtener la rama actual
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
echo -e "Trabajando en la rama: ${BLUE}${CURRENT_BRANCH}${NC}"

git remote set-url origin "$REPO_URL"
git add .

# 5. Realizar commit y push
echo -e "\n${GREEN}[5/5] Realizando commit y subiendo cambios a GitHub...${NC}"
git commit -m "🚀 AUTO-SYNC: Actualización desde Termux (Android) - $(date +'%Y-%m-%d %H:%M:%S')" || echo "No hay cambios nuevos para confirmar."

if git push origin "$CURRENT_BRANCH"; then
    echo -e "\n${GREEN}✨ ¡Sincronización completada con éxito en la rama ${CURRENT_BRANCH}!${NC}"
else
    echo -e "\n${RED}❌ Error al subir los cambios. Por favor, verifica que tu token tenga permisos de 'Contents: Read & Write'.${NC}"
    exit 1
fi

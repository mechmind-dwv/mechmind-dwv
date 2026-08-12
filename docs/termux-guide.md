# Guía de Operación en Termux (Android)

Este documento detalla la solución a los errores más frecuentes al compilar, instalar dependencias y operar **MechMind-dwv** directamente desde un dispositivo Android mediante Termux [1] [2].

## 1. Configuración de PNPM y PATH Global

Si al ejecutar `pnpm add -g pnpm` o instalar dependencias globales Termux muestra el error:

```text
[ERROR] The configured global bin directory ".../.local/share/pnpm/bin" is not in PATH
```

### Solución

1. Ejecute el comando de configuración asistida de pnpm:
   ```bash
   pnpm setup
   ```
2. Recargue el perfil de su shell (por ejemplo, Bash o Zsh):
   ```bash
   source ~/.bashrc
   # O si usa zsh: source ~/.zshrc
   ```

---

## 2. Ignorar o Aprobar Scripts de Compilación (Supply-Chain Policies)

Si pnpm se detiene con el error `[ERR_PNPM_IGNORED_BUILDS]` al compilar paquetes como `core-js` o `unrs-resolver`:

### Solución

Puede permitir la ejecución de los scripts de compilación requeridos ejecutando:
```bash
pnpm approve-builds
```
O bien instalar omitiendo políticas estrictas de bloqueo si se encuentra en un entorno de desarrollo aislado:
```bash
pnpm install --ignore-scripts
```

---

## 3. Salir de un Entorno Virtual Python (`venv`)

Si la terminal se encuentra dentro de un entorno virtual y el comando `deactivat` falla por error tipográfico (`deactive`):

### Solución

El comando correcto es **`deactivate`**:
```bash
deactivate
```

---

## 4. Duplicación de Clones Git

Si intenta clonar el repositorio y recibe:
```text
fatal: destination path 'mechmind-dwv' already exists and is not an empty directory
```

### Solución

No necesita clonar nuevamente si ya se encuentra en el directorio raíz o si la carpeta existe. Simplemente entre a la carpeta y actualice el repositorio:
```bash
cd ~/mechmind-dwv
git pull origin main
```

---

## 5. Conflicto entre NVM y la variable PREFIX en Termux

Si al ejecutar `source ~/.bashrc` aparece el error:
```text
nvm is not compatible with the "PREFIX" environment variable: currently set to "/data/data/com.termux/files/usr"
Run `unset PREFIX` to unset it.
```

### Solución

Desconfigure la variable de entorno `PREFIX` antes de cargar NVM o ejecute en su terminal:
```bash
unset PREFIX
source ~/.bashrc
```

---

## 6. Ramas Git Divergentes al actualizar con `git pull`

Si al ejecutar `git pull origin main` la consola indica:
```text
fatal: Need to specify how to reconcile divergent branches.
```

### Solución

Indique a Git cómo desea conciliar los cambios locales y remotos (por ejemplo, haciendo un fast-forward o un rebase):
```bash
git config pull.rebase false
git pull origin main
# O si prefiere forzar los cambios remotos conservando la base:
git fetch origin main
git reset --hard origin/main
```

---

## 7. Ausencia de Rustup en Termux (`cargo +nightly` no encontrado)

Si recibe el error:
```text
error: no such command: `+nightly`
help: invoke `cargo` through `rustup` to handle `+toolchain` directives
```

### Solución

En algunas instalaciones nativas de Termux, el paquete `rust` de apt provee `cargo` estable pero no incluye el gestor `rustup`. Para instalar `rustup` y el toolchain Nightly necesario para `portable_simd`:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup toolchain install nightly
cargo +nightly --version
```

---

## Referencias

[1] Repositorio oficial de MechMind-dwv: [https://github.com/mechmind-dwv/mechmind-dwv](https://github.com/mechmind-dwv/mechmind-dwv) [1]

[2] Documentación de arquitectura técnica en `./docs/architecture/` [2]

<p align="center"><a href="../README.md">English</a> | Español</p>
<p align="center"><sub>Esta traducción corresponde a la versión 0.3.1 del README en inglés.</sub></p>
<p align="center"><a href="https://laaielichter.github.io/Agentdisk/"><img src="assets/social.jpg" alt="agentdisk: despeja el desorden y conserva el trabajo." width="1000"></a></p>
<p align="center"><img src="logo.svg" width="60" height="60" alt="Ícono de unidad de agentdisk"></p>
<h1 align="center">agentdisk</h1>
<p align="center"><a href="https://laaielichter.github.io/Agentdisk/">Sitio web</a> · <a href="https://laaielichter.github.io/Agentdisk/#install">Obtener agentdisk</a> · <a href="clients.md">Conecta tu agente</a></p>
<p align="center"><b>El informe honesto del espacio en disco para agentes de programación con IA: descubre lo que Claude Code, Cursor, Codex y otros dejaron en tu disco, y limpia con opción de deshacer.</b></p>
<p align="center">
  <a href="../LICENSE"><img alt="Licencia: MIT o Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%7C%20Apache--2.0-blue"></a>
  <img alt="Plataformas: Windows, macOS y Linux" src="https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-informational">
  <img alt="Servidor MCP" src="https://img.shields.io/badge/MCP-server-8A2BE2">
  <a href="https://github.com/laaielichter/Agentdisk/stargazers"><img alt="Estrellas en GitHub" src="https://img.shields.io/github/stars/laaielichter/Agentdisk?style=social"></a>
</p>
<p align="center">
  <img src="demo.gif" alt="agentdisk analiza worktrees de agentes y carpetas de compilación, crea un plan de limpieza y explica por qué conserva un worktree que contiene un archivo .env" width="900">
  <br><sub>Salida real en una máquina de prueba con proyectos de ejemplo.</sub>
</p>

agentdisk es una CLI de un solo binario y un **servidor MCP** que encuentra los worktrees de Git, los puntos de control, las transcripciones y las carpetas duplicadas de `node_modules`, `.venv`, `target/` y `.godot/` que dejan los agentes de programación con IA. Muestra cuánto espacio se puede recuperar *realmente* y solo limpia lo que es seguro, con opción de deshacer.

## Inicio rápido

```sh
agentdisk scan -p ~/Projects
```

Obtendrás tres tablas (datos de agentes y herramientas, worktrees, dependencias y carpetas de compilación), un resumen del espacio recuperable y lo que cada agente ya limpia por su cuenta. No cambia nada en el disco.

Después, si quieres liberar espacio:

```sh
agentdisk plan -p ~/Projects     # a plan; moves nothing
agentdisk apply <plan-id>        # you type a code; items move to quarantine
agentdisk restore <plan-id>      # changed your mind? everything goes back
```

## ¿Por qué agentdisk?

- **Conoce el costo real de los agentes en paralelo.** Cada worktree de un agente tiene sus propias carpetas `node_modules`, `.venv`, `target/` o `.godot/`. agentdisk las reúne en una tabla, separadas por agente.
- **Confía en las cifras.** Los archivos enlazados físicamente (pnpm, uv) se cuentan una sola vez, y el espacio «recuperable» solo incluye los bytes que realmente volverán a estar disponibles.
- **Limpia sin perder trabajo.** Nunca se tocan los worktrees con archivos sin confirmar, un archivo `.env`, un bloqueo de un agente en ejecución o commits que no pertenecen a ninguna rama. Los registros de conversaciones y las memorias nunca se incluyen en los planes.
- **Deshaz cualquier cambio.** La limpieza primero mueve los elementos a cuarentena; su eliminación es un paso posterior e independiente.
- **La persona conserva el control.** Solo una persona en una terminal puede aplicar un plan. Los agentes de IA pueden analizar, explicar y planificar, nada más.

## Instalación

Descarga un binario precompilado para Windows, macOS o Linux desde la
[página de versiones](https://github.com/laaielichter/Agentdisk/releases/latest), descomprímelo y coloca
`agentdisk` en tu `PATH`. `SHA256SUMS.txt` contiene las sumas de verificación.

O compílalo desde el código fuente con [Rust](https://rustup.rs) 1.82 o posterior:

```sh
git clone https://github.com/laaielichter/agentdisk
cd agentdisk
cargo install --locked --path crates/agentdisk-cli
```

En Windows también puedes ejecutar `.\scripts\install.ps1`. Comprueba que funcione con `agentdisk --version`.

## Comandos

| Comando | Qué hace | ¿Cambia el disco? |
|---|---|---|
| `scan [-p dir]` | Datos de agentes y herramientas, worktrees y carpetas de compilación | No |
| `explain <dir>` | Qué es una carpeta, quién la creó, si puede limpiarse y por qué | No |
| `size <dir>` | Tamaño, tamaño en disco y espacio que se liberaría al borrarla | No |
| `advise [-p dir]` | Almacenes compartidos (pnpm, uv), duplicados, cachés, discos de Docker/WSL y ajustes de agentes | No |
| `check` | Aviso rápido cuando queda poco espacio o se acumulan worktrees (para hooks) | No |
| `adapters` | Qué agentes y herramientas reconoce y cuáles encontró aquí | No |
| `plan [-p dir]` | Guarda un plan de limpieza | Escribe un archivo de plan |
| `plans` | Planes guardados, del más reciente al más antiguo, con su tamaño y estado de aplicación | No |
| `apply <id>` | Mueve los elementos del plan a cuarentena (solicita un código) | Mueve archivos |
| `restore <id>` | Los devuelve a su lugar | Mueve archivos |
| `purge` | Elimina los elementos en cuarentena con más de 7 días (te lo solicita) | Elimina |
| `compact <dir>` | Compresión WOF de Windows para carpetas que se leen con frecuencia y cambian poco | Comprime |
| `mcp` | Servidor MCP por stdio | No |

Todos los comandos de solo lectura aceptan `--json`. Guarda una vez la carpeta de tus proyectos con `agentdisk config --init`.
El comando rápido `check` también usa las carpetas de adaptadores configuradas y los límites de aviso `min_free_gb` y
`max_worktrees` (ambos tienen un valor predeterminado de 20). Sus opciones de CLI tienen prioridad sobre esos límites.

## Qué es seguro limpiar

| Tipo | Se incluye en un plan cuando… | Nunca se incluye cuando… |
|---|---|---|
| Worktrees de agentes | están limpios, inactivos desde hace al menos 7 días, desbloqueados y con commits en una rama | tienen archivos sin confirmar o sin seguimiento, archivos ignorados que no son resultados de compilación (como `.env`), están bloqueados o son el worktree principal |
| Carpetas de compilación ([lista completa y marcadores](build-folders.md)) | hay un archivo marcador, Git las ignora, y llevan inactivas el tiempo indicado por `build_days` (14 días de forma predeterminada) | Git las sigue, su editor está abierto (Godot, Unity, Unreal) o todos sus archivos se comparten mediante enlaces físicos |
| Registros y cachés de agentes | están marcados como regenerables, llevan inactivos el tiempo indicado por `worktree_days` (7 días de forma predeterminada) y el agente no está en ejecución | — |
| Transcripciones, puntos de control, memorias, ajustes, modelos y descargas de SDK | — | **siempre se informan; nunca se incluyen en un plan** |

Las ramas siempre se conservan. La cuarentena está en la misma unidad, por lo que `apply` es instantáneo.

## Uso desde tu agente (servidor MCP)

`agentdisk mcp` ofrece herramientas de solo lectura (`agentdisk_scan`, `agentdisk_explain`, `agentdisk_size`,
`agentdisk_advise`, `agentdisk_check`, `agentdisk_quarantine`) y también `agentdisk_plan`. No hay
ninguna herramienta para mover o eliminar archivos.

**Plugin de Claude Code** (servidor MCP, habilidad y comprobación de disco al iniciar una sesión):

```
/plugin marketplace add laaielichter/agentdisk
/plugin install agentdisk@agentdisk
```

¿Ya usas el marketplace de ReversePort? También funciona `/plugin install agentdisk@reverseport`. El plugin inicia
el binario `agentdisk`, así que primero [instálalo](#instalación).

**Cursor, Codex, GitHub Copilot, Gemini CLI, Cline, Windsurf, Kiro, Zed y OpenCode:** consulta
[docs/clients.md](clients.md).

## Agentes y herramientas compatibles

Claude Code · Cursor · OpenAI Codex · GitHub Copilot · Gemini CLI · Cline · Roo Code · Windsurf ·
Kiro · OpenCode · Amp · Aider · Zed · pi · Ollama · LM Studio · caché de Hugging Face · editor Godot ·
ReversePort

Cada uno tiene un archivo TOML pequeño. ¿Falta alguno o cambió una ruta? [Agrega un adaptador](adapters.md).

## Comparación

| | agentdisk | Analizadores de disco (WizTree, dua, WinDirStat) | Limpiadores de compilación (kondo, npkill) | Limpieza propia de los agentes |
|---|---|---|---|---|
| Sabe qué agente creó una carpeta | Sí, 19 agentes y herramientas | No | No | Solo la propia |
| Sabe qué es seguro borrar | Sí, con comprobaciones de Git | No | Solo carpetas de compilación | Solo sus propios datos |
| Cuenta los bytes enlazados físicamente una sola vez | Sí | Algunos | No | — |
| Opción de deshacer | Cuarentena + restauración | No | No | Depende |
| Se puede usar desde agentes de IA | Servidor MCP (solo lectura + plan) | No | No | — |
| Lo que no hace | No tiene interfaz gráfica; todavía cuenta por separado los clones de copia en escritura | | | |

## ¿Por qué no comprimir sin más?

La compresión reduce entre 1,3 y 2 veces el tamaño del código y los resultados de compilación, y casi no ayuda con `.git`, imágenes,
audio y video. El mayor desperdicio son las duplicaciones. `agentdisk advise` señala los almacenes compartidos,
los clones de copia en escritura y los límites propios de los agentes; `agentdisk compact` comprime carpetas que se leen con frecuencia y cambian poco en Windows.

## Preguntas frecuentes

**¿Borrará mi trabajo?** No por sí solo. Los análisis nunca escriben. Un plan no mueve nada. Para `apply` debes
escribir un código y solo se mueven elementos a cuarentena. Solo `purge` elimina, y también te lo solicita.

**¿Puede mi agente de IA limpiar por mí?** Puede crear un plan y mostrarte el comando. Tú decides si lo aplicas.

**¿Envía datos a algún sitio?** No. Se ejecuta localmente y no contiene código de red.

**¿Por qué algo figura como «sin documentar»?** Algunos agentes no documentan dónde guardan los datos. Esas rutas
proceden de informes de usuarios y solo se muestran.

## Limitaciones conocidas

- Los clones de copia en escritura (clonación de bloques de ReFS, clones de APFS y reflinks de Btrfs) todavía se cuentan como espacio separado.
- Detección de uso: Windows se niega a mover carpetas abiertas; Linux comprueba los archivos abiertos; macOS se basa en el tiempo de inactividad y en la detección del editor.
- Las rutas de almacenamiento de los agentes cambian entre versiones; `agentdisk adapters` muestra lo que encontró en tu máquina.

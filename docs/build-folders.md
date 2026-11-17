# Dependency and build folders

agentdisk recognizes a folder only when at least one of its marker files is next to it in the project. A marker such as `ProjectSettings/ProjectVersion.txt` is a path relative to the project folder; `*.uproject` means any file with that extension. Unmarked folders with the same name are left alone.

| Folder | Ecosystem | Marker files (any) | Editor to close | Rebuild |
|---|---|---|---|---|
| `node_modules` | Node | `package.json` | — | `npm/pnpm/yarn install` |
| `.venv` | Python | `pyproject.toml`, `requirements.txt`, `setup.py`, `uv.lock`, `Pipfile`, `poetry.lock` | — | `uv sync / pip install` |
| `venv` | Python | `pyproject.toml`, `requirements.txt`, `setup.py`, `uv.lock`, `Pipfile`, `poetry.lock` | — | `uv sync / pip install` |
| `.tox` | Python | `tox.ini`, `pyproject.toml` | — | `tox` |
| `.pytest_cache` | Python | `pyproject.toml`, `pytest.ini`, `setup.cfg`, `tox.ini` | — | automatic |
| `.mypy_cache` | Python | `pyproject.toml`, `mypy.ini`, `setup.cfg` | — | automatic |
| `.ruff_cache` | Python | `pyproject.toml`, `ruff.toml`, `.ruff.toml` | — | automatic |
| `target` | Rust | `Cargo.toml` | — | `cargo build` |
| `.dart_tool` | Dart | `pubspec.yaml` | — | `dart pub get / flutter pub get` |
| `.godot` | Godot | `project.godot` | `godot` | full reimport when the Godot editor opens |
| `.import` | Godot 3 | `project.godot` | `godot` | full reimport when the Godot editor opens |
| `Library` | Unity | `ProjectSettings/ProjectVersion.txt` | `unity` | Unity recompiles and reimports everything (slow) |
| `Intermediate` | Unreal | `*.uproject` | `unrealeditor` | Unreal rebuilds on next open |
| `DerivedDataCache` | Unreal | `*.uproject` | `unrealeditor` | Unreal rebuilds shaders/assets (slow) |
| `.next` | Node build | `package.json` | — | `next build` |
| `.nuxt` | Node build | `package.json` | — | `nuxt build` |
| `.svelte-kit` | Node build | `package.json` | — | automatic |
| `.turbo` | Node build | `package.json`, `turbo.json` | — | automatic |
| `.parcel-cache` | Node build | `package.json` | — | automatic |
| `.angular` | Node build | `angular.json` | — | automatic |
| `.gradle` | Gradle | `settings.gradle`, `settings.gradle.kts`, `build.gradle`, `build.gradle.kts` | — | `gradle build` |
| `build` | Gradle | `build.gradle`, `build.gradle.kts` | — | `gradle build` |
| `obj` | .NET | `*.csproj`, `*.fsproj`, `*.vbproj` | — | `dotnet build` |
| `.zig-cache` | Zig | `build.zig` | — | `zig build` |
| `zig-out` | Zig | `build.zig` | — | `zig build` |

Finding a recognized folder does not by itself make it eligible for a plan. By default, a build folder must also be git-ignored, contain no tracked files, and have been idle for at least `build_days` (14 days). Godot, Unity, and Unreal folders are kept while their editor is running; files that cannot free space because they are shared through hardlinks are not planned.

# Kioku

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Publish](https://img.shields.io/github/actions/workflow/status/RO-HSA/kioku/publish.yml?label=publish)](https://github.com/RO-HSA/kioku/actions/workflows/publish.yml)
[![Latest Release](https://img.shields.io/github/v/release/RO-HSA/kioku?display_name=tag)](https://github.com/RO-HSA/kioku/releases)

[![React](https://img.shields.io/badge/React-19-20232A?logo=react&logoColor=61DAFB)](https://react.dev/) [![TypeScript](https://img.shields.io/badge/TypeScript-5-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org/) [![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri&logoColor=FFC131)](https://tauri.app/) [![Rust](https://img.shields.io/badge/Rust-1.0%2B-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/) [![Bun](https://img.shields.io/badge/Bun-latest-F9F1E1?logo=bun&logoColor=000000)](https://bun.sh/) [![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-4-06B6D4?logo=tailwindcss&logoColor=white)](https://tailwindcss.com/)

Kioku is an open-source, cross-platform anime tracking desktop app built with Tauri, React, TypeScript, and Rust.
It helps you manage your anime (and soon manga) library, integrated with [MyAnimeList](https://myanimelist.net), [AniList](https://anilist.co), and auto-syncs your library.

## Table of Contents

- [Platform Support](#platform-support)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Build and Distribution](#build-and-distribution)
- [Developer Commands](#developer-commands)
- [Contributing](#contributing)
- [License](#license)
- [Disclaimer](#disclaimer)

## Platform Support

| Platform                      | Status              |
| ----------------------------- | ------------------- |
| Windows                       | Supported           |
| macOS (Apple Silicon + Intel) | Supported           |
| Linux (x64 + ARM)             | Supported           |
| Mobile                        | Partial scaffolding |

## Prerequisites

- [Node.js 22.x +](https://nodejs.org)
- [Bun](https://bun.com)
- [Rust](https://github.com/rust-lang/rustup)
- [Tauri system dependencies for your OS](https://v2.tauri.app/start/prerequisites/)

Linux packages used by CI (Ubuntu/Debian):

```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf xdg-utils
```

## Quick Start

Install dependencies:

```bash
bun install
```

Run the full desktop app (recommended for feature development):

```bash
bun run tauri dev
```

## Build and Distribution

Build frontend assets:

```bash
bun run build
```

Build production desktop bundles:

```bash
bun run tauri build
```

## Developer Commands

| Command               | Description                           |
| --------------------- | ------------------------------------- |
| `bun run dev`         | Start Vite dev server                 |
| `bun run build`       | Type-check and build frontend         |
| `bun run preview`     | Preview built frontend                |
| `bun run tauri dev`   | Run Tauri desktop app in dev mode     |
| `bun run tauri build` | Build production desktop bundles      |
| `bun run lint`        | Run ESLint                            |
| `bun run lint:fix`    | Auto-fix lint issues where possible   |
| `bun run format`      | Format repository files with Prettier |

## Contributing

Contributions are welcome.

1. Fork the repository and create a feature branch.
2. Keep changes focused and commit messages clear.
3. Run `bun run lint` and `bun run format`.
4. Open a pull request with motivation, approach, and impact.

## License

This project is licensed under the GNU General Public License v3.0.
See `LICENSE` for the full text.

## Disclaimer

Kioku is an independent project and is not affiliated with AniList or MyAnimeList.

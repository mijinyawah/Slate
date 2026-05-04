# Slate

Lightweight desktop note-taking app built with Tauri + SvelteKit.

Slate is local-first, fast to open, and intentionally minimal.

## Highlights

- Rich writing support: paragraph, H1-H4, blockquote, bullets, numbered lists, code blocks, divider
- Two toolbar modes: always visible or selection-only floating menu
- Autosave + undo/redo support
- Trash + restore + permanent delete confirmation
- Theme presets + custom theme JSON import/export
- Dark/light toggle + pin window always-on-top
- Sidebar search + note sort controls

## Storage

Notes are saved locally in your Documents folder:

- `~/Documents/Slate/notes/*.json`
- `~/Documents/Slate/trash/*.json`

No account required. No cloud sync in this phase.

## Keyboard shortcuts

- `Cmd/Ctrl + N` new note
- `Cmd/Ctrl + \` toggle sidebar
- `Cmd/Ctrl + Shift + D` toggle dark/light
- `Cmd/Ctrl + ,` open settings
- `Cmd/Ctrl + B` bold
- `Cmd/Ctrl + I` italic
- `Cmd/Ctrl + U` underline
- `Cmd/Ctrl + K` link
- `Cmd/Ctrl + Z` undo
- `Cmd/Ctrl + Shift + Z` redo

## Run locally

Prereqs:

- Node.js 18+
- Rust toolchain
- Tauri v2 prerequisites for your OS

```bash
git clone https://github.com/mijinyawah/Slate.git
cd Slate
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

Build outputs:

- macOS app: `src-tauri/target/release/bundle/macos/slate.app`
- DMG: `src-tauri/target/release/bundle/dmg/slate_0.1.0_aarch64.dmg`

## Tech stack

- Tauri v2
- SvelteKit + TypeScript
- Local filesystem via `@tauri-apps/plugin-fs`

## License

MIT

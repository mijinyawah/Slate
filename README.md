# Slate

Slate is a lightweight desktop note-taking app. Local-first, no accounts, no noise — just you and your notes. Vibe coded with Claude and Codex.

## Features

**Rich text that covers the basics**
Paragraph, H1–H4, bullets, numbered lists, blockquote, code block, divider. Accessible via toolbar or keyboard. Nothing you won't use.

**Stays out of the way**
Pin to always-on-top so it floats above your other windows. Toggle dark/light mode. Everything autosaves. Undo/redo works like you'd expect.

**Your notes, your files**
Notes are stored as plain files on your machine — no account, no sync, no cloud dependency. Readable by any text editor if you ever want to leave.

## Note Storage

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

**Built by a vibe coder for other vibe coders. Make it yours!**
Fork the project and have fun!

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

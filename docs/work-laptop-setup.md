# Slate: rebuild on a work Mac without admin (paste to the other Claude)

I'm setting up Slate (Tauri v2 + SvelteKit) on a corporate Mac where I have NO admin rights. Do not use sudo, Homebrew, or anything that writes outside my home folder. Stop and tell me if a step needs a password.

## 0. Preflight (stop if any fail)
1. `uname -m` → `arm64` = Apple Silicon, `x86_64` = Intel. Note which one.
2. `xcode-select -p` must print a path. If it errors, Xcode Command Line Tools are missing. Installing them usually needs admin, so STOP and tell me to ask IT (or to use the prebuilt app instead). Rust cannot build without them.
3. Check if `node -v` (needs 18+) and `cargo -V` already exist.

## 1. Per-user toolchains (no sudo)
- Node: install fnm or nvm into my home folder, then `fnm install 20` / `nvm install 20`.
- Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y`, then `source "$HOME/.cargo/env"`.
- If the corporate network blocks these downloads, stop and tell me.

## 2. Get the code clean
Prefer `git clone https://github.com/mijinyawah/Slate.git` over a copied folder.
If I gave you a copied folder, delete these before building (they're machine-specific or generated, and gitignored anyway):
`node_modules/`, `src-tauri/target/`, `build/`, `.svelte-kit/`, `src-tauri/gen/schemas/`.
Then confirm `git status` shows nothing missing compared with the repo. There is no .env needed for this project.

## 3. Build
```
npm ci
npm run tauri build                                   # Apple Silicon
# Intel Mac:  rustup target add x86_64-apple-darwin && npm run tauri build -- --target x86_64-apple-darwin
```
Output: `src-tauri/target/release/bundle/macos/slate.app` (Intel: under `target/x86_64-apple-darwin/release/...`).

## 4. Install to my user Applications folder
```
mkdir -p ~/Applications
cp -R src-tauri/target/release/bundle/macos/slate.app ~/Applications/
xattr -dr com.apple.quarantine ~/Applications/slate.app
open ~/Applications/slate.app
```
If the app is killed on launch or a security tool warns, STOP. That's corporate policy (MDM/EDR), not a bug. Don't try to bypass it.

## 5. Notes data
Notes are JSON files in `~/Documents/Slate/notes/` and `~/Documents/Slate/trash/`. If I've given you a copy of my `Slate` folder, quit Slate first (tray icon → Quit), copy it to `~/Documents/Slate/`, then reopen. Themes/settings live inside the app and don't transfer.
Do NOT set up a synced folder between machines: Slate deletes note files it didn't load when it opened, so sync with the app open on two Macs can lose notes.

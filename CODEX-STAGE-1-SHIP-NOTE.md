# Slate — CODEX Stage 1 Ship Note

Date: 2026-05-02
Project: CL-A03 Slate (Scorched-Earth Rebuild)
Path: /Users/hafsah/Claude CoWork Folders/Claude-Cowork-v2/projects/CL-A03-slate/archive/slate
Status: Stage 1 Shipped

## What We Shipped

A fully rebuilt, lightweight desktop notes app (Tauri v2 + SvelteKit) with a clean Numi-inspired UI and local persistence.

Core deliverables:
- Fresh app scaffold from zero (hard reset)
- Local note persistence with autosave
- Search + note list + active note editing
- Rich text editing in contenteditable editor
- Inline formatting toolbar
- Trash + restore flow for deleted notes
- Theme toggle (light/dark)
- Sidebar toggle (clean writing mode)
- Always-on-top pin toggle
- Custom top window chrome (traffic lights + app controls)
- Draggable custom window title region
- Save status indicator at bottom (Saving... / Saved)

## Architecture + Storage

Frontend:
- SvelteKit single-page app UI
- Editor based on contenteditable + `document.execCommand`

Desktop shell:
- Tauri v2 custom window (`decorations: false`, `transparent: true`)
- macOS private API enabled for proper custom-chrome behavior

Local storage keys:
- `slate.notes.v2`: active notes + trash payload
- `slate.notes.v1`: legacy migration source (read + migrate)
- `slate.theme.v1`: persisted theme
- `slate.pin.v1`: persisted always-on-top pin state

Data shape:
- `Note`: id, title, content (HTML), updatedAt
- `TrashedNote`: Note + deletedAt

## Editor Behavior (Stage 1)

Formatting supported:
- Paragraph (`P`)
- Headings (`H1`–`H4`)
- Blockquote
- Bulleted + numbered lists
- Bold, Italic, Strike, Underline
- Code block
- Divider line (`hr`)
- Alignment (left, center, right)

List UX behavior:
- Typing `-` + space or `*` + space auto-starts bullets
- Pressing Enter on an empty bullet exits list mode back to paragraph

Link behavior:
- Pasting plain URLs auto-creates clickable links
- Link tint styled to teal for visual consistency

Visual differentiation:
- Blockquotes use subtle fill + left accent
- Code blocks use subtle filled background + mono font
- Both blockquotes and code blocks are content-width (not full-width bars)

## Window + Chrome Behavior

Custom top bar includes:
- macOS traffic lights (close/minimize/maximize)
- Centered `Slate` drag title region
- Right-side control cluster: theme toggle, list toggle, new note, pin

Move / pin behavior:
- Window can be dragged from the centered title region
- Pin toggles always-on-top and persists state

## Sidebar + Note Management

Implemented:
- Search bar
- Compact note cards with truncation-safe titles
- Per-note delete action in card
- Trash section with restore/permanent remove
- Optional collapsible sidebar for focused writing

## Keyboard Shortcuts

App-level:
- `Cmd/Ctrl + N`: new note
- `Cmd/Ctrl + \\`: toggle sidebar
- `Cmd/Ctrl + Shift + D`: toggle dark/light

Formatting:
- `Cmd/Ctrl + 0`: paragraph
- `Cmd/Ctrl + 1/2/3/4`: heading levels
- `Cmd/Ctrl + Shift + 8`: bullets
- `Cmd/Ctrl + Shift + 9`: blockquote

Undo/redo:
- Toolbar buttons removed by request
- Native undo/redo shortcut behavior remains available

## Tauri Config + Permissions

Window config highlights (`src-tauri/tauri.conf.json`):
- `decorations: false`
- `transparent: true`
- `macOSPrivateApi: true`
- custom size constraints + resizable

Capability permissions (`src-tauri/capabilities/default.json`):
- window close/minimize/maximize
- start dragging
- set always-on-top

## QA Outcome

Validation run:
- `npm run check` -> passing at handoff

## Stage 1 Definition (Met)

Stage 1 goal: stable daily-driver local notes app with polished UI and reliable core writing workflow.

Met in shipped build:
- Fresh clean rebuild
- Stable persistence + migration
- Rich editing baseline
- Deletion safety via trash
- Theme + sidebar + pin + drag
- Visual polish and interaction refinements from iterative QA

## Recommended Stage 2 Backlog (When Reopened)

Potential next features:
- Stronger editor engine (Tiptap/ProseMirror) for robust link + block semantics
- Command palette / slash commands
- Tags / pin-favorites / sorting modes
- Export options (Markdown / PDF / HTML)
- Storage adapter abstraction for future sync
- Global hotkey quick capture window
- Better typography presets + markdown import/export roundtrip

## Handoff Note

This app is in a "ship-ready Stage 1" state for local-first use. Future sessions can reopen from this note and continue with Stage 2 enhancements.

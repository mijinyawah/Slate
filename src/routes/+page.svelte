<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { BaseDirectory, mkdir, readDir, readTextFile, remove, writeTextFile } from '@tauri-apps/plugin-fs';

  type ThemeMode = 'light' | 'dark';
  type ThemeTokenKey =
    | '--bg-1'
    | '--bg-2'
    | '--bg-3'
    | '--bg-4'
    | '--chrome-bg'
    | '--line-1'
    | '--line-2'
    | '--text-1'
    | '--text-2'
    | '--text-3'
    | '--hover-1'
    | '--hover-2'
    | '--chip-bg'
    | '--selection-bg'
    | '--selection-fg'
    | '--settings-bg'
    | '--settings-item-bg'
    | '--accent-caret'
    | '--accent-primary'
    | '--accent-primary-soft'
    | '--link-1'
    | '--link-2';
  type ThemeTokens = Record<ThemeTokenKey, string>;
  type ThemePreset = {
    id: string;
    label: string;
    mode: ThemeMode;
    builtIn?: boolean;
    tokens: ThemeTokens;
  };
  type LineSpacing = 'normal' | 'loose' | 'wide';
  type WordCounterMode = 'words' | 'characters' | 'both';
  type PasteMode = 'plain' | 'basic';
  type CloseBehavior = 'tray' | 'quit';
  type ToolbarMode = 'always' | 'selection';
  type CodeTone = 'plain' | 'accented';
  type AlignMode = 'left' | 'center' | 'right';
  type NoteSortMode = 'recent' | 'oldest' | 'title-asc' | 'title-desc';

  type AppSettings = {
    lineSpacing: LineSpacing;
    uiScale: number;
    wordCounterMode: WordCounterMode;
    noteSortMode: NoteSortMode;
    pasteMode: PasteMode;
    toolbarMode: ToolbarMode;
    codeTone: CodeTone;
    closeBehavior: CloseBehavior;
    confirmPermanentDelete: boolean;
    cursorBlink: boolean;
  };

  type Note = {
    id: string;
    title: string;
    content: string;
    updatedAt: number;
  };

  type TrashedNote = Note & {
    deletedAt: number;
  };

  type NotesPayload = {
    notes: Note[];
    trash: TrashedNote[];
  };

  const STORAGE_KEY = 'slate.notes.v2';
  const LEGACY_STORAGE_KEY = 'slate.notes.v1';
  const THEME_KEY = 'slate.theme.v1';
  const THEME_LIBRARY_KEY = 'slate.theme.library.v1';
  const THEME_MODE_MEMORY_KEY = 'slate.theme.modeMemory.v1';
  const PIN_KEY = 'slate.pin.v1';
  const SETTINGS_KEY = 'slate.settings.v1';
  const DOCUMENT_ROOT = 'Slate';
  const NOTES_DIR = `${DOCUMENT_ROOT}/notes`;
  const TRASH_DIR = `${DOCUMENT_ROOT}/trash`;
  const MAX_HISTORY_ENTRIES = 200;
  const DEFAULT_DARK_THEME_ID = 'theme-midnight-neon';
  const DEFAULT_LIGHT_THEME_ID = 'theme-paper-ledger';
  const DEFAULT_THEME_ID = DEFAULT_DARK_THEME_ID;
  const THEME_TOKEN_KEYS: ThemeTokenKey[] = [
    '--bg-1',
    '--bg-2',
    '--bg-3',
    '--bg-4',
    '--chrome-bg',
    '--line-1',
    '--line-2',
    '--text-1',
    '--text-2',
    '--text-3',
    '--hover-1',
    '--hover-2',
    '--chip-bg',
    '--selection-bg',
    '--selection-fg',
    '--settings-bg',
    '--settings-item-bg',
    '--accent-caret',
    '--accent-primary',
    '--accent-primary-soft',
    '--link-1',
    '--link-2'
  ];
  const BUILTIN_THEMES: ThemePreset[] = [
    {
      id: 'theme-midnight-neon',
      label: 'Midnight Neon',
      mode: 'dark',
      builtIn: true,
      tokens: {
        '--bg-1': '#040507',
        '--bg-2': '#07090c',
        '--bg-3': '#07090b',
        '--bg-4': '#05070a',
        '--chrome-bg': '#080a0e',
        '--line-1': '#171a20',
        '--line-2': '#1b1f26',
        '--text-1': '#38f88e',
        '--text-2': '#5a626f',
        '--text-3': '#3f4651',
        '--hover-1': '#10141a',
        '--hover-2': '#161b22',
        '--chip-bg': '#0f1318',
        '--selection-bg': '#1f4588',
        '--selection-fg': '#e8f1ff',
        '--settings-bg': '#0d1118',
        '--settings-item-bg': '#131924',
        '--accent-caret': '#38f88e',
        '--accent-primary': '#38f88e',
        '--accent-primary-soft': '#1a4630',
        '--link-1': '#47d7cf',
        '--link-2': '#66e6df'
      }
    },
    {
      id: 'theme-plum-nocturne',
      label: 'Plum Nocturne',
      mode: 'dark',
      builtIn: true,
      tokens: {
        '--bg-1': '#181126',
        '--bg-2': '#231833',
        '--bg-3': '#1d142d',
        '--bg-4': '#181127',
        '--chrome-bg': '#24183a',
        '--line-1': '#33254a',
        '--line-2': '#3a2a53',
        '--text-1': '#e5d8ff',
        '--text-2': '#7f739d',
        '--text-3': '#6f638d',
        '--hover-1': '#2a1f40',
        '--hover-2': '#34264d',
        '--chip-bg': '#2a1f3f',
        '--selection-bg': '#5b4a93',
        '--selection-fg': '#f8f3ff',
        '--settings-bg': '#1f1730',
        '--settings-item-bg': '#2b2142',
        '--accent-caret': '#c9afff',
        '--accent-primary': '#c7adff',
        '--accent-primary-soft': '#4a3a73',
        '--link-1': '#9ad9ff',
        '--link-2': '#b6e6ff'
      }
    },
    {
      id: 'theme-atlantic-night',
      label: 'Atlantic Night',
      mode: 'dark',
      builtIn: true,
      tokens: {
        '--bg-1': '#030722',
        '--bg-2': '#09143f',
        '--bg-3': '#081036',
        '--bg-4': '#040a28',
        '--chrome-bg': '#0b1542',
        '--line-1': '#1d2859',
        '--line-2': '#273366',
        '--text-1': '#eaf0ff',
        '--text-2': '#b7bfd6',
        '--text-3': '#7f8ab2',
        '--hover-1': '#142050',
        '--hover-2': '#1f2d66',
        '--chip-bg': '#1a2759',
        '--selection-bg': '#254ea7',
        '--selection-fg': '#f5f8ff',
        '--settings-bg': '#0b1438',
        '--settings-item-bg': '#17234f',
        '--accent-caret': '#4cdcb0',
        '--accent-primary': '#4cdcb0',
        '--accent-primary-soft': '#1d4d53',
        '--link-1': '#67cec5',
        '--link-2': '#85e0d8'
      }
    },
    {
      id: 'theme-paper-ledger',
      label: 'Paper Ledger',
      mode: 'light',
      builtIn: true,
      tokens: {
        '--bg-1': '#f2f0ec',
        '--bg-2': '#ece9e2',
        '--bg-3': '#e8e4dc',
        '--bg-4': '#f7f5f1',
        '--chrome-bg': '#ddd8cd',
        '--line-1': '#d0cbc2',
        '--line-2': '#dad5cd',
        '--text-1': '#231911',
        '--text-2': '#5f5851',
        '--text-3': '#928a80',
        '--hover-1': '#e3dfd7',
        '--hover-2': '#d8d2c8',
        '--chip-bg': '#dcd7ce',
        '--selection-bg': '#b97a1f',
        '--selection-fg': '#fff6e6',
        '--settings-bg': '#ebe7df',
        '--settings-item-bg': '#f6f4ef',
        '--accent-caret': '#bc7a1f',
        '--accent-primary': '#bc7a1f',
        '--accent-primary-soft': '#e8d5b6',
        '--link-1': '#0e7f86',
        '--link-2': '#14939b'
      }
    }
  ];
  const BUILTIN_THEME_IDS = new Set(BUILTIN_THEMES.map((themePreset) => themePreset.id));
  const DEFAULT_SETTINGS: AppSettings = {
    lineSpacing: 'normal',
    uiScale: 1,
    wordCounterMode: 'words',
    noteSortMode: 'recent',
    pasteMode: 'plain',
    toolbarMode: 'always',
    codeTone: 'plain',
    closeBehavior: 'tray',
    confirmPermanentDelete: true,
    cursorBlink: true
  };

  type AppWindowLike = {
    close: () => Promise<void>;
    destroy: () => Promise<void>;
    minimize: () => Promise<void>;
    toggleMaximize: () => Promise<void>;
    setAlwaysOnTop: (value: boolean) => Promise<void>;
  };

  function getSafeAppWindow(): AppWindowLike {
    try {
      return getCurrentWindow();
    } catch {
      // Browser-only dev mode fallback for QA/smoke testing outside Tauri runtime.
      return {
        close: async () => {},
        destroy: async () => {},
        minimize: async () => {},
        toggleMaximize: async () => {},
        setAlwaysOnTop: async () => {}
      };
    }
  }

  const appWindow = getSafeAppWindow();

  let notes = $state<Note[]>([]);
  let trashedNotes = $state<TrashedNote[]>([]);
  let activeNoteId = $state<string | null>(null);
  let searchQuery = $state('');
  let isSaving = $state(false);
  let isSidebarOpen = $state(true);
  let isTrashOpen = $state(false);
  let isSettingsOpen = $state(false);
  let activeThemeId = $state<string>(DEFAULT_THEME_ID);
  let customThemes = $state<ThemePreset[]>([]);
  let themeFileInput = $state<HTMLInputElement | null>(null);
  let isPinned = $state(false);
  let settings = $state<AppSettings>({ ...DEFAULT_SETTINGS });

  let saveTimer: ReturnType<typeof setTimeout> | null = null;
  let persistQueue: Promise<void> = Promise.resolve();
  let editorEl = $state<HTMLDivElement | null>(null);
  let currentEditorNoteId = $state<string | null>(null);
  let showSelectionMenu = $state(false);
  let selectionMenuX = $state(0);
  let selectionMenuY = $state(0);
  let lightboxImageSrc = $state<string | null>(null);
  let lightboxImageCaption = $state('');
  let lastEditorRange: Range | null = null;
  let isFormatMenuOpen = $state(false);
  let formatMenuEl = $state<HTMLDivElement | null>(null);
  let isSelectionFormatMenuOpen = $state(false);
  let selectionFormatMenuEl = $state<HTMLDivElement | null>(null);
  let selectionMenuEl = $state<HTMLDivElement | null>(null);
  const undoHistory = new Map<string, string[]>();
  const redoHistory = new Map<string, string[]>();
  let isRestoringHistory = false;
  let pendingPermanentDeleteNoteId = $state<string | null>(null);

  const allThemes = $derived.by(() => [...BUILTIN_THEMES, ...customThemes]);
  const activeTheme = $derived.by(() => {
    const matched = allThemes.find((themePreset) => themePreset.id === activeThemeId);
    return matched ?? BUILTIN_THEMES.find((themePreset) => themePreset.id === DEFAULT_THEME_ID) ?? BUILTIN_THEMES[0];
  });
  const currentThemeMode = $derived<ThemeMode>(activeTheme.mode);
  const activeThemeCssVars = $derived.by(() =>
    THEME_TOKEN_KEYS.map((token) => `${token}: ${activeTheme.tokens[token]};`).join(' ')
  );
  const isDarkTheme = $derived(currentThemeMode === 'dark');

  const visibleNotes = $derived(sortVisibleNotes(filterNotes(notes, searchQuery), settings.noteSortMode));
  const activeNote = $derived(notes.find((note) => note.id === activeNoteId) ?? null);
  const wordCount = $derived(countWords(textFromHtml(activeNote?.content ?? '')));
  const characterCount = $derived(countCharacters(textFromHtml(activeNote?.content ?? '')));
  const countLabel = $derived.by(() => {
    if (settings.wordCounterMode === 'characters') {
      return `${characterCount}c`;
    }
    if (settings.wordCounterMode === 'both') {
      return `${wordCount}w · ${characterCount}c`;
    }
    return `${wordCount}w`;
  });

  onMount(() => {
    loadTheme();
    loadThemeLibrary();
    loadSettings();
    void loadNotes();
    void loadPinState();

    const onBeforeUnload = (): void => {
      void persistNotes(notes, trashedNotes);
      persistTheme(activeThemeId);
      persistSettings();
      window.localStorage.setItem(PIN_KEY, String(isPinned));
    };
    const onSelectionChange = (): void => {
      rememberEditorSelection();
      updateSelectionMenu();
    };
    const onWindowMouseDown = (event: MouseEvent): void => {
      const target = event.target as Node | null;
      if (!target) {
        return;
      }

      if (isFormatMenuOpen && !formatMenuEl?.contains(target)) {
        isFormatMenuOpen = false;
      }

      if (isSelectionFormatMenuOpen && !selectionFormatMenuEl?.contains(target)) {
        isSelectionFormatMenuOpen = false;
      }
    };

    const onKeydown = (event: KeyboardEvent): void => {
      const mod = event.metaKey || event.ctrlKey;
      if (!mod) {
        return;
      }

      const key = event.key.toLowerCase();

      if (key === 'n') {
        event.preventDefault();
        createNote();
        return;
      }

      if (key === '\\') {
        event.preventDefault();
        toggleSidebar();
        return;
      }

      if (key === 'd' && event.shiftKey) {
        event.preventDefault();
        toggleTheme();
        return;
      }

      if (key === ',') {
        event.preventDefault();
        isSettingsOpen = !isSettingsOpen;
        return;
      }

      if (key === 'escape') {
        isFormatMenuOpen = false;
        isSelectionFormatMenuOpen = false;
        return;
      }

      if (key === 'w') {
        event.preventDefault();
        void closeApp();
        return;
      }

      if (!activeNote || !editorEl || !isEventInsideEditor(event.target)) {
        return;
      }

      if (key === 'b') {
        event.preventDefault();
        applyBold();
        return;
      }

      if (key === 'i') {
        event.preventDefault();
        applyItalic();
        return;
      }

      if (key === 'u') {
        event.preventDefault();
        applyUnderline();
        return;
      }

      if (key === 'k') {
        event.preventDefault();
        promptLink();
        return;
      }

      if (key === 'z' && !event.shiftKey) {
        event.preventDefault();
        applyUndo();
        return;
      }

      if ((key === 'z' && event.shiftKey) || (!event.metaKey && event.ctrlKey && key === 'y')) {
        event.preventDefault();
        applyRedo();
        return;
      }

      if (key === '0') {
        event.preventDefault();
        applyParagraph();
        return;
      }

      if (key === '1') {
        event.preventDefault();
        applyHeading(1);
        return;
      }

      if (key === '2') {
        event.preventDefault();
        applyHeading(2);
        return;
      }

      if (key === '3') {
        event.preventDefault();
        applyHeading(3);
        return;
      }

      if (key === '4') {
        event.preventDefault();
        applyHeading(4);
        return;
      }

      if (key === '8' && event.shiftKey) {
        event.preventDefault();
        applyBullets();
        return;
      }

      if (key === '9' && event.shiftKey) {
        event.preventDefault();
        applyBlockquote();
      }
    };

    window.addEventListener('beforeunload', onBeforeUnload);
    window.addEventListener('keydown', onKeydown);
    window.addEventListener('mousedown', onWindowMouseDown);
    document.addEventListener('selectionchange', onSelectionChange);

    return () => {
      window.removeEventListener('beforeunload', onBeforeUnload);
      window.removeEventListener('keydown', onKeydown);
      window.removeEventListener('mousedown', onWindowMouseDown);
      document.removeEventListener('selectionchange', onSelectionChange);

      if (saveTimer) {
        clearTimeout(saveTimer);
      }

      void persistNotes(notes, trashedNotes);
      persistTheme(activeThemeId);
      persistSettings();
    };
  });

  $effect(() => {
    if (!editorEl) {
      return;
    }

    if (!activeNote) {
      editorEl.innerHTML = '';
      currentEditorNoteId = null;
      lastEditorRange = null;
      return;
    }

    if (activeNote.id !== currentEditorNoteId) {
      currentEditorNoteId = activeNote.id;
      editorEl.innerHTML = activeNote.content || '<p></p>';
      ensureEmptyBlocksHaveLineBreak();
      if (!undoHistory.has(activeNote.id)) {
        undoHistory.set(activeNote.id, [editorEl.innerHTML]);
      }
      if (!redoHistory.has(activeNote.id)) {
        redoHistory.set(activeNote.id, []);
      }
      lastEditorRange = null;
    }
  });

  $effect(() => {
    if (typeof document === 'undefined') {
      return;
    }

    document.documentElement.dataset.theme = currentThemeMode;
    document.documentElement.dataset.themePreset = activeThemeId;
  });

  function createId(): string {
    if (typeof crypto !== 'undefined' && 'randomUUID' in crypto) {
      return crypto.randomUUID();
    }

    return `note-${Date.now()}-${Math.floor(Math.random() * 10000)}`;
  }

  function createBlankNote(): Note {
    return {
      id: createId(),
      title: 'Untitled note',
      content: '<p></p>',
      updatedAt: Date.now()
    };
  }

  function createWelcomeNote(): Note {
    return {
      id: createId(),
      title: 'Quick note',
      content: '<h1>Quick note</h1><p>Start writing. Your notes save automatically.</p>',
      updatedAt: Date.now()
    };
  }

  function deriveTitleFromText(text: string): string {
    const firstLine = text
      .split(/\r?\n/u)
      .map((line) => line.trim())
      .find((line) => line.length > 0);

    return firstLine || 'Untitled note';
  }

  function textFromHtml(content: string): string {
    if (typeof document === 'undefined') {
      return content.replace(/<[^>]+>/gu, ' ');
    }

    const temp = document.createElement('div');
    temp.innerHTML = content;
    return temp.textContent ?? '';
  }

  function filterNotes(input: Note[], query: string): Note[] {
    const normalized = query.trim().toLowerCase();
    if (!normalized) {
      return input;
    }

    return input.filter((note) => {
      const haystack = `${note.title}\n${textFromHtml(note.content)}`.toLowerCase();
      return haystack.includes(normalized);
    });
  }

  function sortVisibleNotes(input: Note[], mode: NoteSortMode): Note[] {
    if (mode === 'oldest') {
      return [...input].sort((a, b) => a.updatedAt - b.updatedAt);
    }

    if (mode === 'title-asc') {
      return [...input].sort((a, b) => a.title.localeCompare(b.title, undefined, { sensitivity: 'base' }));
    }

    if (mode === 'title-desc') {
      return [...input].sort((a, b) => b.title.localeCompare(a.title, undefined, { sensitivity: 'base' }));
    }

    return [...input].sort((a, b) => b.updatedAt - a.updatedAt);
  }

  function countWords(content: string): number {
    const trimmed = content.trim();
    return trimmed ? trimmed.split(/\s+/u).length : 0;
  }

  function countCharacters(content: string): number {
    return content.trim().length;
  }

  function formatUpdatedAt(timestamp: number): string {
    const date = new Date(timestamp);
    return new Intl.DateTimeFormat(undefined, {
      month: 'short',
      day: 'numeric',
      hour: 'numeric',
      minute: '2-digit'
    }).format(date);
  }

  function sortNotes<T extends { updatedAt: number }>(input: T[]): T[] {
    return [...input].sort((a, b) => b.updatedAt - a.updatedAt);
  }

  function sanitizeNote(item: unknown): Note | null {
    if (!item || typeof item !== 'object') {
      return null;
    }

    const candidate = item as Partial<Note>;
    if (typeof candidate.id !== 'string' || typeof candidate.content !== 'string') {
      return null;
    }

    const plainText = textFromHtml(candidate.content);
    const title = typeof candidate.title === 'string' ? candidate.title : deriveTitleFromText(plainText);
    const updatedAt = typeof candidate.updatedAt === 'number' ? candidate.updatedAt : Date.now();

    return {
      id: candidate.id,
      title,
      content: candidate.content,
      updatedAt
    };
  }

  function sanitizeTrashNote(item: unknown): TrashedNote | null {
    const note = sanitizeNote(item);
    if (!note) {
      return null;
    }

    const candidate = item as Partial<TrashedNote>;
    const deletedAt = typeof candidate.deletedAt === 'number' ? candidate.deletedAt : note.updatedAt;

    return {
      ...note,
      deletedAt
    };
  }

  function isHexColor(value: string): boolean {
    return /^#(?:[0-9a-fA-F]{3}){1,2}$/u.test(value.trim());
  }

  function sanitizeThemeTokens(candidate: unknown): ThemeTokens | null {
    if (!candidate || typeof candidate !== 'object') {
      return null;
    }

    const record = candidate as Record<string, unknown>;
    const tokens: Partial<ThemeTokens> = {};
    for (const key of THEME_TOKEN_KEYS) {
      const raw = record[key];
      if (typeof raw !== 'string' || !isHexColor(raw)) {
        return null;
      }
      tokens[key] = raw.trim();
    }

    return tokens as ThemeTokens;
  }

  function sanitizeCustomTheme(candidate: unknown): ThemePreset | null {
    if (!candidate || typeof candidate !== 'object') {
      return null;
    }

    const raw = candidate as Partial<ThemePreset>;
    if (typeof raw.id !== 'string' || !raw.id.startsWith('custom-')) {
      return null;
    }
    if (typeof raw.label !== 'string' || raw.label.trim().length < 2) {
      return null;
    }
    if (raw.mode !== 'light' && raw.mode !== 'dark') {
      return null;
    }

    const tokens = sanitizeThemeTokens(raw.tokens);
    if (!tokens) {
      return null;
    }

    return {
      id: raw.id.trim(),
      label: raw.label.trim().slice(0, 40),
      mode: raw.mode,
      builtIn: false,
      tokens
    };
  }

  function loadTheme(): void {
    const saved = window.localStorage.getItem(THEME_KEY);
    if (!saved) {
      activeThemeId = DEFAULT_THEME_ID;
      return;
    }

    if (saved === 'dark') {
      activeThemeId = DEFAULT_DARK_THEME_ID;
      return;
    }

    if (saved === 'light') {
      activeThemeId = DEFAULT_LIGHT_THEME_ID;
      return;
    }

    activeThemeId = saved;
  }

  function loadThemeLibrary(): void {
    try {
      const raw = window.localStorage.getItem(THEME_LIBRARY_KEY);
      if (!raw) {
        customThemes = [];
        return;
      }

      const parsed = JSON.parse(raw) as unknown;
      if (!Array.isArray(parsed)) {
        customThemes = [];
        return;
      }

      const clean = parsed
        .map((entry) => sanitizeCustomTheme(entry))
        .filter((themePreset): themePreset is ThemePreset => themePreset !== null);
      customThemes = clean;
    } catch {
      customThemes = [];
    }
  }

  function loadSettings(): void {
    try {
      const raw = window.localStorage.getItem(SETTINGS_KEY);
      if (!raw) {
        settings = { ...DEFAULT_SETTINGS };
        return;
      }

      const parsed = JSON.parse(raw) as Partial<AppSettings>;
      settings = {
        lineSpacing:
          parsed.lineSpacing === 'loose' || parsed.lineSpacing === 'wide' || parsed.lineSpacing === 'normal'
            ? parsed.lineSpacing
            : DEFAULT_SETTINGS.lineSpacing,
        uiScale:
          typeof parsed.uiScale === 'number' && parsed.uiScale >= 0.9 && parsed.uiScale <= 1.15
            ? Number(parsed.uiScale.toFixed(2))
            : DEFAULT_SETTINGS.uiScale,
        wordCounterMode:
          parsed.wordCounterMode === 'characters' ||
          parsed.wordCounterMode === 'both' ||
          parsed.wordCounterMode === 'words'
            ? parsed.wordCounterMode
            : DEFAULT_SETTINGS.wordCounterMode,
        noteSortMode:
          parsed.noteSortMode === 'oldest' ||
          parsed.noteSortMode === 'title-asc' ||
          parsed.noteSortMode === 'title-desc' ||
          parsed.noteSortMode === 'recent'
            ? parsed.noteSortMode
            : DEFAULT_SETTINGS.noteSortMode,
        pasteMode: parsed.pasteMode === 'basic' || parsed.pasteMode === 'plain' ? parsed.pasteMode : 'plain',
        toolbarMode:
          parsed.toolbarMode === 'selection' || parsed.toolbarMode === 'always'
            ? parsed.toolbarMode
            : DEFAULT_SETTINGS.toolbarMode,
        codeTone:
          parsed.codeTone === 'accented' || parsed.codeTone === 'plain' ? parsed.codeTone : DEFAULT_SETTINGS.codeTone,
        closeBehavior:
          parsed.closeBehavior === 'quit' || parsed.closeBehavior === 'tray'
            ? parsed.closeBehavior
            : DEFAULT_SETTINGS.closeBehavior,
        confirmPermanentDelete:
          typeof parsed.confirmPermanentDelete === 'boolean'
            ? parsed.confirmPermanentDelete
            : DEFAULT_SETTINGS.confirmPermanentDelete,
        cursorBlink: typeof parsed.cursorBlink === 'boolean' ? parsed.cursorBlink : DEFAULT_SETTINGS.cursorBlink
      };
    } catch {
      settings = { ...DEFAULT_SETTINGS };
    }
  }

  function persistTheme(nextThemeId: string): void {
    window.localStorage.setItem(THEME_KEY, nextThemeId);
  }

  function persistThemeLibrary(): void {
    window.localStorage.setItem(THEME_LIBRARY_KEY, JSON.stringify(customThemes));
  }

  function persistSettings(): void {
    window.localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings));
  }

  async function loadPinState(): Promise<void> {
    const saved = window.localStorage.getItem(PIN_KEY) === 'true';
    isPinned = saved;
    await appWindow.setAlwaysOnTop(saved);
  }

  function noteFileName(noteId: string): string {
    return `${encodeURIComponent(noteId)}.json`;
  }

  async function ensureNotesDirectories(): Promise<void> {
    await mkdir(NOTES_DIR, { baseDir: BaseDirectory.Document, recursive: true });
    await mkdir(TRASH_DIR, { baseDir: BaseDirectory.Document, recursive: true });
  }

  async function readNotesDirectory<T>(
    directory: string,
    sanitize: (item: unknown) => T | null
  ): Promise<T[]> {
    const entries = await readDir(directory, { baseDir: BaseDirectory.Document });
    const loaded: T[] = [];

    for (const entry of entries) {
      if (!entry.isFile || !entry.name.endsWith('.json')) {
        continue;
      }

      try {
        const raw = await readTextFile(`${directory}/${entry.name}`, { baseDir: BaseDirectory.Document });
        const parsed = JSON.parse(raw) as unknown;
        const sanitized = sanitize(parsed);
        if (sanitized) {
          loaded.push(sanitized);
        }
      } catch {
        // Skip corrupted files and continue loading everything else.
      }
    }

    return loaded;
  }

  async function writeNotesDirectory<T extends { id: string }>(directory: string, items: T[]): Promise<void> {
    await mkdir(directory, { baseDir: BaseDirectory.Document, recursive: true });
    const expectedFiles = new Set<string>();

    for (const item of items) {
      const fileName = noteFileName(item.id);
      expectedFiles.add(fileName);
      await writeTextFile(`${directory}/${fileName}`, JSON.stringify(item), {
        baseDir: BaseDirectory.Document
      });
    }

    const existing = await readDir(directory, { baseDir: BaseDirectory.Document });
    for (const entry of existing) {
      if (!entry.isFile || !entry.name.endsWith('.json') || expectedFiles.has(entry.name)) {
        continue;
      }

      await remove(`${directory}/${entry.name}`, { baseDir: BaseDirectory.Document });
    }
  }

  function loadLegacyLocalNotes(): NotesPayload | null {
    try {
      const raw = window.localStorage.getItem(STORAGE_KEY);
      if (raw) {
        const parsed = JSON.parse(raw) as Partial<NotesPayload>;
        const hydratedNotes = Array.isArray(parsed.notes)
          ? parsed.notes.map((item) => sanitizeNote(item)).filter((note): note is Note => note !== null)
          : [];
        const hydratedTrash = Array.isArray(parsed.trash)
          ? parsed.trash
              .map((item) => sanitizeTrashNote(item))
              .filter((note): note is TrashedNote => note !== null)
          : [];

        if (hydratedNotes.length > 0 || hydratedTrash.length > 0) {
          return {
            notes: sortNotes(hydratedNotes),
            trash: sortNotes(hydratedTrash)
          };
        }
      }
    } catch {
      // Ignore parse errors and fall through to legacy key.
    }

    try {
      const legacyRaw = window.localStorage.getItem(LEGACY_STORAGE_KEY);
      if (legacyRaw) {
        const parsedLegacy = JSON.parse(legacyRaw) as unknown;
        if (Array.isArray(parsedLegacy)) {
          const legacyNotes = parsedLegacy
            .map((item) => sanitizeNote(item))
            .filter((note): note is Note => note !== null);

          if (legacyNotes.length > 0) {
            return {
              notes: sortNotes(legacyNotes),
              trash: []
            };
          }
        }
      }
    } catch {
      // Ignore parse errors.
    }

    return null;
  }

  async function loadNotes(): Promise<void> {
    try {
      await ensureNotesDirectories();

      const fileNotes = sortNotes(await readNotesDirectory(NOTES_DIR, sanitizeNote));
      const fileTrash = sortNotes(await readNotesDirectory(TRASH_DIR, sanitizeTrashNote));

      if (fileNotes.length > 0 || fileTrash.length > 0) {
        if (fileNotes.length === 0) {
          const replacement = createWelcomeNote();
          notes = [replacement];
          trashedNotes = fileTrash;
          activeNoteId = replacement.id;
          await persistNotes(notes, trashedNotes);
          return;
        }

        notes = fileNotes;
        trashedNotes = fileTrash;
        activeNoteId = fileNotes[0]?.id ?? null;
        return;
      }

      const migrated = loadLegacyLocalNotes();
      if (migrated && migrated.notes.length > 0) {
        notes = migrated.notes;
        trashedNotes = migrated.trash;
        activeNoteId = notes[0]?.id ?? null;
        await persistNotes(notes, trashedNotes);
        return;
      }

      const welcome = createWelcomeNote();
      notes = [welcome];
      trashedNotes = [];
      activeNoteId = welcome.id;
      await persistNotes(notes, trashedNotes);
    } catch {
      const migrated = loadLegacyLocalNotes();
      if (migrated && migrated.notes.length > 0) {
        notes = migrated.notes;
        trashedNotes = migrated.trash;
        activeNoteId = notes[0]?.id ?? null;
        await persistNotes(notes, trashedNotes);
        return;
      }

      const welcome = createWelcomeNote();
      notes = [welcome];
      trashedNotes = [];
      activeNoteId = welcome.id;
      await persistNotes(notes, trashedNotes);
    }
  }

  async function persistNotes(nextNotes: Note[], nextTrash: TrashedNote[]): Promise<void> {
    try {
      await ensureNotesDirectories();
      await writeNotesDirectory(NOTES_DIR, nextNotes);
      await writeNotesDirectory(TRASH_DIR, nextTrash);
    } catch {
      // Browser/dev fallback when Tauri fs plugin is unavailable.
      window.localStorage.setItem(
        STORAGE_KEY,
        JSON.stringify({
          notes: nextNotes,
          trash: nextTrash
        })
      );
    }
  }

  function queuePersist(): void {
    if (saveTimer) {
      clearTimeout(saveTimer);
    }

    isSaving = true;
    saveTimer = setTimeout(() => {
      const notesSnapshot = [...notes];
      const trashSnapshot = [...trashedNotes];

      persistQueue = persistQueue
        .then(() => persistNotes(notesSnapshot, trashSnapshot))
        .catch(() => {
          // Keep app usable if a write fails.
        })
        .finally(() => {
          isSaving = false;
        });
    }, 250);
  }

  function createNote(): void {
    const note = createBlankNote();
    notes = [note, ...notes];
    activeNoteId = note.id;
    queuePersist();
  }

  function selectNote(noteId: string): void {
    activeNoteId = noteId;
  }

  function deleteNote(noteId: string): void {
    const removed = notes.find((note) => note.id === noteId);
    if (!removed) {
      return;
    }

    const remaining = notes.filter((note) => note.id !== noteId);

    trashedNotes = sortNotes([
      {
        ...removed,
        deletedAt: Date.now()
      },
      ...trashedNotes
    ]);

    if (remaining.length === 0) {
      const replacement = createBlankNote();
      notes = [replacement];
      activeNoteId = replacement.id;
    } else {
      notes = remaining;
      if (activeNoteId === noteId) {
        activeNoteId = remaining[0]?.id ?? null;
      }
    }

    queuePersist();
  }

  function restoreTrashedNote(noteId: string): void {
    const restored = trashedNotes.find((note) => note.id === noteId);
    if (!restored) {
      return;
    }

    trashedNotes = trashedNotes.filter((note) => note.id !== noteId);
    notes = sortNotes([{ ...restored, updatedAt: Date.now() }, ...notes]);
    activeNoteId = restored.id;
    queuePersist();
  }

  function clearTrashedNote(noteId: string): void {
    const exists = trashedNotes.some((note) => note.id === noteId);
    if (!exists) {
      return;
    }

    if (settings.confirmPermanentDelete) {
      pendingPermanentDeleteNoteId = noteId;
      return;
    }

    performClearTrashedNote(noteId);
  }

  function performClearTrashedNote(noteId: string): void {
    trashedNotes = trashedNotes.filter((note) => note.id !== noteId);
    queuePersist();
  }

  function confirmPermanentTrashDelete(): void {
    if (!pendingPermanentDeleteNoteId) {
      return;
    }
    performClearTrashedNote(pendingPermanentDeleteNoteId);
    pendingPermanentDeleteNoteId = null;
  }

  function cancelPermanentTrashDelete(): void {
    pendingPermanentDeleteNoteId = null;
  }

  function syncEditorToNote(): void {
    if (!editorEl || !activeNote) {
      return;
    }

    ensureEmptyBlocksHaveLineBreak();
    const content = editorEl.innerHTML;
    const plainText = editorEl.innerText || '';
    const updatedAt = Date.now();

    notes = sortNotes(
      notes.map((note) => {
        if (note.id !== activeNote.id) {
          return note;
        }

        return {
          ...note,
          content,
          title: deriveTitleFromText(plainText),
          updatedAt
        };
      })
    );

    activeNoteId = activeNote.id;
    queuePersist();
  }

  function ensureEmptyBlocksHaveLineBreak(): void {
    if (!editorEl) {
      return;
    }

    const blocks = editorEl.querySelectorAll('p, div, li');
    for (const block of blocks) {
      const text = (block.textContent ?? '').replace(/\u00a0/gu, ' ').trim();
      if (text.length > 0) {
        continue;
      }

      const hasMedia = block.querySelector('img, hr, figure');
      if (hasMedia) {
        continue;
      }

      if (block.innerHTML.trim() !== '<br>') {
        block.innerHTML = '<br>';
      }
    }
  }

  function pushHistorySnapshot(noteId: string, html: string): void {
    const stack = undoHistory.get(noteId) ?? [];
    if (stack.at(-1) === html) {
      return;
    }

    stack.push(html);
    if (stack.length > MAX_HISTORY_ENTRIES) {
      stack.shift();
    }

    undoHistory.set(noteId, stack);
    redoHistory.set(noteId, []);
  }

  function pushCurrentHistorySnapshot(): void {
    if (!editorEl || !activeNote || isRestoringHistory) {
      return;
    }
    pushHistorySnapshot(activeNote.id, editorEl.innerHTML);
  }

  function restoreEditorHtml(html: string): void {
    if (!editorEl || !activeNote) {
      return;
    }

    isRestoringHistory = true;
    editorEl.innerHTML = html;
    ensureEmptyBlocksHaveLineBreak();
    const selection = window.getSelection();
    const caret = document.createRange();
    caret.selectNodeContents(editorEl);
    caret.collapse(false);
    selection?.removeAllRanges();
    selection?.addRange(caret);
    lastEditorRange = caret.cloneRange();
    isRestoringHistory = false;
    syncEditorToNote();
  }

  function isNodeInsideEditor(node: Node | null): boolean {
    if (!editorEl || !node) {
      return false;
    }

    return editorEl.contains(node);
  }

  function isRangeInsideEditor(range: Range | null): boolean {
    if (!range) {
      return false;
    }

    return isNodeInsideEditor(range.startContainer) && isNodeInsideEditor(range.endContainer);
  }

  function rememberEditorSelection(): void {
    const selection = window.getSelection();
    if (!selection || selection.rangeCount === 0) {
      return;
    }

    const range = selection.getRangeAt(0);
    if (isRangeInsideEditor(range)) {
      lastEditorRange = range.cloneRange();
    }
  }

  function restoreEditorSelection(): boolean {
    if (!editorEl) {
      return false;
    }

    const selection = window.getSelection();
    if (selection && selection.rangeCount > 0 && isRangeInsideEditor(selection.getRangeAt(0))) {
      return true;
    }

    if (selection && lastEditorRange && isRangeInsideEditor(lastEditorRange)) {
      selection.removeAllRanges();
      selection.addRange(lastEditorRange.cloneRange());
      return true;
    }

    editorEl.focus();
    const fallback = document.createRange();
    fallback.selectNodeContents(editorEl);
    fallback.collapse(false);
    selection?.removeAllRanges();
    selection?.addRange(fallback);
    lastEditorRange = fallback.cloneRange();
    return true;
  }

  function getSelectedBlocks(range: Range): HTMLElement[] {
    if (!editorEl) {
      return [];
    }

    const BLOCK_TAGS = new Set(['P', 'DIV', 'H1', 'H2', 'H3', 'H4', 'BLOCKQUOTE', 'PRE']);
    const blocks: HTMLElement[] = [];
    const walker = document.createTreeWalker(editorEl, NodeFilter.SHOW_ELEMENT);

    let current = walker.nextNode();
    while (current) {
      if (current instanceof HTMLElement && BLOCK_TAGS.has(current.tagName) && range.intersectsNode(current)) {
        blocks.push(current);
      }
      current = walker.nextNode();
    }

    return blocks;
  }

  function replaceBlockTag(block: HTMLElement, tagName: 'p' | 'h1' | 'h2' | 'h3' | 'h4' | 'blockquote' | 'pre'): HTMLElement {
    const wanted = tagName.toUpperCase();
    if (block.tagName === wanted) {
      return block;
    }

    const replacement = document.createElement(tagName);
    while (block.firstChild) {
      replacement.appendChild(block.firstChild);
    }
    block.replaceWith(replacement);
    return replacement;
  }

  function applyBlockFormatToSelection(tagName: 'p' | 'h1' | 'h2' | 'h3' | 'h4' | 'blockquote' | 'pre'): void {
    if (!editorEl || !restoreEditorSelection()) {
      return;
    }
    pushCurrentHistorySnapshot();

    const selection = window.getSelection();
    if (!selection || selection.rangeCount === 0) {
      return;
    }

    const currentRange = selection.getRangeAt(0);
    if (selection.isCollapsed) {
      const anchor = getSelectionAnchorElement();
      const block = getClosestBlockElement(anchor);
      if (!block) {
        runEditorCommand('formatBlock', `<${tagName}>`);
        return;
      }

      const converted = replaceBlockTag(block, tagName);
      const caret = document.createRange();
      caret.selectNodeContents(converted);
      caret.collapse(false);
      selection.removeAllRanges();
      selection.addRange(caret);
      lastEditorRange = caret.cloneRange();
      syncEditorToNote();
      return;
    }

    const blocks = getSelectedBlocks(currentRange.cloneRange());
    if (blocks.length === 0) {
      runEditorCommand('formatBlock', `<${tagName}>`);
      return;
    }

    const converted = blocks.map((block) => replaceBlockTag(block, tagName));
    const caret = document.createRange();
    caret.selectNodeContents(converted[converted.length - 1]);
    caret.collapse(false);
    selection.removeAllRanges();
    selection.addRange(caret);
    lastEditorRange = caret.cloneRange();
    syncEditorToNote();
  }

  function runEditorCommand(command: string, value?: string, captureHistory = true): void {
    if (!editorEl || !restoreEditorSelection()) {
      return;
    }

    if (captureHistory) {
      pushCurrentHistorySnapshot();
    }

    editorEl.focus({ preventScroll: true });
    document.execCommand(command, false, value);
    rememberEditorSelection();
    syncEditorToNote();
  }

  function preserveEditorFocus(event: MouseEvent): void {
    event.preventDefault();
  }

  function getSelectionAnchorElement(): HTMLElement | null {
    const selection = window.getSelection();
    if (!selection || selection.rangeCount === 0) {
      return null;
    }

    const node = selection.anchorNode;
    if (!node) {
      return null;
    }

    return node.nodeType === Node.TEXT_NODE ? node.parentElement : (node as HTMLElement);
  }

  function getClosestBlockElement(start: HTMLElement | null): HTMLElement | null {
    if (!start || !editorEl) {
      return null;
    }

    let current: HTMLElement | null = start;
    while (current && current !== editorEl) {
      if (['P', 'DIV', 'LI', 'H1', 'H2', 'H3', 'H4', 'BLOCKQUOTE', 'PRE'].includes(current.tagName)) {
        return current;
      }
      current = current.parentElement;
    }

    return null;
  }

  function getCurrentListItem(start: HTMLElement | null): HTMLLIElement | null {
    if (!start || !editorEl) {
      return null;
    }

    let current: HTMLElement | null = start;
    while (current && current !== editorEl) {
      if (current.tagName === 'LI') {
        return current as HTMLLIElement;
      }
      current = current.parentElement;
    }

    return null;
  }

  function isCaretAtStartOfElement(element: HTMLElement): boolean {
    const selection = window.getSelection();
    if (!selection || selection.rangeCount === 0 || !selection.isCollapsed) {
      return false;
    }

    const range = selection.getRangeAt(0).cloneRange();
    if (!isNodeInsideEditor(range.startContainer)) {
      return false;
    }

    const prefix = range.cloneRange();
    prefix.setStart(element, 0);
    const beforeText = prefix.toString().replace(/\u00a0/gu, ' ').replace(/\u200b/gu, '');
    return beforeText.trim().length === 0;
  }

  function handleEditorKeydown(event: KeyboardEvent): void {
    if (!editorEl) {
      return;
    }

    const anchor = getSelectionAnchorElement();
    const listItem = getCurrentListItem(anchor);
    const quoteBlock = getClosestAncestor(anchor, 'BLOCKQUOTE');

    if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
      ensureEmptyBlocksHaveLineBreak();
    }

    if (event.key === 'Enter' && listItem) {
      const itemText = (listItem.textContent ?? '').trim();
      if (!itemText) {
        event.preventDefault();
        pushCurrentHistorySnapshot();
        document.execCommand('outdent');
        document.execCommand('formatBlock', false, '<p>');
        syncEditorToNote();
        return;
      }
    }

    if (event.key === 'Backspace' && listItem && isCaretAtStartOfElement(listItem)) {
      event.preventDefault();
      pushCurrentHistorySnapshot();
      document.execCommand('outdent');
      if (!(listItem.textContent ?? '').trim()) {
        document.execCommand('formatBlock', false, '<p>');
      }
      syncEditorToNote();
      return;
    }

    if (event.key === 'Enter' && quoteBlock) {
      event.preventDefault();
      const selection = window.getSelection();
      if (!selection || selection.rangeCount === 0) {
        return;
      }

      if (isCaretOnEmptyBlockquoteLine(selection, quoteBlock)) {
        pushCurrentHistorySnapshot();
        exitBlockquote(quoteBlock);
      } else {
        pushCurrentHistorySnapshot();
        document.execCommand('insertLineBreak');
        syncEditorToNote();
      }
      return;
    }

    if (event.key === ' ' && !listItem && maybeAutoConvertToBullet(anchor)) {
      event.preventDefault();
      syncEditorToNote();
      return;
    }

    if (event.key === '-' && !listItem && maybeAutoConvertToDivider()) {
      event.preventDefault();
      syncEditorToNote();
      return;
    }
  }

  function handleEditorPaste(event: ClipboardEvent): void {
    if (!editorEl || !event.clipboardData) {
      return;
    }

    const clipboardItems = [...event.clipboardData.items];
    const imageItem = clipboardItems.find((item) => item.type.startsWith('image/'));
    if (imageItem) {
      const file = imageItem.getAsFile();
      if (file) {
        event.preventDefault();
        insertPastedImage(file);
      }
      return;
    }

    const plainText = event.clipboardData.getData('text/plain');
    const trimmed = plainText.trim();
    if (!trimmed) {
      return;
    }

    const hasSingleUrl = /^\S+$/u.test(trimmed) && /^(https?:\/\/|www\.)/iu.test(trimmed);

    if (!hasSingleUrl && settings.pasteMode === 'basic') {
      return;
    }

    event.preventDefault();
    editorEl.focus();

    if (hasSingleUrl) {
      const normalized = normalizeUrl(trimmed);
      const selection = window.getSelection();
      const selectedText = selection?.toString() ?? '';
      if (selectedText.trim()) {
        document.execCommand('createLink', false, normalized);
      } else {
        document.execCommand(
          'insertHTML',
          false,
          `<a href="${escapeHtmlAttribute(normalized)}">${escapeHtml(normalized)}</a>`
        );
      }
      syncEditorToNote();
      return;
    }

    const normalizedHtml = escapeHtml(plainText).replace(/\r?\n/gu, '<br>');
    document.execCommand('insertHTML', false, normalizedHtml);
    syncEditorToNote();
  }

  function applyParagraph(): void {
    applyBlockFormatToSelection('p');
  }

  function applyHeading(level: 1 | 2 | 3 | 4): void {
    if (level === 1) {
      applyBlockFormatToSelection('h1');
      return;
    }
    if (level === 2) {
      applyBlockFormatToSelection('h2');
      return;
    }
    if (level === 3) {
      applyBlockFormatToSelection('h3');
      return;
    }
    applyBlockFormatToSelection('h4');
  }

  function applyBlockquote(): void {
    const anchor = getSelectionAnchorElement();
    const block = getClosestBlockElement(anchor);
    if (block?.tagName === 'BLOCKQUOTE') {
      applyParagraph();
      return;
    }

    applyBlockFormatToSelection('blockquote');
  }

  function applyBullets(): void {
    runEditorCommand('insertUnorderedList');
  }

  function applyOrderedList(): void {
    runEditorCommand('insertOrderedList');
  }

  function applyBold(): void {
    runEditorCommand('bold');
  }

  function applyItalic(): void {
    runEditorCommand('italic');
  }

  function applyStrike(): void {
    runEditorCommand('strikeThrough');
  }

  function applyCode(): void {
    applyBlockFormatToSelection('pre');
  }

  function applyUnderline(): void {
    runEditorCommand('underline');
  }

  function applyRemoveFormatting(): void {
    runEditorCommand('removeFormat');
  }

  function applyDivider(): void {
    runEditorCommand('insertHorizontalRule');
  }

  function applyAlignLeft(): void {
    runEditorCommand('justifyLeft');
  }

  function applyAlignCenter(): void {
    runEditorCommand('justifyCenter');
  }

  function applyAlignRight(): void {
    runEditorCommand('justifyRight');
  }

  function handleAlignmentChange(event: Event): void {
    const value = (event.currentTarget as HTMLSelectElement).value as AlignMode;
    if (value === 'left') {
      applyAlignLeft();
    } else if (value === 'center') {
      applyAlignCenter();
    } else {
      applyAlignRight();
    }
    isFormatMenuOpen = false;
    isSelectionFormatMenuOpen = false;
  }

  function toggleFormatMenu(): void {
    isFormatMenuOpen = !isFormatMenuOpen;
  }

  function runFromFormatMenu(action: () => void): void {
    action();
    isFormatMenuOpen = false;
  }

  function handleFormatBlockStyleChange(event: Event): void {
    handleBlockStyleChange(event);
    isFormatMenuOpen = false;
  }

  function toggleSelectionFormatMenu(): void {
    isSelectionFormatMenuOpen = !isSelectionFormatMenuOpen;
  }

  function runFromSelectionFormatMenu(action: () => void): void {
    action();
    isSelectionFormatMenuOpen = false;
  }

  function handleSelectionFormatBlockStyleChange(event: Event): void {
    handleBlockStyleChange(event);
    isSelectionFormatMenuOpen = false;
  }

  function handleSelectionAlignmentChange(event: Event): void {
    handleAlignmentChange(event);
    isSelectionFormatMenuOpen = false;
  }

  function promptLink(): void {
    if (!editorEl) {
      return;
    }

    const entered = window.prompt('Enter URL');
    if (!entered) {
      return;
    }

    const normalized = normalizeUrl(entered.trim());
    runEditorCommand('createLink', normalized);
  }

  function applyUndo(): void {
    if (!editorEl || !activeNote || !restoreEditorSelection()) {
      return;
    }

    editorEl.focus({ preventScroll: true });
    const before = editorEl.innerHTML;
    document.execCommand('undo', false);

    if (editorEl.innerHTML !== before) {
      rememberEditorSelection();
      syncEditorToNote();
      return;
    }

    const noteId = activeNote.id;
    const stack = undoHistory.get(noteId) ?? [];
    if (stack.length === 0) {
      return;
    }

    const current = editorEl.innerHTML;
    if (stack.at(-1) === current) {
      stack.pop();
    }

    const previous = stack.pop();
    if (!previous) {
      undoHistory.set(noteId, stack);
      return;
    }

    undoHistory.set(noteId, stack);
    const redoStack = redoHistory.get(noteId) ?? [];
    redoStack.push(current);
    if (redoStack.length > MAX_HISTORY_ENTRIES) {
      redoStack.shift();
    }
    redoHistory.set(noteId, redoStack);
    restoreEditorHtml(previous);
  }

  function applyRedo(): void {
    if (!editorEl || !activeNote || !restoreEditorSelection()) {
      return;
    }

    editorEl.focus({ preventScroll: true });
    const before = editorEl.innerHTML;
    document.execCommand('redo', false);

    if (editorEl.innerHTML !== before) {
      rememberEditorSelection();
      syncEditorToNote();
      return;
    }

    const noteId = activeNote.id;
    const redoStack = redoHistory.get(noteId) ?? [];
    const next = redoStack.pop();
    if (!next) {
      return;
    }

    redoHistory.set(noteId, redoStack);
    pushHistorySnapshot(noteId, before);
    restoreEditorHtml(next);
  }

  function handleBlockStyleChange(event: Event): void {
    const value = (event.currentTarget as HTMLSelectElement).value;

    if (value === 'p') {
      applyParagraph();
      return;
    }

    if (value === 'h1') {
      applyHeading(1);
      return;
    }

    if (value === 'h2') {
      applyHeading(2);
      return;
    }

    if (value === 'h3') {
      applyHeading(3);
      return;
    }

    if (value === 'h4') {
      applyHeading(4);
      return;
    }

    if (value === 'blockquote') {
      applyBlockquote();
    }
  }

  function loadThemeModeMemory(): { light: string; dark: string } {
    const fallback = { light: DEFAULT_LIGHT_THEME_ID, dark: DEFAULT_DARK_THEME_ID };
    try {
      const raw = window.localStorage.getItem(THEME_MODE_MEMORY_KEY);
      if (!raw) {
        return fallback;
      }
      const parsed = JSON.parse(raw) as { light?: string; dark?: string };
      return {
        light: typeof parsed.light === 'string' ? parsed.light : fallback.light,
        dark: typeof parsed.dark === 'string' ? parsed.dark : fallback.dark
      };
    } catch {
      return fallback;
    }
  }

  function persistThemeModeMemory(next: { light: string; dark: string }): void {
    window.localStorage.setItem(THEME_MODE_MEMORY_KEY, JSON.stringify(next));
  }

  function rememberThemePreference(themeId: string): void {
    const selected = allThemes.find((themePreset) => themePreset.id === themeId);
    if (!selected) {
      return;
    }

    const memory = loadThemeModeMemory();
    if (selected.mode === 'dark') {
      memory.dark = selected.id;
    } else {
      memory.light = selected.id;
    }
    persistThemeModeMemory(memory);
  }

  function applyThemePreset(nextThemeId: string): void {
    const exists = allThemes.some((themePreset) => themePreset.id === nextThemeId);
    if (!exists) {
      return;
    }

    activeThemeId = nextThemeId;
    persistTheme(nextThemeId);
    rememberThemePreference(nextThemeId);
  }

  function toggleTheme(): void {
    const memory = loadThemeModeMemory();
    if (currentThemeMode === 'dark') {
      const target = allThemes.some((themePreset) => themePreset.id === memory.light && themePreset.mode === 'light')
        ? memory.light
        : DEFAULT_LIGHT_THEME_ID;
      applyThemePreset(target);
      return;
    }

    const target = allThemes.some((themePreset) => themePreset.id === memory.dark && themePreset.mode === 'dark')
      ? memory.dark
      : DEFAULT_DARK_THEME_ID;
    applyThemePreset(target);
  }

  function updateThemePreset(nextThemeId: string): void {
    applyThemePreset(nextThemeId);
  }

  function buildImportedTheme(raw: unknown): ThemePreset | null {
    if (!raw || typeof raw !== 'object') {
      return null;
    }

    const source = raw as {
      id?: unknown;
      label?: unknown;
      mode?: unknown;
      tokens?: unknown;
      palette?: unknown;
    };

    const mode: ThemeMode = source.mode === 'light' ? 'light' : source.mode === 'dark' ? 'dark' : 'dark';
    const tokens = sanitizeThemeTokens(source.tokens ?? source.palette);
    if (!tokens) {
      return null;
    }

    const baseLabel = typeof source.label === 'string' && source.label.trim().length > 1 ? source.label.trim() : 'Custom Theme';
    const idBase =
      typeof source.id === 'string' && source.id.trim().length > 1 && !BUILTIN_THEME_IDS.has(source.id.trim())
        ? source.id.trim()
        : `custom-${Date.now()}`;
    const uniqueId = allThemes.some((themePreset) => themePreset.id === idBase) ? `${idBase}-${Date.now()}` : idBase;

    return {
      id: uniqueId.startsWith('custom-') ? uniqueId : `custom-${uniqueId}`,
      label: baseLabel.slice(0, 40),
      mode,
      builtIn: false,
      tokens
    };
  }

  async function importThemeFromFile(event: Event): Promise<void> {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) {
      return;
    }

    try {
      const rawText = await file.text();
      const parsed = JSON.parse(rawText) as unknown;
      const imported = buildImportedTheme(parsed);
      if (!imported) {
        window.alert('Theme file invalid. Use a JSON file with mode + full token palette.');
        return;
      }

      customThemes = [...customThemes.filter((themePreset) => themePreset.id !== imported.id), imported];
      persistThemeLibrary();
      applyThemePreset(imported.id);
    } catch {
      window.alert('Unable to import theme JSON.');
    } finally {
      input.value = '';
    }
  }

  function triggerThemeImport(): void {
    themeFileInput?.click();
  }

  function exportCurrentTheme(): void {
    const payload = {
      label: activeTheme.label,
      mode: activeTheme.mode,
      tokens: activeTheme.tokens
    };
    const blob = new Blob([JSON.stringify(payload, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement('a');
    anchor.href = url;
    anchor.download = `${activeTheme.label.toLowerCase().replace(/[^a-z0-9]+/gu, '-') || 'slate-theme'}.json`;
    document.body.append(anchor);
    anchor.click();
    anchor.remove();
    URL.revokeObjectURL(url);
  }

  function removeSelectedCustomTheme(): void {
    if (BUILTIN_THEME_IDS.has(activeThemeId)) {
      return;
    }

    customThemes = customThemes.filter((themePreset) => themePreset.id !== activeThemeId);
    persistThemeLibrary();
    applyThemePreset(DEFAULT_THEME_ID);
  }

  async function togglePin(): Promise<void> {
    const next = !isPinned;
    isPinned = next;
    await appWindow.setAlwaysOnTop(next);
    window.localStorage.setItem(PIN_KEY, String(next));
  }

  function toggleSidebar(): void {
    isSidebarOpen = !isSidebarOpen;
  }

  function updateLineSpacing(next: LineSpacing): void {
    settings.lineSpacing = next;
    persistSettings();
  }

  function updateUiScale(next: number): void {
    settings.uiScale = Number(next.toFixed(2));
    persistSettings();
  }

  function updateWordCounterMode(next: WordCounterMode): void {
    settings.wordCounterMode = next;
    persistSettings();
  }

  function updateNoteSortMode(next: NoteSortMode): void {
    settings.noteSortMode = next;
    persistSettings();
  }

  function updatePasteMode(next: PasteMode): void {
    settings.pasteMode = next;
    persistSettings();
  }

  function updateToolbarMode(next: ToolbarMode): void {
    settings.toolbarMode = next;
    isFormatMenuOpen = false;
    isSelectionFormatMenuOpen = false;
    persistSettings();
    updateSelectionMenu();
  }

  function updateCodeTone(next: CodeTone): void {
    settings.codeTone = next;
    persistSettings();
  }

  function updateCloseBehavior(next: CloseBehavior): void {
    settings.closeBehavior = next;
    persistSettings();
  }

  function updateConfirmPermanentDelete(next: boolean): void {
    settings.confirmPermanentDelete = next;
    persistSettings();
  }

  function updateCursorBlink(next: boolean): void {
    settings.cursorBlink = next;
    persistSettings();
  }

  function getClosestAncestor(start: HTMLElement | null, tagName: string): HTMLElement | null {
    if (!start || !editorEl) {
      return null;
    }

    let current: HTMLElement | null = start;
    while (current && current !== editorEl) {
      if (current.tagName === tagName) {
        return current;
      }
      current = current.parentElement;
    }

    return null;
  }

  function isCaretOnEmptyBlockquoteLine(selection: Selection, blockquote: HTMLElement): boolean {
    if (selection.rangeCount === 0) {
      return false;
    }

    const range = selection.getRangeAt(0);
    const before = range.cloneRange();
    before.setStart(blockquote, 0);

    const after = range.cloneRange();
    after.setEnd(blockquote, blockquote.childNodes.length);

    const beforeLine = (before.toString().replace(/\u00a0/gu, ' ').split(/\r?\n/gu).at(-1) ?? '').trim();
    const afterLine = (after.toString().replace(/\u00a0/gu, ' ').split(/\r?\n/gu)[0] ?? '').trim();

    return `${beforeLine}${afterLine}`.length === 0;
  }

  function exitBlockquote(blockquote: HTMLElement): void {
    const paragraph = document.createElement('p');
    paragraph.innerHTML = '<br>';

    if (blockquote.parentNode) {
      blockquote.parentNode.insertBefore(paragraph, blockquote.nextSibling);
    }

    const range = document.createRange();
    range.setStart(paragraph, 0);
    range.collapse(true);
    const selection = window.getSelection();
    selection?.removeAllRanges();
    selection?.addRange(range);
    syncEditorToNote();
  }

  function updateSelectionMenu(): void {
    if (!editorEl || settings.toolbarMode !== 'selection') {
      showSelectionMenu = false;
      isSelectionFormatMenuOpen = false;
      return;
    }

    const selection = window.getSelection();
    if (!selection || selection.rangeCount === 0 || selection.isCollapsed) {
      showSelectionMenu = false;
      isSelectionFormatMenuOpen = false;
      return;
    }

    const range = selection.getRangeAt(0);
    if (!editorEl.contains(range.commonAncestorContainer)) {
      showSelectionMenu = false;
      isSelectionFormatMenuOpen = false;
      return;
    }

    let anchorRect = range.getBoundingClientRect();
    const focusNode = selection.focusNode;
    if (focusNode) {
      const focusOffset = selection.focusOffset;
      const focusRange = document.createRange();
      const maxOffset = focusNode.nodeType === Node.TEXT_NODE ? (focusNode.textContent?.length ?? 0) : focusNode.childNodes.length;
      focusRange.setStart(focusNode, Math.min(focusOffset, maxOffset));
      focusRange.collapse(true);
      anchorRect = focusRange.getBoundingClientRect();
    }
    if (!anchorRect.width && !anchorRect.height) {
      const rects = range.getClientRects();
      anchorRect = rects.length > 0 ? rects[rects.length - 1] : range.getBoundingClientRect();
    }

    const rect = anchorRect;
    if (!rect.width && !rect.height) {
      showSelectionMenu = false;
      isSelectionFormatMenuOpen = false;
      return;
    }

    const menuWidth = selectionMenuEl?.offsetWidth ?? 304;
    const halfMenuWidth = menuWidth / 2;
    const safeEdge = 14;
    const targetX = rect.left + rect.width / 2;
    selectionMenuX = Math.min(Math.max(targetX, halfMenuWidth + safeEdge), window.innerWidth - halfMenuWidth - safeEdge);
    selectionMenuY = Math.max(rect.top - 10, 58);
    showSelectionMenu = true;
  }

  function handleEditorClick(event: MouseEvent): void {
    const target = event.target as HTMLElement | null;
    if (!target) {
      return;
    }

    const image = target.closest('img') as HTMLImageElement | null;
    if (!image || !editorEl?.contains(image)) {
      return;
    }

    const figure = image.closest('figure');
    const caption = figure?.querySelector('figcaption')?.textContent?.trim() ?? '';
    lightboxImageSrc = image.src;
    lightboxImageCaption = caption;
  }

  function closeImageLightbox(): void {
    lightboxImageSrc = null;
    lightboxImageCaption = '';
  }

  function insertPastedImage(file: File): void {
    const editor = editorEl;
    if (!editor) {
      return;
    }

    const reader = new FileReader();
    reader.onload = () => {
      const result = typeof reader.result === 'string' ? reader.result : '';
      if (!result) {
        return;
      }

      editor.focus();
      document.execCommand(
        'insertHTML',
        false,
        `<figure class="note-image"><img src="${escapeHtmlAttribute(result)}" alt="Pasted image" /><figcaption data-placeholder="Add caption"></figcaption></figure><p><br></p>`
      );
      syncEditorToNote();
    };

    reader.readAsDataURL(file);
  }

  function isEventInsideEditor(target: EventTarget | null): boolean {
    if (!editorEl || !(target instanceof Node)) {
      return false;
    }
    return editorEl.contains(target);
  }

  function maybeAutoConvertToBullet(anchor: HTMLElement | null): boolean {
    if (!anchor) {
      return false;
    }

    const selection = window.getSelection();
    if (!selection || !selection.isCollapsed || selection.rangeCount === 0) {
      return false;
    }

    const block = getClosestBlockElement(anchor);
    if (!block || block.tagName === 'LI') {
      return false;
    }

    const markerText = (block.textContent ?? '').replace(/\u00a0/gu, ' ').trim();
    if (!/^[-*]$/u.test(markerText)) {
      return false;
    }

    pushCurrentHistorySnapshot();
    block.textContent = '';
    const ul = document.createElement('ul');
    const li = document.createElement('li');
    li.innerHTML = '<br>';
    ul.appendChild(li);
    block.replaceWith(ul);

    const caret = document.createRange();
    caret.selectNodeContents(li);
    caret.collapse(true);
    selection.removeAllRanges();
    selection.addRange(caret);
    lastEditorRange = caret.cloneRange();
    return true;
  }

  function maybeAutoConvertToDivider(): boolean {
    const selection = window.getSelection();
    if (!selection || !selection.isCollapsed) {
      return false;
    }

    const activeRange = selection.getRangeAt(0);
    const activeNode = selection.anchorNode;
    if (!activeNode || activeNode.nodeType !== Node.TEXT_NODE) {
      return false;
    }

    const textNode = activeNode as Text;
    const fullText = (textNode.textContent ?? '').replace(/\u00a0/gu, ' ');
    const caretOffset = selection.anchorOffset;
    const lineStart = fullText.lastIndexOf('\n', Math.max(0, caretOffset - 1)) + 1;
    const linePrefix = fullText.slice(lineStart, caretOffset);

    if (!/^\s*--$/u.test(linePrefix)) {
      return false;
    }

    const markerRange = activeRange.cloneRange();
    markerRange.setStart(textNode, lineStart);
    markerRange.setEnd(textNode, caretOffset);
    pushCurrentHistorySnapshot();
    markerRange.deleteContents();

    document.execCommand('insertHTML', false, '<hr><p><br></p>');
    return true;
  }

  function normalizeUrl(raw: string): string {
    if (!raw) {
      return '';
    }

    if (/^https?:\/\//iu.test(raw)) {
      return raw;
    }

    if (/^www\./iu.test(raw)) {
      return `https://${raw}`;
    }

    return `https://${raw}`;
  }

  function escapeHtml(value: string): string {
    return value.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
  }

  function escapeHtmlAttribute(value: string): string {
    return escapeHtml(value).replaceAll('"', '&quot;');
  }

  async function closeApp(): Promise<void> {
    if (settings.closeBehavior === 'quit') {
      await appWindow.destroy();
      return;
    }

    await appWindow.close();
  }

  async function minimizeApp(): Promise<void> {
    await appWindow.minimize();
  }

  async function maximizeApp(): Promise<void> {
    await appWindow.toggleMaximize();
  }
</script>

<main
  class="app-shell"
  class:dark={isDarkTheme}
  class:sidebar-closed={!isSidebarOpen}
  data-line-spacing={settings.lineSpacing}
  data-cursor-blink={settings.cursorBlink ? 'on' : 'off'}
  data-code-tone={settings.codeTone}
  style={`--ui-scale: ${settings.uiScale};${activeThemeCssVars}`}
>
  <header class="window-chrome" data-tauri-drag-region>
    <div class="chrome-left" data-tauri-drag-region>
      <div class="traffic">
        <button class="traffic-btn close" type="button" title="Close" aria-label="Close" onclick={closeApp}></button>
        <button class="traffic-btn mini" type="button" title="Minimize" aria-label="Minimize" onclick={minimizeApp}></button>
        <button class="traffic-btn max" type="button" title="Maximize" aria-label="Maximize" onclick={maximizeApp}></button>
      </div>
    </div>

    <div class="chrome-drag" data-tauri-drag-region>
      Slate
    </div>

    <div class="chrome-right">
      <button
        class="icon-btn theme-toggle-top"
        type="button"
        title={currentThemeMode === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'}
        aria-label={currentThemeMode === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'}
        onclick={toggleTheme}
      >
        {currentThemeMode === 'dark' ? '◑' : '◐'}
      </button>
      <button class="icon-btn" type="button" title="Toggle list" aria-label="Toggle list" onclick={toggleSidebar}>
        <span class="list-icon" aria-hidden="true"></span>
      </button>
      <button class="icon-btn" type="button" title="New note" aria-label="New note" onclick={createNote}>+</button>
      <button class="icon-btn" type="button" title="Settings" aria-label="Settings" onclick={() => (isSettingsOpen = !isSettingsOpen)}>
        ⚙
      </button>
      <button class="icon-btn" type="button" title={isPinned ? 'Unpin window' : 'Pin window'} aria-label={isPinned ? 'Unpin window' : 'Pin window'} onclick={togglePin}>
        <span class="pin-icon" class:pinned={isPinned} aria-hidden="true"></span>
      </button>
    </div>
  </header>

  <section class="notes-panel">
    <label class="search-wrap" for="note-search">
      <input
        id="note-search"
        type="search"
        placeholder="Search"
        value={searchQuery}
        oninput={(event) => {
          searchQuery = (event.currentTarget as HTMLInputElement).value;
        }}
      />
    </label>

    <div class="sort-wrap">
      <select
        class="sort-select"
        aria-label="Sort notes"
        value={settings.noteSortMode}
        onchange={(event) => updateNoteSortMode((event.currentTarget as HTMLSelectElement).value as NoteSortMode)}
      >
        <option value="recent">Newest first</option>
        <option value="oldest">Oldest first</option>
        <option value="title-asc">Title A-Z</option>
        <option value="title-desc">Title Z-A</option>
      </select>
    </div>

    <ul class="note-list">
      {#if visibleNotes.length === 0}
        <li class="empty-state">No notes found.</li>
      {:else}
        {#each visibleNotes as note (note.id)}
          <li>
            <div class="note-card" class:active={note.id === activeNoteId}>
              <button type="button" class="note-item" onclick={() => selectNote(note.id)}>
                <span class="title">{note.title}</span>
                <span class="meta">{formatUpdatedAt(note.updatedAt)}</span>
              </button>
              <button
                type="button"
                class="note-delete"
                title="Delete note"
                aria-label="Delete note"
                onclick={() => deleteNote(note.id)}
              >
                ×
              </button>
            </div>
          </li>
        {/each}
      {/if}
    </ul>

    <section class="trash-wrap">
      <button class="trash-toggle" type="button" onclick={() => (isTrashOpen = !isTrashOpen)}>
        <span>Trash</span>
        <span>{trashedNotes.length}</span>
      </button>

      {#if isTrashOpen}
        <ul class="trash-list">
          {#if trashedNotes.length === 0}
            <li class="empty-state">Nothing in trash.</li>
          {:else}
            {#each trashedNotes as note (note.id)}
              <li class="trash-item">
                <div class="trash-copy">
                  <span class="title">{note.title}</span>
                  <span class="meta">{formatUpdatedAt(note.deletedAt)}</span>
                </div>
                <div class="trash-actions">
                  <button
                    type="button"
                    class="mini-btn"
                    onclick={(event) => {
                      event?.stopPropagation?.();
                      restoreTrashedNote(note.id);
                    }}
                  >
                    Restore
                  </button>
                  <button
                    type="button"
                    class="mini-btn danger"
                    onclick={(event) => {
                      event?.stopPropagation?.();
                      clearTrashedNote(note.id);
                    }}
                  >
                    ×
                  </button>
                </div>
              </li>
            {/each}
          {/if}
        </ul>
      {/if}
    </section>
  </section>

  <section class="editor-panel">
    {#if activeNote}
      {#if settings.toolbarMode === 'always'}
        <div
          class="toolbar"
          role="toolbar"
          tabindex="-1"
          aria-label="Text formatting toolbar"
        >
          <div class="tool-group">
            <button type="button" class="tool-btn" title="Bold" onmousedown={preserveEditorFocus} onclick={applyBold}>B</button>
            <button type="button" class="tool-btn" title="Italic" onmousedown={preserveEditorFocus} onclick={applyItalic}>I</button>
            <button type="button" class="tool-btn" title="Strike" onmousedown={preserveEditorFocus} onclick={applyStrike}>S</button>
            <button type="button" class="tool-btn" title="Underline" onmousedown={preserveEditorFocus} onclick={applyUnderline}>U</button>
            <button type="button" class="tool-btn" title="Remove formatting" onmousedown={preserveEditorFocus} onclick={applyRemoveFormatting}>Tx</button>
          </div>
          <div class="tool-menu-wrap" bind:this={formatMenuEl}>
            <button
              type="button"
              class="tool-btn menu-toggle"
              title="More formatting"
              onclick={toggleFormatMenu}
            >
              ⋯
            </button>
            {#if isFormatMenuOpen}
              <div class="tool-menu" role="menu" aria-label="More formatting options">
                <label class="menu-label" for="block-style-select">Style</label>
                <select
                  id="block-style-select"
                  class="tool-select menu-select"
                  title="Block style"
                  onchange={handleFormatBlockStyleChange}
                >
                  <option value="p">P</option>
                  <option value="h1">H1</option>
                  <option value="h2">H2</option>
                  <option value="h3">H3</option>
                  <option value="h4">H4</option>
                  <option value="blockquote">Quote</option>
                </select>

                <label class="menu-label" for="align-select">Align</label>
                <select
                  id="align-select"
                  class="tool-select menu-select"
                  title="Alignment"
                  onchange={handleAlignmentChange}
                >
                  <option value="left">Left</option>
                  <option value="center">Center</option>
                  <option value="right">Right</option>
                </select>

                <div class="menu-actions">
                  <button type="button" class="mini-btn" onmousedown={preserveEditorFocus} onclick={() => runFromFormatMenu(applyBullets)}>Bullets</button>
                  <button type="button" class="mini-btn" onmousedown={preserveEditorFocus} onclick={() => runFromFormatMenu(applyOrderedList)}>Numbered</button>
                  <button type="button" class="mini-btn" onmousedown={preserveEditorFocus} onclick={() => runFromFormatMenu(applyCode)}>Code</button>
                  <button type="button" class="mini-btn" onmousedown={preserveEditorFocus} onclick={() => runFromFormatMenu(applyDivider)}>Divider</button>
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      {#if settings.toolbarMode === 'selection' && showSelectionMenu}
        <div bind:this={selectionMenuEl} class="selection-menu" style={`left:${selectionMenuX}px;top:${selectionMenuY}px;`}>
          <button type="button" class="tool-btn" title="Bold" onmousedown={preserveEditorFocus} onclick={applyBold}>B</button>
          <button type="button" class="tool-btn" title="Italic" onmousedown={preserveEditorFocus} onclick={applyItalic}>I</button>
          <button type="button" class="tool-btn" title="Strike" onmousedown={preserveEditorFocus} onclick={applyStrike}>S</button>
          <button type="button" class="tool-btn" title="Underline" onmousedown={preserveEditorFocus} onclick={applyUnderline}>U</button>
          <button type="button" class="tool-btn" title="Remove formatting" onmousedown={preserveEditorFocus} onclick={applyRemoveFormatting}>Tx</button>
          <div class="tool-menu-wrap" bind:this={selectionFormatMenuEl}>
            <button
              type="button"
              class="tool-btn menu-toggle"
              title="More formatting"
              onclick={toggleSelectionFormatMenu}
            >
              ⋯
            </button>
            {#if isSelectionFormatMenuOpen}
              <div class="tool-menu selection-tool-menu" role="menu" aria-label="Selection formatting options">
                <label class="menu-label" for="selection-block-style-select">Style</label>
                <select
                  id="selection-block-style-select"
                  class="tool-select menu-select"
                  title="Block style"
                  onchange={handleSelectionFormatBlockStyleChange}
                >
                  <option value="p">P</option>
                  <option value="h1">H1</option>
                  <option value="h2">H2</option>
                  <option value="h3">H3</option>
                  <option value="h4">H4</option>
                  <option value="blockquote">Quote</option>
                </select>

                <label class="menu-label" for="selection-align-select">Align</label>
                <select
                  id="selection-align-select"
                  class="tool-select menu-select"
                  title="Alignment"
                  onchange={handleSelectionAlignmentChange}
                >
                  <option value="left">Left</option>
                  <option value="center">Center</option>
                  <option value="right">Right</option>
                </select>

                <div class="menu-actions">
                  <button type="button" class="mini-btn" onmousedown={preserveEditorFocus} onclick={() => runFromSelectionFormatMenu(applyBullets)}>Bullets</button>
                  <button type="button" class="mini-btn" onmousedown={preserveEditorFocus} onclick={() => runFromSelectionFormatMenu(applyOrderedList)}>Numbered</button>
                  <button type="button" class="mini-btn" onmousedown={preserveEditorFocus} onclick={() => runFromSelectionFormatMenu(applyCode)}>Code</button>
                  <button type="button" class="mini-btn" onmousedown={preserveEditorFocus} onclick={() => runFromSelectionFormatMenu(applyDivider)}>Divider</button>
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <div
        bind:this={editorEl}
        class="editor"
        contenteditable="true"
        role="textbox"
        tabindex="0"
        aria-label="Note editor"
        onkeydown={handleEditorKeydown}
        onpaste={handleEditorPaste}
        onclick={handleEditorClick}
        onmouseup={updateSelectionMenu}
        onkeyup={updateSelectionMenu}
        onscroll={updateSelectionMenu}
        oninput={syncEditorToNote}
      ></div>
    {/if}
  </section>

  <footer class="save-status">
    <span class="save-dot" class:saving={isSaving}></span>
    <span>{isSaving ? 'Saving...' : 'Saved'}</span>
    <span class="status-sep">·</span>
    <span>{countLabel}</span>
  </footer>

  {#if isSettingsOpen}
    <div
      class="settings-scrim"
      role="button"
      tabindex="0"
      aria-label="Close settings"
      onclick={() => (isSettingsOpen = false)}
      onkeydown={(event) => {
        if (event.key === 'Enter' || event.key === ' ') {
          event.preventDefault();
          isSettingsOpen = false;
        }
        if (event.key === 'Escape') {
          event.preventDefault();
          isSettingsOpen = false;
        }
      }}
    >
      <div
        class="settings-panel"
        role="dialog"
        tabindex="-1"
        aria-label="Settings"
        onclick={(event) => event.stopPropagation()}
        onkeydown={(event) => {
          if (event.key === 'Escape') {
            event.preventDefault();
            isSettingsOpen = false;
          }
        }}
      >
        <header class="settings-head">
          <h2>Settings</h2>
          <button class="icon-btn settings-close" type="button" title="Close settings" aria-label="Close settings" onclick={() => (isSettingsOpen = false)}>
            ×
          </button>
        </header>

        <div class="settings-list">
          <section class="settings-item">
            <div class="settings-copy">
              <span class="settings-label">Theme preset</span>
              <span class="settings-sub">Built-ins plus imported JSON themes</span>
            </div>
            <select
              class="settings-select"
              value={activeThemeId}
              onchange={(event) => updateThemePreset((event.currentTarget as HTMLSelectElement).value)}
            >
              {#each BUILTIN_THEMES as themePreset}
                <option value={themePreset.id}>{themePreset.label}</option>
              {/each}
              {#if customThemes.length > 0}
                <optgroup label="Custom themes">
                  {#each customThemes as themePreset}
                    <option value={themePreset.id}>{themePreset.label}</option>
                  {/each}
                </optgroup>
              {/if}
            </select>
          </section>

          <section class="settings-item settings-item-stack">
            <div class="settings-copy">
              <span class="settings-label">Theme JSON</span>
              <span class="settings-sub">Import a `.json` palette or export the current one</span>
            </div>
            <div class="settings-actions">
              <button type="button" class="mini-btn" onclick={triggerThemeImport}>Import</button>
              <button type="button" class="mini-btn" onclick={exportCurrentTheme}>Export</button>
              <button
                type="button"
                class="mini-btn danger"
                disabled={BUILTIN_THEME_IDS.has(activeThemeId)}
                onclick={removeSelectedCustomTheme}
              >
                Remove custom
              </button>
            </div>
            <input
              bind:this={themeFileInput}
              class="theme-file-input"
              type="file"
              accept="application/json,.json"
              onchange={importThemeFromFile}
            />
          </section>

          <section class="settings-item">
            <div class="settings-copy">
              <span class="settings-label">Line spacing</span>
            </div>
            <select
              class="settings-select"
              value={settings.lineSpacing}
              onchange={(event) => updateLineSpacing((event.currentTarget as HTMLSelectElement).value as LineSpacing)}
            >
              <option value="normal">Normal</option>
              <option value="loose">Loose</option>
              <option value="wide">Wide</option>
            </select>
          </section>

          <section class="settings-item">
            <div class="settings-copy">
              <span class="settings-label">UI scale</span>
              <span class="settings-sub">{Math.round(settings.uiScale * 100)}%</span>
            </div>
            <input
              class="settings-range"
              type="range"
              min="0.9"
              max="1.15"
              step="0.05"
              value={settings.uiScale}
              oninput={(event) => updateUiScale(Number((event.currentTarget as HTMLInputElement).value))}
            />
          </section>

        <section class="settings-item">
          <div class="settings-copy">
            <span class="settings-label">Word counter</span>
          </div>
          <select
            class="settings-select"
            value={settings.wordCounterMode}
            onchange={(event) =>
              updateWordCounterMode((event.currentTarget as HTMLSelectElement).value as WordCounterMode)}
          >
            <option value="words">Words</option>
            <option value="characters">Characters</option>
            <option value="both">Both</option>
          </select>
        </section>

        <section class="settings-item">
          <div class="settings-copy">
            <span class="settings-label">Paste mode</span>
          </div>
          <select
            class="settings-select"
            value={settings.pasteMode}
            onchange={(event) => updatePasteMode((event.currentTarget as HTMLSelectElement).value as PasteMode)}
          >
            <option value="plain">Plain text (Slate style)</option>
            <option value="basic">Keep basic formatting</option>
          </select>
        </section>

        <section class="settings-item">
          <div class="settings-copy">
            <span class="settings-label">Toolbar style</span>
          </div>
          <select
            class="settings-select"
            value={settings.toolbarMode}
            onchange={(event) => updateToolbarMode((event.currentTarget as HTMLSelectElement).value as ToolbarMode)}
          >
            <option value="always">Always show toolbar</option>
            <option value="selection">Selection menu only</option>
          </select>
        </section>

        <section class="settings-item">
          <div class="settings-copy">
            <span class="settings-label">Code style</span>
          </div>
          <select
            class="settings-select"
            value={settings.codeTone}
            onchange={(event) => updateCodeTone((event.currentTarget as HTMLSelectElement).value as CodeTone)}
          >
            <option value="plain">Plain code</option>
            <option value="accented">Subtle colorized code</option>
          </select>
        </section>

        <section class="settings-item">
          <div class="settings-copy">
            <span class="settings-label">Close behavior</span>
          </div>
          <select
            class="settings-select"
            value={settings.closeBehavior}
            onchange={(event) =>
              updateCloseBehavior((event.currentTarget as HTMLSelectElement).value as CloseBehavior)}
          >
            <option value="tray">Close to tray</option>
            <option value="quit">Quit app on close button</option>
          </select>
        </section>

        <section class="settings-item toggle-row">
          <label class="settings-copy" for="confirm-delete-toggle">
            <span class="settings-label">Confirm permanent delete</span>
          </label>
          <input
            id="confirm-delete-toggle"
            type="checkbox"
            checked={settings.confirmPermanentDelete}
            onchange={(event) => updateConfirmPermanentDelete((event.currentTarget as HTMLInputElement).checked)}
          />
        </section>

        <section class="settings-item toggle-row">
          <label class="settings-copy" for="cursor-blink-toggle">
            <span class="settings-label">Blinking cursor</span>
            <span class="settings-sub">Uses theme accent color</span>
          </label>
          <input
            id="cursor-blink-toggle"
            type="checkbox"
            checked={settings.cursorBlink}
            onchange={(event) => updateCursorBlink((event.currentTarget as HTMLInputElement).checked)}
          />
        </section>
        </div>
      </div>
    </div>
  {/if}

  {#if pendingPermanentDeleteNoteId}
    <div
      class="confirm-scrim"
      role="button"
      tabindex="0"
      aria-label="Cancel permanent delete"
      onclick={cancelPermanentTrashDelete}
      onkeydown={(event) => {
        if (event.key === 'Enter' || event.key === ' ' || event.key === 'Escape') {
          event.preventDefault();
          cancelPermanentTrashDelete();
        }
      }}
    >
      <div
        class="confirm-panel"
        role="dialog"
        tabindex="-1"
        aria-label="Confirm permanent delete"
        onclick={(event) => event.stopPropagation()}
        onkeydown={(event) => event.stopPropagation()}
      >
        <h3>Delete permanently?</h3>
        <p>This note will be removed from trash and can’t be restored.</p>
        <div class="confirm-actions">
          <button type="button" class="mini-btn" onclick={cancelPermanentTrashDelete}>Cancel</button>
          <button type="button" class="mini-btn danger confirm-delete-btn" onclick={confirmPermanentTrashDelete}>Delete</button>
        </div>
      </div>
    </div>
  {/if}
</main>

{#if lightboxImageSrc}
  <div class="lightbox-scrim" role="presentation">
    <button type="button" class="lightbox-dismiss" aria-label="Close image preview" onclick={closeImageLightbox}></button>
    <figure class="lightbox-frame">
      <img src={lightboxImageSrc} alt="Expanded note" />
      {#if lightboxImageCaption}
        <figcaption>{lightboxImageCaption}</figcaption>
      {/if}
    </figure>
  </div>
{/if}

<style>
  :global(html, body) {
    margin: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    font-family: 'SF Pro Text', 'Avenir Next', 'Helvetica Neue', sans-serif;
    color: #202228;
    background: transparent;
  }

  :global(html[data-theme='dark'] body) {
    background: transparent;
    color: #f3f5f9;
  }

  :global(*) {
    box-sizing: border-box;
  }

  .app-shell {
    --bg-1: #fbfcfe;
    --bg-2: #fdfdff;
    --bg-3: #f8faff;
    --bg-4: #fdfefe;
    --chrome-bg: #f4f6fb;
    --line-1: #d8deea;
    --line-2: #e4e8f1;
    --text-1: #202228;
    --text-2: #81889a;
    --text-3: #8b92a3;
    --hover-1: #f2f5fb;
    --hover-2: #ebf1ff;
    --chip-bg: #ffffff;
    --selection-bg: #3a5d9a;
    --selection-fg: #f6f9ff;
    --settings-bg: #f4f7fe;
    --settings-item-bg: #ffffff;
    --accent-caret: #6aa3ff;
    --accent-primary: #6aa3ff;
    --accent-primary-soft: #dae8ff;
    --link-1: #2b9da1;
    --link-2: #37b2b6;
    --editor-line-height: 1.64;
    --paragraph-spacing: 0.42em;
    --shell-safe-inset: clamp(0px, calc((var(--ui-scale) - 1) * 26px), 10px);

    height: calc((100vh - (var(--shell-safe-inset) * 2)) / var(--ui-scale));
    width: calc((100% - (var(--shell-safe-inset) * 2)) / var(--ui-scale));
    display: grid;
    grid-template-columns: 232px minmax(0, 1fr);
    grid-template-rows: 40px 1fr 24px;
    grid-template-areas:
      'chrome chrome'
      'sidebar editor'
      'status status';
    border: 1px solid var(--line-1);
    border-radius: 12px;
    margin: var(--shell-safe-inset);
    position: relative;
    overflow: hidden;
    background: var(--bg-1);
    box-shadow: 0 8px 24px rgba(17, 24, 39, 0.08);
    transform: scale(var(--ui-scale));
    transform-origin: top left;
    will-change: transform;
  }

  .app-shell.dark {
    --bg-1: #1a1f26;
    --bg-2: #1f242d;
    --bg-3: #1b2029;
    --bg-4: #181d25;
    --chrome-bg: #1b2028;
    --line-1: #2c3340;
    --line-2: #2b313c;
    --text-1: #e9edf6;
    --text-2: #a3aaba;
    --text-3: #98a0b1;
    --hover-1: #2a303b;
    --hover-2: #2a3447;
    --chip-bg: #222934;
    --selection-bg: #4a6fae;
    --selection-fg: #f6f9ff;
    --accent-caret: #85b3ff;
    --accent-primary: #85b3ff;
    --accent-primary-soft: #203457;
    --link-1: #4fc3bc;
    --link-2: #67d8d1;
    --settings-bg: #1a2230;
    --settings-item-bg: #222c3b;
  }

  .app-shell.sidebar-closed {
    grid-template-columns: 0 1fr;
  }

  .app-shell[data-line-spacing='normal'] {
    --editor-line-height: 1.64;
    --paragraph-spacing: 0.42em;
  }

  .app-shell[data-line-spacing='loose'] {
    --editor-line-height: 1.72;
    --paragraph-spacing: 0.52em;
  }

  .app-shell[data-line-spacing='wide'] {
    --editor-line-height: 1.82;
    --paragraph-spacing: 0.62em;
  }

  .window-chrome {
    grid-area: chrome;
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto;
    align-items: center;
    height: 40px;
    padding: 0 10px;
    background: var(--chrome-bg);
  }

  .save-status {
    grid-area: status;
    background: var(--bg-1);
    color: var(--text-2);
    font-size: 11px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 7px;
    padding: 0;
  }

  .status-sep {
    opacity: 0.5;
  }

  .save-dot {
    width: 6px;
    height: 6px;
    border-radius: 999px;
    background: var(--accent-primary);
    opacity: 0.8;
  }

  .save-dot.saving {
    animation: save-pulse 0.9s infinite;
  }

  .chrome-left {
    display: flex;
    align-items: center;
    justify-self: start;
  }

  .traffic {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .traffic-btn {
    width: 12px;
    height: 12px;
    border: 0;
    border-radius: 999px;
    cursor: pointer;
    padding: 0;
  }

  .traffic-btn.close {
    background: #ff5f57;
  }

  .traffic-btn.mini {
    background: #febc2e;
  }

  .traffic-btn.max {
    background: #28c840;
  }

  .chrome-drag {
    justify-self: stretch;
    align-self: stretch;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 0;
    background: transparent;
    margin: 0;
    padding: 6px 0 0;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.02em;
    color: var(--text-3);
    cursor: grab;
    user-select: none;
  }

  .chrome-drag:active {
    cursor: grabbing;
  }

  .chrome-right {
    display: flex;
    align-items: center;
    gap: 4px;
    min-width: 0;
    flex-wrap: nowrap;
  }

  .chrome-right {
    justify-self: end;
  }

  .mini-btn,
  .search-wrap input,
  .trash-toggle {
    border: 1px solid var(--line-1);
    border-radius: 7px;
    background: var(--chip-bg);
    color: var(--text-2);
    transition: all 140ms ease;
  }

  .mini-btn:hover,
  .search-wrap input:hover,
  .trash-toggle:hover {
    border-color: var(--accent-primary);
    color: var(--text-1);
  }

  .icon-btn,
  .tool-btn,
  .tool-select {
    border: 0;
    border-radius: 9px;
    background: color-mix(in srgb, var(--chip-bg) 72%, transparent 28%);
    color: var(--text-2);
    transition: all 140ms ease;
  }

  .icon-btn:hover,
  .tool-btn:hover,
  .tool-select:hover {
    background: color-mix(in srgb, var(--chip-bg) 90%, var(--hover-1) 10%);
    color: var(--text-1);
  }

  .icon-btn {
    width: 26px;
    height: 26px;
    font-size: 15px;
    line-height: 1;
    cursor: pointer;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .list-icon {
    width: 11px;
    height: 8px;
    border-top: 2px solid currentColor;
    border-bottom: 2px solid currentColor;
    position: relative;
  }

  .list-icon::after {
    content: '';
    position: absolute;
    left: 0;
    right: 0;
    top: 2px;
    border-top: 2px solid currentColor;
  }

  .theme-toggle-top {
    font-size: 12px;
  }

  .pin-icon {
    width: 10px;
    height: 10px;
    border: 1.5px solid currentColor;
    border-radius: 999px 999px 999px 2px;
    transform: rotate(-45deg);
    position: relative;
    display: inline-block;
  }

  .pin-icon::after {
    content: '';
    position: absolute;
    width: 1.5px;
    height: 8px;
    background: currentColor;
    left: 4px;
    top: 7px;
    border-radius: 999px;
    transform: rotate(45deg);
  }

  .pin-icon.pinned {
    background: currentColor;
    opacity: 0.9;
  }

  .notes-panel {
    grid-area: sidebar;
    border-right: 1px solid var(--line-2);
    background: linear-gradient(180deg, var(--bg-2) 0%, var(--bg-3) 100%);
    display: flex;
    flex-direction: column;
    min-height: 0;
    min-width: 0;
    overflow: hidden;
  }

  .search-wrap {
    padding: 0 10px 8px;
  }

  .search-wrap input {
    width: 100%;
    height: 30px;
    padding: 0 10px;
    border: 0;
    border-radius: 9px;
    background: color-mix(in srgb, var(--chip-bg) 70%, transparent 30%);
    color: var(--text-1);
    font-size: 12px;
    outline: none;
  }

  .search-wrap input:focus {
    background: color-mix(in srgb, var(--chip-bg) 84%, var(--hover-1) 16%);
  }

  .sort-wrap {
    padding: 0 10px 8px;
  }

  .sort-select {
    width: 100%;
    height: 26px;
    border: 1px solid var(--line-1);
    border-radius: 8px;
    background: color-mix(in srgb, var(--chip-bg) 76%, transparent 24%);
    color: var(--text-2);
    font-size: 11px;
    padding: 0 26px 0 9px;
    outline: none;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-image:
      linear-gradient(45deg, transparent 50%, var(--text-3) 50%),
      linear-gradient(135deg, var(--text-3) 50%, transparent 50%);
    background-position:
      calc(100% - 13px) 11px,
      calc(100% - 9px) 11px;
    background-size: 4px 4px, 4px 4px;
    background-repeat: no-repeat;
  }

  .sort-select:hover {
    border-color: var(--accent-primary);
    color: var(--text-1);
  }

  .note-list,
  .trash-list {
    list-style: none;
    margin: 0;
    padding: 0 8px 10px;
    overflow: auto;
    min-height: 0;
  }

  .empty-state {
    padding: 10px 8px;
    font-size: 11px;
    color: var(--text-3);
  }

  .note-card {
    display: flex;
    align-items: center;
    border-radius: 8px;
    min-width: 0;
    transition: background 140ms ease;
  }

  .note-card:hover {
    background: var(--hover-1);
  }

  .note-card.active {
    background: var(--hover-2);
  }

  .note-item {
    flex: 1;
    border: 0;
    border-radius: 8px;
    background: transparent;
    text-align: left;
    padding: 8px 8px;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    overflow: hidden;
  }

  .title {
    display: block;
    max-width: 100%;
    font-size: 12px;
    color: var(--text-1);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meta {
    font-size: 10px;
    color: var(--text-3);
  }

  .note-delete {
    border: 0;
    background: transparent;
    width: 22px;
    height: 22px;
    border-radius: 6px;
    margin-right: 6px;
    color: var(--text-3);
    cursor: pointer;
    font-size: 15px;
    line-height: 1;
    opacity: 0;
    transition: all 140ms ease;
  }

  .note-card:hover .note-delete,
  .note-card.active .note-delete {
    opacity: 1;
  }

  .note-delete:hover {
    color: var(--accent-primary);
    background: color-mix(in srgb, var(--chip-bg) 80%, var(--accent-primary-soft) 20%);
  }

  .trash-wrap {
    border-top: 1px solid var(--line-2);
    padding: 8px;
  }

  .trash-toggle {
    width: 100%;
    height: 28px;
    border: 0;
    border-radius: 9px;
    background: color-mix(in srgb, var(--chip-bg) 70%, transparent 30%);
    padding: 0 10px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 11px;
    cursor: pointer;
  }

  .trash-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    border-radius: 8px;
    padding: 7px 8px;
  }

  .trash-item:hover {
    background: var(--hover-1);
  }

  .trash-copy {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .trash-actions {
    display: flex;
    gap: 4px;
  }

  .mini-btn {
    height: 22px;
    padding: 0 7px;
    font-size: 10px;
    cursor: pointer;
  }

  .mini-btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .mini-btn.danger:hover {
    border-color: #c2838c;
    color: #b05366;
  }

  .editor-panel {
    grid-area: editor;
    display: flex;
    flex-direction: column;
    min-height: 0;
    background: var(--bg-4);
  }

  .toolbar {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 6px 8px;
    overflow: visible;
    position: relative;
  }

  .tool-group {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  .tool-btn {
    height: 24px;
    min-width: 24px;
    padding: 0 7px;
    font-size: 11px;
    line-height: 1;
    cursor: pointer;
  }

  .selection-menu {
    position: fixed;
    z-index: 35;
    transform: translate(-50%, -100%);
    background: color-mix(in srgb, var(--bg-1) 90%, #0f1623 10%);
    border: 1px solid color-mix(in srgb, var(--line-1) 78%, #1b2740 22%);
    border-radius: 9px;
    padding: 4px;
    display: inline-flex;
    align-items: center;
    gap: 4px;
    box-shadow: 0 10px 22px rgba(10, 14, 24, 0.25);
  }

  .selection-menu .tool-btn {
    min-width: 26px;
    height: 26px;
  }

  .tool-select {
    height: 24px;
    padding: 0 8px;
    font-size: 11px;
    color: var(--text-2);
    cursor: pointer;
    outline: none;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-image: none;
    box-shadow: none;
  }

  .tool-menu-wrap {
    position: relative;
    display: inline-flex;
  }

  .menu-toggle {
    min-width: 28px;
    font-size: 16px;
    line-height: 1;
  }

  .tool-menu {
    position: absolute;
    top: calc(100% + 6px);
    right: 0;
    z-index: 36;
    min-width: 178px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 8px;
    border-radius: 10px;
    border: 1px solid var(--line-1);
    background: color-mix(in srgb, var(--bg-1) 94%, #0f1623 6%);
    box-shadow: 0 12px 28px rgba(10, 14, 24, 0.28);
  }

  .menu-label {
    font-size: 10px;
    color: var(--text-3);
    letter-spacing: 0.02em;
    padding: 0 2px;
  }

  .menu-select {
    width: 100%;
    height: 27px;
    border: 1px solid var(--line-1);
    border-radius: 8px;
    color: var(--text-1);
    background: color-mix(in srgb, var(--chip-bg) 88%, transparent 12%);
    padding: 0 25px 0 8px;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-image:
      linear-gradient(45deg, transparent 50%, var(--text-3) 50%),
      linear-gradient(135deg, var(--text-3) 50%, transparent 50%);
    background-position:
      calc(100% - 12px) 12px,
      calc(100% - 8px) 12px;
    background-size: 4px 4px, 4px 4px;
    background-repeat: no-repeat;
  }

  .menu-select:focus {
    outline: none;
    border-color: color-mix(in srgb, var(--accent-caret) 60%, var(--line-1) 40%);
  }

  .menu-actions {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 5px;
  }

  .menu-actions .mini-btn {
    height: 24px;
    border-radius: 7px;
    border: 1px solid var(--line-1);
    background: color-mix(in srgb, var(--chip-bg) 84%, transparent 16%);
    color: var(--text-2);
    font-size: 10px;
    padding: 0 7px;
  }

  .menu-actions .mini-btn:hover {
    color: var(--text-1);
    border-color: var(--accent-primary);
  }

  .editor {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    outline: none;
    background: transparent;
    padding: 16px 34px 24px;
    color: var(--text-1);
    font-size: 16px;
    line-height: var(--editor-line-height);
    letter-spacing: 0.002em;
    caret-color: var(--accent-caret);
    white-space: pre-wrap;
    word-break: break-word;
  }

  .editor:empty::before {
    content: 'Start writing…';
    color: var(--text-3);
  }

  .editor :global(h1),
  .editor :global(h2),
  .editor :global(h3),
  .editor :global(h4) {
    margin: 0.56em 0 0.3em;
    line-height: 1.2;
    max-width: 100%;
    white-space: normal;
    overflow-wrap: anywhere;
  }

  .editor :global(h1) {
    font-size: 1.8rem;
  }

  .editor :global(h2) {
    font-size: 1.45rem;
  }

  .editor :global(h3) {
    font-size: 1.2rem;
  }

  .editor :global(h4) {
    font-size: 1rem;
  }

  .editor :global(p) {
    margin: var(--paragraph-spacing) 0;
    max-width: 100%;
    white-space: normal;
    overflow-wrap: anywhere;
  }

  .app-shell[data-cursor-blink='on'] .editor {
    animation: slate-caret-blink 1.08s steps(1, end) infinite;
  }

  .app-shell[data-cursor-blink='off'] .editor {
    animation: none;
    caret-color: transparent;
  }

  .editor :global(blockquote) {
    margin: 0.75em 0;
    padding: 0.4em 0.7em;
    border-left: 2px solid var(--accent-primary);
    border-radius: 5px;
    background: color-mix(in srgb, var(--chip-bg) 85%, var(--accent-primary-soft) 15%);
    color: var(--text-2);
    display: inline-block;
    width: fit-content;
    max-width: 100%;
  }

  .app-shell:not(.dark) .editor :global(blockquote) {
    background: color-mix(in srgb, var(--accent-primary-soft) 56%, #ffffff 44%);
    border-left-color: var(--accent-primary);
    color: color-mix(in srgb, var(--text-1) 88%, #364463 12%);
  }

  .editor :global(a) {
    color: var(--link-1);
    text-decoration-color: color-mix(in srgb, var(--link-1) 70%, transparent 30%);
    text-underline-offset: 2px;
  }

  .editor :global(a:hover) {
    color: var(--link-2);
  }

  .editor :global(pre) {
    margin: 0.65em 0;
    padding: 0.55em 0.7em;
    border: 1px solid var(--line-1);
    border-radius: 6px;
    background: color-mix(in srgb, var(--chip-bg) 88%, #d9e2f3 12%);
    color: var(--text-1);
    display: inline-block;
    width: fit-content;
    max-width: 100%;
    overflow-x: hidden;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    word-break: break-word;
    font-family: 'SF Mono', 'JetBrains Mono', ui-monospace, 'Menlo', monospace;
    font-size: 0.92em;
  }

  .editor :global(pre code) {
    background: transparent;
    padding: 0;
    border: 0;
  }

  .app-shell[data-code-tone='accented'] .editor :global(pre code) {
    color: var(--accent-primary);
  }

  .app-shell[data-code-tone='accented'] .editor :global(pre) {
    background: color-mix(in srgb, var(--chip-bg) 82%, var(--accent-primary-soft) 18%);
  }

  .editor :global(ul) {
    margin: 0.45em 0;
    padding-left: 1.15em;
  }

  .editor :global(hr) {
    border: 0;
    border-top: 1px solid var(--line-1);
    margin: 1em 0;
  }

  .editor :global(::selection) {
    background: var(--selection-bg);
    color: var(--selection-fg);
    text-shadow: none;
  }

  .editor :global(::-moz-selection) {
    background: var(--selection-bg);
    color: var(--selection-fg);
    text-shadow: none;
  }

  .editor :global(*::selection) {
    background: var(--selection-bg);
    color: var(--selection-fg);
    text-shadow: none;
  }

  .editor :global(*::-moz-selection) {
    background: var(--selection-bg);
    color: var(--selection-fg);
    text-shadow: none;
  }

  .editor :global(figure.note-image) {
    display: block;
    margin: 0.8em 0;
    max-width: min(680px, 100%);
  }

  .editor :global(figure.note-image img) {
    display: block;
    width: auto;
    max-width: min(100%, 680px);
    max-height: 420px;
    border-radius: 7px;
    border: 1px solid var(--line-1);
    box-shadow: 0 3px 16px rgba(11, 18, 32, 0.16);
    cursor: zoom-in;
  }

  .editor :global(figure.note-image figcaption) {
    margin-top: 6px;
    font-size: 12px;
    color: var(--text-2);
    min-height: 1.3em;
    outline: none;
  }

  .editor :global(figure.note-image figcaption:empty::before) {
    content: attr(data-placeholder);
    color: var(--text-3);
  }

  .settings-scrim {
    position: absolute;
    inset: 0;
    background: rgba(10, 16, 28, 0.46);
    backdrop-filter: blur(1.5px);
    -webkit-backdrop-filter: blur(1.5px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
    padding: 18px;
  }

  .settings-panel {
    --settings-panel-bg: var(--settings-bg);
    --settings-border: var(--line-1);
    --settings-text: var(--text-1);
    --settings-subtext: var(--text-3);
    --settings-accent: var(--accent-primary);

    width: min(520px, calc(100vw - 24px));
    max-height: min(78vh, 620px);
    overflow: auto;
    border-radius: 12px;
    background: var(--settings-panel-bg);
    color: var(--settings-text);
    box-shadow:
      0 28px 64px rgba(2, 8, 20, 0.58),
      0 6px 24px rgba(2, 8, 20, 0.34);
    border: 1px solid var(--settings-border);
    opacity: 1;
  }

  .confirm-scrim {
    position: absolute;
    inset: 0;
    z-index: 58;
    background: rgba(10, 16, 28, 0.34);
    backdrop-filter: blur(1px);
    -webkit-backdrop-filter: blur(1px);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
  }

  .confirm-panel {
    width: min(340px, calc(100vw - 28px));
    border-radius: 10px;
    border: 1px solid var(--line-1);
    background: color-mix(in srgb, var(--bg-1) 94%, #121a27 6%);
    box-shadow:
      0 20px 52px rgba(2, 8, 20, 0.42),
      0 4px 18px rgba(2, 8, 20, 0.24);
    padding: 14px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .confirm-panel h3 {
    margin: 0;
    font-size: 14px;
    color: var(--text-1);
  }

  .confirm-panel p {
    margin: 0;
    font-size: 12px;
    color: var(--text-2);
    line-height: 1.45;
  }

  .confirm-actions {
    margin-top: 2px;
    display: flex;
    justify-content: flex-end;
    gap: 6px;
  }

  .confirm-delete-btn {
    background: color-mix(in srgb, var(--accent-primary) 24%, var(--chip-bg) 76%);
    border-color: color-mix(in srgb, var(--accent-primary) 56%, var(--line-1) 44%);
    color: color-mix(in srgb, #ffffff 84%, var(--accent-primary) 16%);
  }

  .settings-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 14px 8px;
  }

  .settings-head h2 {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.01em;
    color: var(--settings-subtext);
  }

  .settings-close {
    width: 26px;
    height: 26px;
  }

  .settings-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 6px 12px 14px;
  }

  .settings-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    border-radius: 10px;
    padding: 10px 11px;
    background: var(--settings-item-bg);
  }

  .settings-item-stack {
    align-items: flex-start;
    flex-direction: column;
  }

  .settings-copy {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .settings-label {
    font-size: 12px;
    color: var(--settings-text);
  }

  .settings-sub {
    font-size: 10px;
    color: var(--settings-subtext);
    margin-top: 2px;
  }

  .settings-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .theme-file-input {
    display: none;
  }

  .settings-select {
    border: 1px solid var(--settings-border);
    border-radius: 8px;
    height: 28px;
    padding: 0 28px 0 9px;
    background: var(--settings-item-bg);
    color: var(--settings-text);
    min-width: 190px;
    font-size: 11px;
    outline: none;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-image:
      linear-gradient(45deg, transparent 50%, var(--settings-subtext) 50%),
      linear-gradient(135deg, var(--settings-subtext) 50%, transparent 50%);
    background-position:
      calc(100% - 14px) 12px,
      calc(100% - 9px) 12px;
    background-size: 5px 5px, 5px 5px;
    background-repeat: no-repeat;
  }

  .settings-range {
    width: 170px;
    accent-color: var(--settings-accent);
  }

  .toggle-row input[type='checkbox'] {
    appearance: none;
    -webkit-appearance: none;
    width: 34px;
    height: 20px;
    border-radius: 999px;
    border: 1px solid var(--settings-border);
    background: var(--settings-item-bg);
    position: relative;
    cursor: pointer;
    transition: all 140ms ease;
  }

  .toggle-row input[type='checkbox']::after {
    content: '';
    position: absolute;
    width: 14px;
    height: 14px;
    border-radius: 999px;
    background: var(--settings-subtext);
    top: 2px;
    left: 2px;
    transition: all 140ms ease;
  }

  .toggle-row input[type='checkbox']:checked {
    background: color-mix(in srgb, var(--settings-accent) 44%, var(--settings-item-bg) 56%);
    border-color: color-mix(in srgb, var(--settings-accent) 60%, var(--settings-border) 40%);
  }

  .toggle-row input[type='checkbox']:checked::after {
    left: 17px;
    background: #f8fbff;
  }

  .lightbox-scrim {
    position: fixed;
    inset: 0;
    z-index: 70;
    background: rgba(8, 12, 18, 0.72);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 18px;
  }

  .lightbox-dismiss {
    position: absolute;
    inset: 0;
    border: 0;
    background: transparent;
    cursor: zoom-out;
  }

  .lightbox-frame {
    margin: 0;
    max-width: min(92vw, 1080px);
    max-height: 88vh;
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-items: center;
    position: relative;
    z-index: 1;
  }

  .lightbox-frame img {
    max-width: 100%;
    max-height: calc(88vh - 40px);
    border-radius: 9px;
    box-shadow: 0 10px 40px rgba(2, 8, 20, 0.45);
    border: 1px solid rgba(190, 206, 235, 0.28);
  }

  .lightbox-frame figcaption {
    color: #d7e1f1;
    font-size: 12px;
    text-align: center;
  }

  @media (max-width: 860px) {
    .app-shell {
      grid-template-columns: 200px minmax(0, 1fr);
      margin: 6px;
      height: calc((100vh - 12px) / var(--ui-scale));
      width: calc((100% - 12px) / var(--ui-scale));
      border-radius: 8px;
    }

    .window-chrome {
      grid-template-columns: auto minmax(0, 1fr) auto;
      padding: 0 6px;
    }

    .chrome-drag {
      font-size: 11px;
      padding-top: 4px;
    }

    .icon-btn {
      width: 22px;
      height: 22px;
      font-size: 13px;
      border-radius: 7px;
    }

    .list-icon {
      width: 9px;
      height: 7px;
      border-top-width: 1.5px;
      border-bottom-width: 1.5px;
    }

    .list-icon::after {
      top: 1.5px;
      border-top-width: 1.5px;
    }

    .editor {
      font-size: 15px;
      padding: 14px 22px 18px;
    }
  }

  @keyframes save-pulse {
    0% {
      transform: scale(1);
      opacity: 0.9;
    }
    50% {
      transform: scale(1.25);
      opacity: 0.35;
    }
    100% {
      transform: scale(1);
      opacity: 0.9;
    }
  }

  @keyframes slate-caret-blink {
    0%,
    48% {
      caret-color: var(--accent-caret);
    }
    49%,
    100% {
      caret-color: transparent;
    }
  }
</style>

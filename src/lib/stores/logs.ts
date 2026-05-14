import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';

export type LogLine = { ts: number; level: 'info' | 'warn' | 'error'; text: string };

function make() {
  const { subscribe, update } = writable<LogLine[]>([]);
  function push(level: LogLine['level'], text: string) {
    update(lines => {
      const next = [...lines, { ts: Date.now(), level, text }];
      return next.slice(-500);
    });
  }
  async function init() {
    await listen<{ level: LogLine['level']; text: string }>('log', e => {
      push(e.payload.level, e.payload.text);
    });
  }
  return { subscribe, push, init };
}

export const logs = make();

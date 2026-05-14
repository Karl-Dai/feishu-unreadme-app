import { writable } from 'svelte/store';

export type LogLine = { ts: number; level: 'info' | 'warn' | 'error'; text: string };

function make() {
  const { subscribe, update } = writable<LogLine[]>([]);
  function push(level: LogLine['level'], text: string) {
    update(lines => {
      const next = [...lines, { ts: Date.now(), level, text }];
      return next.slice(-500);
    });
  }
  return { subscribe, push };
}

export const logs = make();

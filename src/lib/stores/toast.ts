import { writable } from 'svelte/store';

export type Toast = { id: number; kind: 'info' | 'error'; text: string };

function makeToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);
  let next = 1;
  return {
    subscribe,
    push(kind: Toast['kind'], text: string, ttl = 4000) {
      const id = next++;
      update(list => [...list, { id, kind, text }]);
      setTimeout(() => update(list => list.filter(t => t.id !== id)), ttl);
    },
    error(text: string) { this.push('error', text, 6000); },
    info(text: string) { this.push('info', text); },
  };
}

export const toasts = makeToastStore();

import { writable } from 'svelte/store';
import { ipc, reportError } from '../ipc';

type State =
  | { kind: 'idle'; current: string }
  | { kind: 'checking' }
  | { kind: 'available'; current: string; next: string; notes: string }
  | { kind: 'up_to_date'; current: string }
  | { kind: 'downloading'; progress: number }
  | { kind: 'ready_to_install' };

function make() {
  const { subscribe, set } = writable<State>({ kind: 'idle', current: '0.0.0' });

  async function init() {
    try {
      const current = await ipc.getAppVersion();
      set({ kind: 'idle', current });
    } catch (e) {
      reportError(e);
    }
  }

  return { subscribe, init };
}

export const appUpdate = make();

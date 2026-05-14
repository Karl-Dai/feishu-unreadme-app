import { writable } from 'svelte/store';
import { ipc, reportError } from '../ipc';
import { toasts } from './toast';

type State =
  | { kind: 'idle'; current: string }
  | { kind: 'checking'; current: string }
  | { kind: 'available'; current: string; next: string; notes: string | null }
  | { kind: 'up_to_date'; current: string }
  | { kind: 'installing'; current: string };

function make() {
  const { subscribe, set, update } = writable<State>({ kind: 'idle', current: '0.0.0' });

  async function init() {
    try {
      const current = await ipc.getAppVersion();
      set({ kind: 'checking', current });
      const info = await ipc.checkAppUpdate();
      if (info) {
        set({ kind: 'available', current, next: info.version, notes: info.notes });
      } else {
        set({ kind: 'up_to_date', current });
      }
    } catch (e) {
      reportError(e);
      const current = await ipc.getAppVersion().catch(() => '?');
      set({ kind: 'idle', current });
    }
  }

  async function install() {
    update(s => 'current' in s ? { kind: 'installing', current: s.current } : s);
    try {
      await ipc.triggerAppUpdate();
      toasts.info('更新已安装,即将重启');
    } catch (e) {
      reportError(e);
    }
  }

  return { subscribe, init, install };
}

export const appUpdate = make();

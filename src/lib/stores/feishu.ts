import { writable } from 'svelte/store';
import { ipc, reportError, type InstallInfo } from '../ipc';

type State =
  | { kind: 'loading' }
  | { kind: 'ok'; info: InstallInfo }
  | { kind: 'not-found'; tried: string[] }
  | { kind: 'error'; message: string };

function make() {
  const { subscribe, set } = writable<State>({ kind: 'loading' });

  async function detect() {
    set({ kind: 'loading' });
    try {
      const info = await ipc.detectFeishuInstall();
      set({ kind: 'ok', info });
    } catch (e) {
      // FeishuNotFound 不是错误条件,是正常分支
      if ((e as { kind?: string }).kind === 'FeishuNotFound') {
        set({ kind: 'not-found', tried: [] });
      } else {
        set({ kind: 'error', message: reportError(e) });
      }
    }
  }

  async function pickManually(path: string) {
    set({ kind: 'loading' });
    try {
      const info = await ipc.validateFeishuPath(path);
      set({ kind: 'ok', info });
    } catch (e) {
      set({ kind: 'error', message: reportError(e) });
    }
  }

  return { subscribe, detect, pickManually };
}

export const feishu = make();

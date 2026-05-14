import { writable } from 'svelte/store';
import { ipc, reportError, type ApplyReport, type InstallInfo, type PatchUiState } from '../ipc';
import { toasts } from './toast';

type State =
  | { kind: 'idle'; ui: PatchUiState | null }
  | { kind: 'working' }
  | { kind: 'last_report'; report: ApplyReport; ui: PatchUiState };

function make() {
  const { subscribe, set, update } = writable<State>({ kind: 'idle', ui: null });

  async function refresh(install: InstallInfo) {
    try {
      const ui = await ipc.getPatchState(install);
      update(s => s.kind === 'last_report' ? { ...s, ui } : { kind: 'idle', ui });
    } catch (e) {
      reportError(e);
    }
  }

  async function apply(install: InstallInfo) {
    set({ kind: 'working' });
    try {
      const report = await ipc.applyPatch(install);
      const ui = await ipc.getPatchState(install);
      set({ kind: 'last_report', report, ui });
      toasts.info(`补丁成功:命中 ${report.patches_hit}/${report.patches_attempted} 条`);
    } catch (e) {
      reportError(e);
      const ui = await ipc.getPatchState(install).catch(() => null);
      set({ kind: 'idle', ui });
    }
  }

  async function restore(install: InstallInfo) {
    set({ kind: 'working' });
    try {
      await ipc.restoreBackup(install);
      const ui = await ipc.getPatchState(install);
      set({ kind: 'idle', ui });
      toasts.info('已恢复备份');
    } catch (e) {
      reportError(e);
      const ui = await ipc.getPatchState(install).catch(() => null);
      set({ kind: 'idle', ui });
    }
  }

  return { subscribe, refresh, apply, restore };
}

export const patch = make();

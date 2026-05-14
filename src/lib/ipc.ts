import { invoke as raw } from '@tauri-apps/api/core';
import { toasts } from './stores/toast';

export type AppErrorWire = {
  kind:
    | 'FeishuNotFound' | 'FeishuRunning' | 'AsarLocked' | 'AsarMalformed'
    | 'BackupExists' | 'BackupMissing' | 'PatchNoHit' | 'PatchVersionIncompatible'
    | 'RemoteFetchFailed' | 'RemoteSignatureInvalid' | 'RemoteVersionRegression'
    | 'PermissionDenied' | 'UpdaterFailed' | 'Io' | 'Internal';
  message: string;
};

export type InstallInfo = {
  install_path: string;
  asar_path: string;
  version: string | null;
  is_running: boolean;
};

export type PatchUiState =
  | { state: 'unpatched' }
  | { state: 'patched'; feishu_version: string; patch_version: string }
  | { state: 'stale'; feishu_version_seen: string }
  | { state: 'unknown' }
  | { state: 'incompatible' };

export type ApplyReport = {
  patches_attempted: number;
  patches_hit: number;
  files_modified: string[];
  backup_path: string;
  hash_before: string;
  hash_after: string;
};

export type AppUpdateInfo = { version: string; notes: string | null };

export class AppError extends Error {
  kind: AppErrorWire['kind'];
  constructor(wire: AppErrorWire) {
    super(wire.message);
    this.kind = wire.kind;
  }
}

async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await raw<T>(cmd, args);
  } catch (e) {
    const wire = e as AppErrorWire;
    if (wire && typeof wire === 'object' && 'kind' in wire) {
      throw new AppError(wire);
    }
    throw new Error(String(e));
  }
}

export const ipc = {
  detectFeishuInstall: () => call<InstallInfo>('detect_feishu_install'),
  validateFeishuPath:  (path: string) => call<InstallInfo>('validate_feishu_path', { path }),
  getPatchState:       (install: InstallInfo) => call<PatchUiState>('get_patch_state', { install }),
  getAppVersion:       () => call<string>('get_app_version'),
  applyPatch:          (install: InstallInfo) => call<ApplyReport>('apply_patch_cmd', { install }),
  restoreBackup:       (install: InstallInfo) => call<void>('restore_backup_cmd', { install }),
  checkAppUpdate:      () => call<AppUpdateInfo | null>('check_app_update'),
  triggerAppUpdate:    () => call<void>('trigger_app_update'),
};

/** 把 AppError 转成用户可读 toast,并把消息返回给调用方便于 inline 显示 */
export function reportError(e: unknown): string {
  if (e instanceof AppError) {
    toasts.error(e.message);
    return e.message;
  }
  const msg = e instanceof Error ? e.message : String(e);
  toasts.error(msg);
  return msg;
}

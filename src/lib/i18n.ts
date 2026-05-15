import { derived, writable } from 'svelte/store';

export type Lang = 'zh' | 'en';

const STORAGE_KEY = 'feishu-unreadme:lang';

function initial(): Lang {
  if (typeof window === 'undefined') return 'zh';
  const v = window.localStorage.getItem(STORAGE_KEY);
  return v === 'en' || v === 'zh' ? v : 'zh';
}

export const lang = writable<Lang>(initial());

lang.subscribe((v) => {
  if (typeof window !== 'undefined') window.localStorage.setItem(STORAGE_KEY, v);
});

export function toggleLang() {
  lang.update((v) => (v === 'zh' ? 'en' : 'zh'));
}

export type Dict = {
  subtitle: string;
  session: string; target: string; mode: string; modeInteractive: string;
  lang: string; langZh: string; langEn: string;

  feishuTagScanning: string; feishuTagReady: string;
  feishuTagNotFound: string; feishuTagError: string;
  feishuScanning: string;
  feishuKvPath: string; feishuKvVersion: string;
  feishuVersionUnknown: string;
  feishuBtnChange: string; feishuBtnPick: string; feishuBtnRetry: string;
  feishuNotFoundHint: string;
  feishuPickerTitle: string;

  patchTagWorking: string; patchTagProbing: string;
  patchTagUnpatched: string; patchTagPatched: string;
  patchTagStale: string; patchTagUnknown: string; patchTagIncompat: string;
  patchDetailWorking: string;
  patchDetailUnpatched: string;
  patchDetailPatched: (v: string, p: string) => string;
  patchDetailStale: (v: string) => string;
  patchDetailUnknown: string;
  patchDetailIncompat: string;
  patchReportSummary: string;
  patchBtnApply: string; patchBtnRestore: string;
  patchRestoreConfirm: string;
  patchDash: string;

  updateTagChecking: string; updateTagUpToDate: string;
  updateTagAvailable: string; updateTagInstalling: string; updateTagIdle: string;
  updateChecking: string; updateInstalling: string;
  updateKvCurrent: string; updateKvRemote: string;
  updateNotesSummary: string;
  updateBtnInstall: string;

  logCount: (n: number) => string;
  logEmpty: string;

  toastMsg: string;
  toastErr: string;
};

const zh: Dict = {
  subtitle: '飞书未读小红点补丁工具',
  session: '会话',
  target: '目标',
  mode: '模式',
  modeInteractive: '交互式',
  lang: '语言',
  langZh: '中',
  langEn: 'EN',

  feishuTagScanning: '扫描中',
  feishuTagReady: '就绪',
  feishuTagNotFound: '未找到',
  feishuTagError: '错误',
  feishuScanning: '扫描 /Applications…',
  feishuKvPath: '路径',
  feishuKvVersion: '版本',
  feishuVersionUnknown: '(未知)',
  feishuBtnChange: '修改路径',
  feishuBtnPick: '选择目录',
  feishuBtnRetry: '重试',
  feishuNotFoundHint: '→ 未自动定位到飞书,请手动选择安装目录。',
  feishuPickerTitle: '选择飞书安装目录',

  patchTagWorking: '处理中',
  patchTagProbing: '探测中',
  patchTagUnpatched: '未补丁',
  patchTagPatched: '已补丁',
  patchTagStale: '需重跑',
  patchTagUnknown: '未知',
  patchTagIncompat: '不兼容',
  patchDetailWorking: '正在写入补丁…',
  patchDetailUnpatched: '当前未打补丁',
  patchDetailPatched: (v: string, p: string) => `飞书 ${v} · 规则 ${p}`,
  patchDetailStale: (v: string) => `飞书已升级(上次记录 ${v}),需要重跑`,
  patchDetailUnknown: '状态未解析',
  patchDetailIncompat: '飞书新版本规则未命中,等待新补丁',
  patchReportSummary: '上次执行报告',
  patchBtnApply: '一键补丁',
  patchBtnRestore: '恢复备份',
  patchRestoreConfirm: '确认恢复备份?将丢弃当前已打的补丁。',
  patchDash: '─',

  updateTagChecking: '检查中',
  updateTagUpToDate: '已是最新',
  updateTagAvailable: '有新版',
  updateTagInstalling: '安装中',
  updateTagIdle: '空闲',
  updateChecking: '正在请求 release 接口…',
  updateInstalling: '正在下载并校验签名…',
  updateKvCurrent: '当前',
  updateKvRemote: '远端',
  updateNotesSummary: '更新说明',
  updateBtnInstall: '下载并安装',

  logCount: (n: number) => `${n} 条`,
  logEmpty: '— 暂无事件 —',

  toastMsg: '消息',
  toastErr: '错误',
};

const en: Dict = {
  subtitle: 'feishu unread badge patcher',
  session: 'session',
  target: 'target',
  mode: 'mode',
  modeInteractive: 'interactive',
  lang: 'lang',
  langZh: '中',
  langEn: 'EN',

  feishuTagScanning: 'SCANNING',
  feishuTagReady: 'READY',
  feishuTagNotFound: 'NOT FOUND',
  feishuTagError: 'ERROR',
  feishuScanning: 'scanning /Applications…',
  feishuKvPath: 'path',
  feishuKvVersion: 'version',
  feishuVersionUnknown: '(unknown)',
  feishuBtnChange: 'change path',
  feishuBtnPick: 'select directory',
  feishuBtnRetry: 'retry',
  feishuNotFoundHint: '→ auto-locate failed. specify install directory manually.',
  feishuPickerTitle: 'choose feishu install directory',

  patchTagWorking: 'WORKING',
  patchTagProbing: 'PROBING',
  patchTagUnpatched: 'UNPATCHED',
  patchTagPatched: 'PATCHED',
  patchTagStale: 'STALE',
  patchTagUnknown: 'UNKNOWN',
  patchTagIncompat: 'INCOMPATIBLE',
  patchDetailWorking: 'applying patch…',
  patchDetailUnpatched: 'no patch installed',
  patchDetailPatched: (v: string, p: string) => `feishu ${v} · ruleset ${p}`,
  patchDetailStale: (v: string) => `feishu upgraded (last seen ${v}) · re-apply needed`,
  patchDetailUnknown: 'state unresolved',
  patchDetailIncompat: 'new feishu version not covered by ruleset · awaiting update',
  patchReportSummary: 'last execution report',
  patchBtnApply: 'apply patch',
  patchBtnRestore: 'restore backup',
  patchRestoreConfirm: 'restore backup? all current patches will be discarded.',
  patchDash: '─',

  updateTagChecking: 'CHECKING',
  updateTagUpToDate: 'UP TO DATE',
  updateTagAvailable: 'NEW VERSION',
  updateTagInstalling: 'INSTALLING',
  updateTagIdle: 'IDLE',
  updateChecking: 'querying release endpoint…',
  updateInstalling: 'downloading binary, verifying signature…',
  updateKvCurrent: 'current',
  updateKvRemote: 'remote',
  updateNotesSummary: 'release notes',
  updateBtnInstall: 'download and install',

  logCount: (n: number) => `${n} lines`,
  logEmpty: '— no events yet —',

  toastMsg: 'MSG',
  toastErr: 'ERR',
};

export const t = derived(lang, ($lang) => ($lang === 'zh' ? zh : en));

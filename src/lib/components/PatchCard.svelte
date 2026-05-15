<script lang="ts">
  import { feishu } from '$lib/stores/feishu';
  import { patch } from '$lib/stores/patch';
  import { t } from '$lib/i18n';

  $: if ($feishu.kind === 'ok') patch.refresh($feishu.info);

  async function onApply() {
    if ($feishu.kind !== 'ok') return;
    await patch.apply($feishu.info);
  }
  async function onRestore() {
    if ($feishu.kind !== 'ok') return;
    if (!confirm($t.patchRestoreConfirm)) return;
    await patch.restore($feishu.info);
  }

  type Tag = { text: string; cls: 'ok' | 'warn' | 'err' | 'muted' | 'accent' };

  function tagFor(s: typeof $patch, dict: typeof $t): Tag {
    if (s.kind === 'working')      return { text: dict.patchTagWorking, cls: 'accent' };
    const ui = 'ui' in s ? s.ui : null;
    if (!ui)                       return { text: dict.patchTagProbing, cls: 'muted' };
    switch (ui.state) {
      case 'unpatched':    return { text: dict.patchTagUnpatched, cls: 'muted' };
      case 'patched':      return { text: dict.patchTagPatched,   cls: 'ok' };
      case 'stale':        return { text: dict.patchTagStale,     cls: 'warn' };
      case 'unknown':      return { text: dict.patchTagUnknown,   cls: 'muted' };
      case 'incompatible': return { text: dict.patchTagIncompat,  cls: 'err' };
    }
  }

  $: tag = tagFor($patch, $t);

  function detail(s: typeof $patch, dict: typeof $t): string {
    if (s.kind === 'working') return dict.patchDetailWorking;
    const ui = 'ui' in s ? s.ui : null;
    if (!ui) return dict.patchDash;
    switch (ui.state) {
      case 'unpatched':    return dict.patchDetailUnpatched;
      case 'patched':      return dict.patchDetailPatched(ui.feishu_version, ui.patch_version);
      case 'stale':        return dict.patchDetailStale(ui.feishu_version_seen);
      case 'unknown':      return dict.patchDetailUnknown;
      case 'incompatible': return dict.patchDetailIncompat;
    }
  }
</script>

<section>
  <div class="head">
    <span class="prompt">patch/status</span>
    <span class="tag tag--{tag.cls}">{tag.text}</span>
  </div>

  <div class="body">
    <p class="detail">→ {detail($patch, $t)}</p>

    {#if $patch.kind === 'last_report'}
      <details>
        <summary>{$t.patchReportSummary}</summary>
        <pre>{JSON.stringify($patch.report, null, 2)}</pre>
      </details>
    {/if}

    <div class="actions">
      <button class="btn" onclick={onApply} disabled={$feishu.kind !== 'ok' || $patch.kind === 'working'}>
        {$t.patchBtnApply}
      </button>
      <span class="sep">·</span>
      <button class="btn btn--danger" onclick={onRestore} disabled={$feishu.kind !== 'ok' || $patch.kind === 'working'}>
        {$t.patchBtnRestore}
      </button>
    </div>
  </div>
</section>

<style>
  section { display: flex; flex-direction: column; gap: 10px; }
  .head { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
  .body { display: flex; flex-direction: column; gap: 12px; }
  .detail { margin: 0; font-size: 12.5px; color: var(--fg-dim); }
  .actions { display: flex; align-items: center; gap: 14px; }
  .sep { color: var(--fg-muted); }
  details summary {
    font-size: 11px; letter-spacing: 0.08em; text-transform: uppercase;
    color: var(--fg-muted); cursor: pointer; user-select: none;
  }
  details summary:hover { color: var(--accent); }
  pre {
    margin: 8px 0 0;
    padding: 10px 12px;
    background: rgba(51, 255, 102, 0.04);
    border-left: 2px solid var(--border);
    font-size: 11.5px;
    color: var(--fg-dim);
    overflow-x: auto;
    line-height: 1.5;
  }
</style>

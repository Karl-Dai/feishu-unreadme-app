<script lang="ts">
  import { feishu } from '$lib/stores/feishu';
  import { patch } from '$lib/stores/patch';

  $: if ($feishu.kind === 'ok') patch.refresh($feishu.info);

  async function onApply() {
    if ($feishu.kind !== 'ok') return;
    await patch.apply($feishu.info);
  }
  async function onRestore() {
    if ($feishu.kind !== 'ok') return;
    if (!confirm('确认恢复备份?将丢弃当前已打的补丁。')) return;
    await patch.restore($feishu.info);
  }

  function label(s: typeof $patch): string {
    if (s.kind === 'working') return '处理中…';
    const ui = 'ui' in s ? s.ui : null;
    if (!ui) return '探测中';
    switch (ui.state) {
      case 'unpatched':    return '未补丁';
      case 'patched':      return `已补丁 (飞书 ${ui.feishu_version}, 规则 ${ui.patch_version})`;
      case 'stale':        return `飞书已升级 (上次记录 ${ui.feishu_version_seen}),需要重跑`;
      case 'unknown':      return '状态未知';
      case 'incompatible': return '飞书新版本规则未命中,等待新补丁';
    }
  }
</script>

<section class="card">
  <h2>补丁状态</h2>
  <p class="status">{label($patch)}</p>
  {#if $patch.kind === 'last_report'}
    <details>
      <summary>上次执行报告</summary>
      <pre>{JSON.stringify($patch.report, null, 2)}</pre>
    </details>
  {/if}
  <div class="actions">
    <button on:click={onApply} disabled={$feishu.kind !== 'ok' || $patch.kind === 'working'}>一键补丁</button>
    <button on:click={onRestore} disabled={$feishu.kind !== 'ok' || $patch.kind === 'working'}>恢复备份</button>
  </div>
</section>

<style>
  .card { background: #fff; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,.06); }
  h2 { margin: 0 0 8px; font-size: 16px; }
  .status { font-size: 14px; margin: 0 0 12px; }
  .actions { display: flex; gap: 8px; }
  button { padding: 6px 12px; border: 1px solid #d0d7de; background: #f6f8fa; border-radius: 6px; cursor: pointer; }
  button[disabled] { opacity: .5; cursor: not-allowed; }
  pre { font-size: 12px; background: #f6f8fa; padding: 8px; border-radius: 4px; overflow-x: auto; }
</style>

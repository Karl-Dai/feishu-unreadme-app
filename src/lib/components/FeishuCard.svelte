<script lang="ts">
  import { feishu } from '$lib/stores/feishu';
  import { open } from '@tauri-apps/plugin-dialog';

  async function pick() {
    const path = await open({ directory: true, multiple: false, title: '选择飞书安装目录' });
    if (typeof path === 'string') {
      await feishu.pickManually(path);
    }
  }
</script>

<section class="card">
  <h2>飞书安装</h2>
  {#if $feishu.kind === 'loading'}
    <p class="muted">探测中…</p>
  {:else if $feishu.kind === 'ok'}
    <p><strong>路径:</strong> <code>{$feishu.info.install_path}</code></p>
    <p><strong>版本:</strong> {$feishu.info.version ?? '(未知)'}</p>
    <p class="muted">asar: <code>{$feishu.info.asar_path}</code></p>
    <button on:click={pick}>改路径</button>
  {:else if $feishu.kind === 'not-found'}
    <p class="warn">未自动找到飞书,请手动选择安装目录</p>
    <button on:click={pick}>选择目录</button>
  {:else}
    <p class="err">{$feishu.message}</p>
    <button on:click={pick}>重新选择</button>
  {/if}
</section>

<style>
  .card { background: #fff; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,.06); }
  h2 { margin: 0 0 12px; font-size: 16px; }
  .muted { color: #57606a; font-size: 13px; }
  .warn { color: #9a6700; }
  .err  { color: #b00020; }
  code { font-family: ui-monospace, SFMono-Regular, monospace; font-size: 12px; }
  button { padding: 6px 12px; border: 1px solid #d0d7de; background: #f6f8fa; border-radius: 6px; cursor: pointer; }
</style>

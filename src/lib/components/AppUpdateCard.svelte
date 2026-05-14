<script lang="ts">
  import { appUpdate } from '$lib/stores/app_update';
</script>

<section class="card">
  <h2>应用更新</h2>
  {#if $appUpdate.kind === 'checking'}
    <p>检测中… (当前 v{$appUpdate.current})</p>
  {:else if $appUpdate.kind === 'up_to_date'}
    <p>已是最新版本(v{$appUpdate.current})</p>
  {:else if $appUpdate.kind === 'available'}
    <p>当前 v{$appUpdate.current} → 新版 <strong>v{$appUpdate.next}</strong></p>
    {#if $appUpdate.notes}
      <details><summary>更新说明</summary><pre>{$appUpdate.notes}</pre></details>
    {/if}
    <button onclick={() => appUpdate.install()}>下载并安装</button>
  {:else if $appUpdate.kind === 'installing'}
    <p>正在下载并安装…</p>
  {:else}
    <p>当前版本:v{$appUpdate.current}</p>
  {/if}
</section>

<style>
  .card { background: #fff; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,.06); }
  h2 { margin: 0 0 8px; font-size: 16px; }
  pre { font-size: 12px; background: #f6f8fa; padding: 8px; border-radius: 4px; }
  button { padding: 6px 12px; border: 1px solid #d0d7de; background: #f6f8fa; border-radius: 6px; cursor: pointer; }
</style>

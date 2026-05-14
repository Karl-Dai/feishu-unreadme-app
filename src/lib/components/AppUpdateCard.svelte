<script lang="ts">
  import { appUpdate } from '$lib/stores/app_update';
</script>

<section class="card">
  <h2>应用更新</h2>
  {#if $appUpdate.kind === 'idle'}
    <p>当前版本:<strong>v{$appUpdate.current}</strong></p>
    <p class="muted">更新检测将在 Task 16 接入</p>
  {:else}
    <p>检测中…</p>
  {/if}
</section>

<style>
  .card { background: #fff; border-radius: 8px; padding: 16px; box-shadow: 0 1px 3px rgba(0,0,0,.06); }
  h2 { margin: 0 0 8px; font-size: 16px; }
  .muted { color: #57606a; font-size: 13px; }
</style>

<script lang="ts">
  import { logs } from '$lib/stores/logs';
  let open = $state(false);
</script>

<div class="drawer" class:open>
  <button class="handle" on:click={() => open = !open}>
    {open ? '收起日志' : `展开日志 (${$logs.length})`}
  </button>
  {#if open}
    <div class="body">
      {#each $logs as l}
        <div class="line {l.level}">
          <span class="ts">{new Date(l.ts).toLocaleTimeString()}</span>
          <span>{l.text}</span>
        </div>
      {/each}
      {#if $logs.length === 0}
        <p class="muted">暂无日志</p>
      {/if}
    </div>
  {/if}
</div>

<style>
  .drawer { position: fixed; bottom: 0; left: 0; right: 0; background: #1f2328; color: #f6f8fa; font-size: 12px; }
  .handle { width: 100%; padding: 6px; border: 0; background: #24292f; color: inherit; cursor: pointer; }
  .body { max-height: 200px; overflow: auto; padding: 8px; font-family: ui-monospace, SFMono-Regular, monospace; }
  .line { display: flex; gap: 8px; }
  .ts { color: #8b949e; }
  .info { color: #f6f8fa; }
  .warn { color: #f9c513; }
  .error { color: #ff7b72; }
  .muted { color: #8b949e; }
</style>

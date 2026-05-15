<script lang="ts">
  import { logs } from '$lib/stores/logs';
  import { t, lang } from '$lib/i18n';
  let open = $state(false);
</script>

<div class="drawer" class:open>
  <button class="handle" onclick={() => (open = !open)}>
    <span class="left">
      <span class="prompt">tail</span>
      <span class="path">var/log/feishu-unreadme</span>
    </span>
    <span class="right">
      <span class="count">{$t.logCount($logs.length)}</span>
      <span class="caret">{open ? '▼' : '▲'}</span>
    </span>
  </button>

  {#if open}
    <div class="body">
      {#if $logs.length === 0}
        <p class="empty">{$t.logEmpty}</p>
      {:else}
        {#each $logs as l}
          <div class="line {l.level}">
            <span class="ts">{new Date(l.ts).toLocaleTimeString($lang === 'zh' ? 'zh-CN' : 'en-US', { hour12: false })}</span>
            <span class="lvl">{l.level.toUpperCase().padEnd(5)}</span>
            <span class="msg">{l.text}</span>
          </div>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<style>
  .drawer {
    position: fixed;
    bottom: 0; left: 0; right: 0;
    z-index: 50;
    background: linear-gradient(to top, rgba(10, 14, 10, 0.98), rgba(10, 14, 10, 0.92));
    border-top: 1px solid var(--border);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
  }
  .handle {
    width: 100%;
    padding: 8px 32px;
    background: transparent;
    border: 0;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 11px;
    letter-spacing: 0.08em;
    color: var(--fg-dim);
  }
  .handle:hover { color: var(--fg); }
  .left, .right { display: flex; align-items: center; gap: 12px; }
  .path { color: var(--fg-muted); }
  .count { color: var(--fg-muted); }
  .caret { color: var(--fg-dim); }
  .body {
    max-height: 220px;
    overflow: auto;
    padding: 8px 32px 14px;
    font-size: 12px;
    line-height: 1.65;
  }
  .empty { margin: 0; color: var(--fg-muted); font-size: 11px; letter-spacing: 0.08em; }
  .line { display: grid; grid-template-columns: 70px 60px 1fr; gap: 12px; }
  .ts  { color: var(--fg-muted); }
  .lvl { color: var(--fg-muted); letter-spacing: 0.08em; font-size: 10.5px; align-self: center; }
  .msg { color: var(--fg-dim); word-break: break-all; }
  .line.info  .msg { color: var(--fg); }
  .line.warn  .lvl { color: var(--warn); }
  .line.warn  .msg { color: var(--warn); }
  .line.error .lvl { color: var(--danger); }
  .line.error .msg { color: var(--danger); }
  .body::-webkit-scrollbar { width: 6px; height: 6px; }
  .body::-webkit-scrollbar-thumb { background: var(--border); border-radius: 0; }
</style>

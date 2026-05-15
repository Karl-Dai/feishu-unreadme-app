<script lang="ts">
  import { appUpdate } from '$lib/stores/app_update';
  import { t } from '$lib/i18n';
</script>

<section>
  <div class="head">
    <span class="prompt">updater/check</span>
    {#if $appUpdate.kind === 'checking'}
      <span class="tag tag--muted">{$t.updateTagChecking}</span>
    {:else if $appUpdate.kind === 'up_to_date'}
      <span class="tag tag--ok">{$t.updateTagUpToDate}</span>
    {:else if $appUpdate.kind === 'available'}
      <span class="tag tag--accent">{$t.updateTagAvailable}</span>
    {:else if $appUpdate.kind === 'installing'}
      <span class="tag tag--accent">{$t.updateTagInstalling}</span>
    {:else}
      <span class="tag tag--muted">{$t.updateTagIdle}</span>
    {/if}
  </div>

  <div class="body">
    {#if $appUpdate.kind === 'checking'}
      <p class="dim">{$t.updateChecking}<span class="cursor"></span></p>
      <p class="kv">{$t.updateKvCurrent} <span class="v">v{$appUpdate.current}</span></p>
    {:else if $appUpdate.kind === 'up_to_date'}
      <p class="kv">{$t.updateKvCurrent} <span class="v">v{$appUpdate.current}</span></p>
    {:else if $appUpdate.kind === 'available'}
      <p class="kv">
        {$t.updateKvCurrent} <span class="v dim">v{$appUpdate.current}</span>
        <span class="arrow">→</span>
        {$t.updateKvRemote} <span class="v accent">v{$appUpdate.next}</span>
      </p>
      {#if $appUpdate.notes}
        <details>
          <summary>{$t.updateNotesSummary}</summary>
          <pre>{$appUpdate.notes}</pre>
        </details>
      {/if}
      <button class="btn" onclick={() => appUpdate.install()}>{$t.updateBtnInstall}</button>
    {:else if $appUpdate.kind === 'installing'}
      <p class="dim">{$t.updateInstalling}<span class="cursor"></span></p>
    {:else}
      <p class="kv">{$t.updateKvCurrent} <span class="v">v{$appUpdate.current}</span></p>
    {/if}
  </div>
</section>

<style>
  section { display: flex; flex-direction: column; gap: 10px; }
  .head { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
  .body { display: flex; flex-direction: column; gap: 12px; }
  p { margin: 0; }
  .kv {
    font-size: 12.5px;
    display: inline-flex;
    flex-wrap: wrap;
    gap: 10px;
    align-items: center;
    color: var(--fg-muted);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }
  .v {
    text-transform: none;
    letter-spacing: normal;
    color: var(--fg);
  }
  .arrow { color: var(--fg-dim); }
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
    white-space: pre-wrap;
  }
</style>

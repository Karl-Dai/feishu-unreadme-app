<script lang="ts">
  import { toasts } from '$lib/stores/toast';
  import { t as i18n } from '$lib/i18n';
</script>

<div class="stack">
  {#each $toasts as t (t.id)}
    <div class="toast {t.kind}">
      <span class="bracket">[</span>
      <span class="label">{t.kind === 'error' ? $i18n.toastErr : $i18n.toastMsg}</span>
      <span class="bracket">]</span>
      <span class="text">{t.text}</span>
    </div>
  {/each}
</div>

<style>
  .stack {
    position: fixed;
    top: 18px; right: 18px;
    z-index: 200;
    display: flex; flex-direction: column;
    gap: 6px;
    pointer-events: none;
  }
  .toast {
    pointer-events: auto;
    padding: 8px 12px;
    background: var(--bg-elev);
    border: 1px solid var(--border);
    border-left: 2px solid var(--fg);
    color: var(--fg);
    font-size: 12px;
    letter-spacing: 0.04em;
    max-width: 380px;
    display: flex; gap: 8px; align-items: baseline;
    box-shadow: 0 0 24px rgba(51, 255, 102, 0.08);
    animation: slide-in 160ms ease-out;
  }
  .toast.error { border-left-color: var(--danger); color: var(--danger); }
  .toast.warn  { border-left-color: var(--warn); color: var(--warn); }
  .bracket { color: var(--fg-muted); }
  .label { font-size: 10.5px; letter-spacing: 0.12em; }
  .text { color: inherit; word-break: break-word; }
  @keyframes slide-in {
    from { opacity: 0; transform: translateX(8px); }
    to   { opacity: 1; transform: translateX(0); }
  }
</style>

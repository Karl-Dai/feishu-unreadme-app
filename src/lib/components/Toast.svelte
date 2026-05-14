<script lang="ts">
  import { toasts } from '$lib/stores/toast';
</script>

<div class="toast-stack">
  {#each $toasts as t (t.id)}
    <div class="toast {t.kind}">{t.text}</div>
  {/each}
</div>

<style>
  .toast-stack {
    position: fixed; top: 16px; right: 16px; z-index: 100;
    display: flex; flex-direction: column; gap: 8px;
  }
  .toast {
    padding: 10px 14px; border-radius: 6px; background: #fff;
    box-shadow: 0 2px 8px rgba(0,0,0,.12); max-width: 360px; font-size: 14px;
  }
  .toast.error { background: #ffeaea; color: #b00020; }
</style>

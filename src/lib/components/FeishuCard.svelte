<script lang="ts">
  import { feishu } from '$lib/stores/feishu';
  import { open } from '@tauri-apps/plugin-dialog';
  import { t } from '$lib/i18n';

  async function pick() {
    const path = await open({ directory: true, multiple: false, title: $t.feishuPickerTitle });
    if (typeof path === 'string') {
      await feishu.pickManually(path);
    }
  }
</script>

<section>
  <div class="head">
    <span class="prompt">feishu/locate</span>
    {#if $feishu.kind === 'loading'}
      <span class="tag tag--muted">{$t.feishuTagScanning}</span>
    {:else if $feishu.kind === 'ok'}
      <span class="tag tag--ok">{$t.feishuTagReady}</span>
    {:else if $feishu.kind === 'not-found'}
      <span class="tag tag--warn">{$t.feishuTagNotFound}</span>
    {:else}
      <span class="tag tag--err">{$t.feishuTagError}</span>
    {/if}
  </div>

  <div class="body">
    {#if $feishu.kind === 'loading'}
      <p class="dim">{$t.feishuScanning}<span class="cursor"></span></p>
    {:else if $feishu.kind === 'ok'}
      <div class="kv">
        <span class="k">{$t.feishuKvPath}</span><span class="v">{$feishu.info.install_path}</span>
        <span class="k">{$t.feishuKvVersion}</span><span class="v">{$feishu.info.version ?? $t.feishuVersionUnknown}</span>
        <span class="k">asar</span><span class="v break">{$feishu.info.asar_path}</span>
      </div>
      <button class="btn" onclick={pick}>{$t.feishuBtnChange}</button>
    {:else if $feishu.kind === 'not-found'}
      <p class="warn">{$t.feishuNotFoundHint}</p>
      <button class="btn" onclick={pick}>{$t.feishuBtnPick}</button>
    {:else}
      <p class="err">→ {$feishu.message}</p>
      <button class="btn" onclick={pick}>{$t.feishuBtnRetry}</button>
    {/if}
  </div>
</section>

<style>
  section { display: flex; flex-direction: column; gap: 10px; }
  .head { display: flex; align-items: center; justify-content: space-between; gap: 12px; }
  .body { display: flex; flex-direction: column; gap: 12px; }
  .kv {
    display: grid;
    grid-template-columns: 90px 1fr;
    row-gap: 4px;
    column-gap: 16px;
    font-size: 12.5px;
  }
  .k { color: var(--fg-muted); text-transform: uppercase; letter-spacing: 0.08em; font-size: 11px; align-self: center; }
  .v { color: var(--fg); word-break: break-all; }
  .v.break { color: var(--fg-dim); }
  p { margin: 0; }
</style>

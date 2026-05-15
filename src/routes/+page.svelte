<script lang="ts">
  import { onMount } from 'svelte';
  import FeishuCard from '$lib/components/FeishuCard.svelte';
  import PatchCard from '$lib/components/PatchCard.svelte';
  import AppUpdateCard from '$lib/components/AppUpdateCard.svelte';
  import LogDrawer from '$lib/components/LogDrawer.svelte';
  import { feishu } from '$lib/stores/feishu';
  import { appUpdate } from '$lib/stores/app_update';
  import { logs } from '$lib/stores/logs';
  import { lang, t } from '$lib/i18n';

  let now = $state(new Date());

  onMount(() => {
    logs.init();
    feishu.detect();
    appUpdate.init();
    const tick = setInterval(() => (now = new Date()), 1000);
    return () => clearInterval(tick);
  });

  const banner = String.raw`
   ┌─┐┌─┐┬┌─┐┬ ┬┬ ┬   ┬ ┬┌┐┌┬─┐┌─┐┌─┐┌┬┐┌┬┐┌─┐
   ├┤ ├┤ │└─┐├─┤│ │───│ ││││├┬┘├┤ ├─┤ │││││├┤
   └  └─┘┴└─┘┴ ┴└─┘   └─┘┘└┘┴└─└─┘┴ ┴─┴┘┴ ┴└─┘`;
</script>

<main>
  <header>
    <pre class="banner">{banner}</pre>
    <p class="subtitle">{$t.subtitle}</p>
    <div class="meta">
      <span class="dim">{$t.session}</span>
      <span>{now.toLocaleString($lang === 'zh' ? 'zh-CN' : 'en-US', { hour12: false })}</span>
      <span class="sep">·</span>
      <span class="dim">{$t.target}</span>
      <span>feishu/lark</span>
      <span class="sep">·</span>
      <span class="dim">{$t.mode}</span>
      <span class="accent">{$t.modeInteractive}</span>
      <span class="sep">·</span>
      <span class="dim">{$t.lang}</span>
      <button class="lang-btn" class:active={$lang === 'zh'} onclick={() => lang.set('zh')}>{$t.langZh}</button>
      <span class="sep-thin">|</span>
      <button class="lang-btn" class:active={$lang === 'en'} onclick={() => lang.set('en')}>{$t.langEn}</button>
      <span class="cursor"></span>
    </div>
    <div class="rule"></div>
  </header>

  <FeishuCard />
  <div class="rule"></div>
  <PatchCard />
  <div class="rule"></div>
  <AppUpdateCard />
</main>

<LogDrawer />

<style>
  main {
    max-width: 880px;
    margin: 0 auto;
    padding: 32px 32px 240px;
    display: flex;
    flex-direction: column;
    gap: 28px;
    animation: flicker 8s linear infinite;
  }
  header { display: flex; flex-direction: column; gap: 14px; }
  .banner {
    margin: 0;
    color: var(--fg);
    line-height: 1.1;
    font-size: 12px;
    white-space: pre;
    text-shadow: 0 0 8px rgba(51, 255, 102, 0.45), 0 0 18px rgba(51, 255, 102, 0.2);
  }
  .subtitle { margin: 0; font-size: 12px; color: var(--fg-dim); letter-spacing: 0.04em; }
  .meta { font-size: 11px; letter-spacing: 0.06em; display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }
  .sep { color: var(--fg-muted); }
  .sep-thin { color: var(--fg-muted); opacity: 0.5; margin: 0 -2px; }
  .lang-btn {
    background: transparent; border: 0; padding: 0; margin: 0;
    cursor: pointer; color: var(--fg-muted); font: inherit;
    letter-spacing: inherit; transition: color 80ms linear;
  }
  .lang-btn:hover { color: var(--fg); }
  .lang-btn.active { color: var(--accent); }
  .rule {
    border-top: 1px dashed var(--border);
    margin: 0;
    opacity: 0.7;
  }
</style>

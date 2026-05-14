<script lang="ts">
  import { onMount } from 'svelte';
  import FeishuCard from '$lib/components/FeishuCard.svelte';
  import PatchCard from '$lib/components/PatchCard.svelte';
  import AppUpdateCard from '$lib/components/AppUpdateCard.svelte';
  import LogDrawer from '$lib/components/LogDrawer.svelte';
  import { feishu } from '$lib/stores/feishu';
  import { appUpdate } from '$lib/stores/app_update';

  onMount(() => {
    feishu.detect();
    appUpdate.init();
  });
</script>

<main>
  <header>
    <h1>feishu-unreadme</h1>
  </header>
  <FeishuCard />
  <PatchCard />
  <AppUpdateCard />
</main>
<LogDrawer />

<style>
  main { max-width: 720px; margin: 0 auto; padding: 24px 16px 64px; display: flex; flex-direction: column; gap: 16px; }
  header h1 { margin: 0 0 16px; font-size: 22px; }
</style>

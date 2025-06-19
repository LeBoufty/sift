<script>
  import Footer from "../props/Footer.svelte";
  let { children } = $props();
  import "/src/style.css";
  import { SETTINGS } from "./settings";
</script>

<div class={SETTINGS.theme} data-sveltekit-preload-data="hover">
  {@render children()}
  <Footer />
</div>

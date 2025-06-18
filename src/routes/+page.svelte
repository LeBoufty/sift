<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  const dummyChecklists = [
    { name: "Checklist #1", entryCount: 54, id: 0 },
    { name: "Checklist #2", entryCount: 2, id: 1 },
    { name: "Checklist #3", entryCount: 4, id: 2 },
    // { name: "Checklist #4", entryCount: 48653, id: 3 },
    // We consider there's a getLast3Checklists command...
  ];
</script>

<main class="m-4 p-4 dark:bg-zinc-800 bg-zinc-100 rounded-2xl">
  <h1 class="text-center text-black dark:text-white font-black text-5xl m-1">
    Sift
  </h1>

  <h2 class="text-center text-zinc-900 dark:text-zinc-100 text-2xl m-2">
    Last checklists
  </h2>

  <div class="grid grid-cols-1 gap-4">
    {#each dummyChecklists as cl}
      <a href="/checklist/{cl.id}">
        <div
          class="mx-auto flex max-w-sm items-center gap-x-4 rounded-xl bg-white p-6 shadow-lg outline outline-black/5 dark:bg-zinc-900 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10"
        >
          <img class="size-12 shrink-0" src="/tauri.svg" alt="Tauri Logo" />
          <div>
            <div class="text-xl font-medium text-black dark:text-white">
              {cl.name}
            </div>
            <p class="text-gray-500 dark:text-gray-400">
              {cl.entryCount} entries
            </p>
          </div>
        </div>
      </a>
    {/each}
    <a href="/checklists">
      <div
        class="mx-auto max-w-sm gap-x-4 rounded-xl bg-zinc-100 p-4 shadow-lg outline outline-black/5 dark:bg-zinc-900 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10"
      >
        <div>
          <div
            class="text-xl font-medium text-black dark:text-white text-center"
          >
            All checklists
          </div>
        </div>
      </div>
    </a>
  </div>
</main>

<style lang="postcss">
  @reference "tailwindcss";
  :root {
  }
</style>

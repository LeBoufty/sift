<script lang="ts">
  import type { ChecklistInfo } from "../../types/checklists";
  import ChecklistList from "../ChecklistList.svelte";
  const dummyChecklists: ChecklistInfo[] = [
    { name: "Checklist #1", entryCount: 54, id: 0 },
    { name: "Checklist #2", entryCount: 2, id: 1 },
    { name: "我爱伟大的最高领", entryCount: 4, id: 2 },
    { name: "Awawawawawawawawawawawawa", entryCount: 48653, id: 3 },
    { name: "Checklist #5", entryCount: 666, id: 4 },
    { name: "Checklist #6", entryCount: 0, id: 5 },
    { name: "Checklist #7", entryCount: 777, id: 6 },
  ];
</script>

<main class="m-4 p-4 dark:bg-zinc-800 bg-zinc-100 rounded-2xl">
  <h1 class="text-center text-black dark:text-white font-black text-4xl m-8">
    Your checklists
  </h1>

  <a href="/create_checklist">
    <div
      class="m-8 mx-auto flex max-w-sm items-center gap-x-4 rounded-xl bg-zinc-100 p-4 shadow-lg outline outline-black/5 dark:bg-zinc-900 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10"
    >
      <div class="text-5xl font-bold text-zinc-900 dark:text-zinc-100">+</div>
      <div class="text-xl font-medium text-black dark:text-white text-center">
        Create checklist
      </div>
    </div>
  </a>

  <ChecklistList checklists={dummyChecklists} />
</main>

<style lang="postcss">
  @reference "tailwindcss";
</style>

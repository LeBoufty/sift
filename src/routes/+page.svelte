<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ChecklistList from "../props/ChecklistList.svelte";
  import AllChecklistsButton from "../props/AllChecklistsButton.svelte";
  import CreateChecklistButton from "../props/CreateChecklistButton.svelte";

  // KEEPING THIS PART TO REMEMBER SYNTAX (i am dumb)
  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
  // -------------------------------

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
    <ChecklistList checklists={dummyChecklists} />
    <AllChecklistsButton />
    <CreateChecklistButton />
  </div>
</main>

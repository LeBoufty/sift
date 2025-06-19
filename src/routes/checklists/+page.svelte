<script lang="ts">
  import type { ChecklistInfo } from "../../types/checklists";
  import ChecklistList from "../../props/ChecklistList.svelte";
  import CreateChecklistButton from "../../props/CreateChecklistButton.svelte";
  import { push_to_history } from "../history";
  import BackAndHistory from "../../props/BackAndHistory.svelte";

  push_to_history({
    name: "Your checklists",
    href: "/checklists",
  });

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
  <BackAndHistory />
  <h1 class="text-center text-black dark:text-white font-black text-4xl m-8">
    Your checklists
  </h1>

  <CreateChecklistButton />

  <ChecklistList checklists={dummyChecklists} />
</main>

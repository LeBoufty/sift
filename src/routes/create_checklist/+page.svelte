<script>
  import BackAndHistory from "../../props/BackAndHistory.svelte";
  import Title from "../../props/Title.svelte";
  import { push_to_history } from "../history";

  push_to_history({
    name: "New checklist",
    href: "/create_checklist",
  });
</script>

<main class="m-4 p-4 dark:bg-zinc-800 bg-zinc-100 rounded-2xl">
  <BackAndHistory />
  <Title text="New checklist" />
</main>

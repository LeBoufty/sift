<script lang="ts">
  import { get_previous_page, pop_history } from "../routes/history";
  const goto = get_previous_page().href;
  const pop = pop_history;
</script>

<a href={goto} onclick={pop}>
  <div
    class="flex bg-zinc-200 dark:bg-zinc-700 p-2 rounded-2xl size-12 items-center"
  >
    <h1
      class="align-middle text-3xl font-black text-zinc-700 dark:text-zinc-300"
    >
      &larr;
    </h1>
  </div>
</a>

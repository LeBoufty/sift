<script lang="ts">
  import {
    get_last_pages,
    get_history_size,
    type HistoryEntry,
  } from "../routes/history";
  let last: HistoryEntry[] = [];
  const size = get_history_size();
  if (size > 3) {
    last = [{ href: "/", name: "..." }, ...get_last_pages(2)];
  } else {
    last = get_last_pages(3);
  }
  const history = last.map((l) => l.name).join(" > ");
</script>

<div class="flex-nowrap text-zinc-500 dark:text-zinc-400">{history}</div>

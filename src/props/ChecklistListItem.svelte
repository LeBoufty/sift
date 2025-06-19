<script lang="ts">
  let { cl } = $props();
</script>

<a href="/checklist/{cl.id}">
  <div
    class="mx-auto flex max-w-sm items-center gap-x-4 rounded-xl bg-white p-6 shadow-lg outline outline-black/5 dark:bg-zinc-900 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10"
  >
    <img class="size-12 shrink-0" src="/logo.png" alt="Sift Logo" />
    <div class="max-w-full text-ellipsis overflow-hidden">
      <div class="text-xl font-medium text-black dark:text-white">
        {cl.name}
      </div>
      <p class="text-gray-500 dark:text-gray-400">
        {cl.entryCount} entries
      </p>
    </div>
  </div>
</a>

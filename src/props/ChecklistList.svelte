<script lang="ts">
  import ChecklistListItem from "./ChecklistListItem.svelte";
  let { checklists } = $props();
</script>

<div class="grid grid-cols-1 gap-4">
  {#each checklists as cl}
    <ChecklistListItem {cl} />
  {/each}
</div>

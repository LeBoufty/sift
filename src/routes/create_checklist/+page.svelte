<script lang="ts">
  import BackAndHistory from "../../props/BackAndHistory.svelte";
  import Title from "../../props/Title.svelte";
  import { push_to_history } from "../history";
  import {
    CHECKBOX,
    default_category,
    default_checklist,
    STARS,
    VALUE,
  } from "../../types/checklists";
  import type {
    Checklist,
    ChecklistCategory,
    CheckTypeType,
  } from "../../types/checklists";

  push_to_history({
    name: "New checklist",
    href: "/create_checklist",
  });

  let checklist: Checklist = $state(
    JSON.parse(
      localStorage.getItem("checklist") || JSON.stringify(default_checklist()),
    ),
  );

  const saveChecklist = () => {
    localStorage.setItem("checklist", JSON.stringify(checklist));
  };

  const add_category = () => {
    const max_id = Math.max(1, ...checklist.categories.map((c) => c.id));
    checklist.categories.push(default_category(max_id + 1));
    saveChecklist();
  };

  const delete_category = (category: ChecklistCategory) => {
    checklist.categories.splice(checklist.categories.indexOf(category), 1);
    saveChecklist();
  };

  function get_icon(checkType: CheckTypeType): string {
    switch (checkType) {
      case STARS:
        return "/check_star_icon.png";
      case CHECKBOX:
        return "/check_box_icon.png";
      case VALUE:
        return "/pencil_icon.png";
    }
  }
</script>

<main class="m-4 p-4 dark:bg-zinc-800 bg-zinc-100 rounded-2xl">
  <BackAndHistory />
  <Title text="New checklist" />
  <div class="justify-center grid grid-cols-1 gap-6 dark:text-white">
    <!-- Checklist name -->
    <div class="flex gap-4 justify-center items-center content-center">
      <div class="text-2xl">Name:</div>
      <input
        type="text"
        name="name"
        class="border rounded-2xl text-xl p-2 dark:bg-zinc-700"
        defaultValue={checklist.name}
      />
    </div>
    <!-- Categories -->
    {#each checklist.categories as category}
      <div
        class="p-4 rounded-xl bg-gray-100 border-zinc-300 dark:bg-zinc-700 dark:border-zinc-600 border-2 shadow-lg"
      >
        <div class="flex justify-around">
          <input
            type="text"
            class="font-bold dark:text-zinc-200 text-zinc-800 border rounded-xl p-1"
            defaultValue={category.name}
          />
          <button
            class="text-white font-bold text-xs rounded-2xl bg-red-900 shadow-sm shadow-red-900 p-2"
            onclick={() => delete_category(category)}>Delete</button
          >
        </div>
        <hr class="m-2 text-zinc-500" />
        <div class="grid grid-cols-1">
          <table class="table-fixed text-center">
            <thead>
              <tr>
                <th> Name </th>
                <th> Type </th>
                <th> Weight </th>
                <th> Edit </th>
              </tr>
            </thead>
            <tbody>
              {#each category.checks as check}
                <tr>
                  <td>{check.name}</td>
                  <td class="flex justify-center"
                    ><img
                      class="size-7"
                      src={get_icon(check.ctype.type)}
                      alt={check.ctype.type}
                    /></td
                  >
                  <td>{check.weight}</td>
                  <td>AAAAA</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/each}
    <button
      onclick={add_category}
      class="p-1 text-green-950 rounded-xl bg-green-100 border-green-950 dark:border-green-300 border-2 shadow-lg text-xl"
      >+ Add category</button
    >
  </div>
</main>

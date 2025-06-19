<script lang="ts">
  import BackAndHistory from "../../props/BackAndHistory.svelte";
  import Title from "../../props/Title.svelte";
  import { push_to_history } from "../history";
  import { SETTINGS, GRADING_FORMATS } from "../settings";

  push_to_history({
    name: "Settings",
    href: "/settings",
  });

  const switch_theme = () => {
    SETTINGS.update((settings) => {
      const newTheme = settings.theme === "dark" ? "light" : "dark";
      console.log(newTheme); // Debug log
      return { ...settings, theme: newTheme };
    });
  };

  let grading: string = $SETTINGS.grades;

  const switch_grading = () => {
    let index = GRADING_FORMATS.indexOf(grading)!;
    index = index + 1 < GRADING_FORMATS.length ? index + 1 : 0;
    SETTINGS.update((settings) => {
      grading = GRADING_FORMATS[index];
      return { ...settings, grades: grading };
    });
  };
</script>

<main class="m-4 p-4 dark:bg-zinc-800 bg-zinc-100 rounded-2xl">
  <BackAndHistory />
  <Title text="Settings" />
  <div class="justify-center grid grid-cols-1">
    <button
      type="button"
      class="max-w-full bg-orange-300 shadow-lg shadow-orange-300/50 p-4 m-4 rounded-4xl dark:bg-sky-800 dark:shadow-sky-800 font-bold dark:text-sky-200 text-orange-950"
      on:click={switch_theme}
    >
      Change theme
    </button>
    <button
      type="button"
      class="max-w-full bg-zinc-200 shadow-lg p-4 m-4 rounded-4xl dark:bg-zinc-600 font-bold dark:text-white"
      on:click={switch_grading}
    >
      Grading system: {grading}
    </button>
  </div>
</main>

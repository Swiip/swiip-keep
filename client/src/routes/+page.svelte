<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Todos from "../components/todos.svelte";

    let name = "";
    let greetMsg = "";

    const greet = async () => {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        greetMsg = await invoke("greet", { name });
    };
</script>

<div class="container">
    <form class="row" on:submit|preventDefault={greet}>
        <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
        <button type="submit">Greet</button>
    </form>

    <p>{greetMsg}</p>

    <Todos />
</div>

<script lang="ts">
    import "todomvc-app-css/index.css";
    import { onMount } from "svelte";
    import type { ChangeEventHandler, FocusEventHandler } from "svelte/elements";
    import type { KeyboardEventHandler } from "svelte/elements";
    import { invoke } from "@tauri-apps/api/core";

    type Todo = {
        id: string;
        text: string;
        completed: boolean;
    };

    const ENTER_KEY = 13;
    const ESCAPE_KEY = 27;

    let currentFilter = $state<"all" | "active" | "completed">("all");
    let items = $state<Todo[]>([]);
    let editing = $state<number | undefined>(undefined);
    let init = $state(true);

    const active = (item: Todo) => !item.completed;
    const completed = (item: Todo) => item.completed;

    const filtered = $derived(
        currentFilter === "all" ? items : items.filter(currentFilter === "completed" ? completed : active),
    );

    const numActive = $derived(items.filter(active).length);

    const numCompleted = $derived(items.filter(completed).length);

    $effect(() => {
        const save = async () => {
            if (init) {
                return;
            }
            console.log("save start");
            const result = await invoke<string>("save_todos", { todos: items });
            console.log("save result", result);
        };

        void save();
    });

    const load = async () => {
        updateView();
        items = await invoke<Todo[]>("load_todos");
        console.log("loading result", items);
        init = false;
    };

    const updateView = () => {
        currentFilter = "all";
        if (location.hash === "#/active") {
            currentFilter = "active";
        } else if (location.hash === "#/completed") {
            currentFilter = "completed";
        }
    };

    const clearCompleted = () => {
        items = items.filter(active);
    };

    const remove = (index: number) => {
        items = items.slice(0, index).concat(items.slice(index + 1));
    };

    const toggleAll: ChangeEventHandler<HTMLInputElement> = (event) => {
        const target = event.target as HTMLInputElement;
        items = items.map((item) => ({
            id: item.id,
            text: item.text,
            completed: target.checked,
        }));
    };

    const createNew: KeyboardEventHandler<HTMLInputElement> = (event) => {
        if (event.which === ENTER_KEY) {
            const target = event.target as HTMLInputElement;
            items = items.concat({
                id: crypto.randomUUID(),
                text: target.value,
                completed: false,
            });
            target.value = "";
        }
    };

    const handleEdit: KeyboardEventHandler<HTMLInputElement> = (event) => {
        const target = event.target as HTMLInputElement;
        if (event.which === ENTER_KEY) target.blur();
        else if (event.which === ESCAPE_KEY) editing = undefined;
    };

    const submit: FocusEventHandler<HTMLInputElement> = (event) => {
        const target = event.target as HTMLInputElement;
        items[editing as number].text = target.value;
        editing = undefined;
    };

    onMount(load);
</script>

<svelte:window on:hashchange={updateView} />

<header class="header">
    <h1>TodoMVC</h1>
    <h2>Svelte 5 / Tauri 2 / Vercel Rust</h2>
    <!-- svelte-ignore a11y-autofocus -->
    <input class="new-todo" on:keydown={createNew} placeholder="What needs to be done?" autofocus />
</header>

{#if items.length > 0}
    <section class="main">
        <input
            id="toggle-all"
            class="toggle-all"
            type="checkbox"
            on:change={toggleAll}
            checked={numCompleted === items.length}
        />
        <label for="toggle-all">Mark all as complete</label>

        <ul class="todo-list">
            {#each filtered as item, index (item.id)}
                <li class:completed={item.completed} class:editing={editing === index}>
                    <div class="view">
                        <input class="toggle" type="checkbox" bind:checked={item.completed} />
                        <!-- svelte-ignore a11y-label-has-associated-control -->
                        <label on:dblclick={() => (editing = index)}>{item.text}</label>
                        <button on:click={() => remove(index)} class="destroy"></button>
                    </div>

                    {#if editing === index}
                        <!-- svelte-ignore a11y-autofocus -->
                        <input
                            value={item.text}
                            id="edit"
                            class="edit"
                            on:keydown={handleEdit}
                            on:blur={submit}
                            autofocus
                        />
                    {/if}
                </li>
            {/each}
        </ul>

        <footer class="footer">
            <span class="todo-count">
                <strong>{numActive}</strong>
                {numActive === 1 ? "item" : "items"} left
            </span>

            <ul class="filters">
                <li>
                    <a class:selected={currentFilter === "all"} href="#/">All</a>
                </li>
                <li>
                    <a class:selected={currentFilter === "active"} href="#/active">Active</a>
                </li>
                <li>
                    <a class:selected={currentFilter === "completed"} href="#/completed">Completed</a>
                </li>
            </ul>

            {#if numCompleted}
                <button class="clear-completed" on:click={clearCompleted}>Clear completed</button>
            {/if}
        </footer>
    </section>
{/if}

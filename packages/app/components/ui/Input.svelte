<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    let username: string = '';
    const dispatch = createEventDispatcher();

    const handleInput = () => {
        if (isValidUsername(username)) {
            dispatch('valid', username);
        } else {
            dispatch('invalid', 'Invalid username');
        }
    };

    function isValidUsername(value: string): boolean {
        return /^[a-zA-Z0-9_]{3,20}$/.test(value);
    }
</script>

<div class="flex flex-col items-start w-full max-w-sm">
    <label for="username" class="mb-1 text-sm font-semibold text-gray-700">
        Username
    </label>
    <input
            id="username"
            type="text"
            bind:value={username}
            on:input={handleInput}
            placeholder="Enter your username"
            aria-describedby="usernameHelp"
            class="w-full px-4 py-2 text-gray-900 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500"
    />
    <p id="usernameHelp" class="mt-1 text-xs text-gray-500">
        Use 3-20 characters. Letters, numbers, and underscores only.
    </p>
</div>

<style>
    /* Add any extra styles or responsiveness as needed */
</style>

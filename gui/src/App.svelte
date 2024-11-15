<script lang="ts">
    import '@fontsource-variable/manrope';
    import Create from './partials/Create.svelte';
    import Process from './partials/Process.svelte';
    import Solution from './partials/Solution.svelte';
    import { default as ErrorPreview } from './partials/Error.svelte';
    import { writable } from 'svelte/store';

    const DOMAIN = import.meta.env.VITE_SERVER_DOMAIN;

    const StateEnum = {
        Creating: 0,
        Processing: 1,
        Result: 2,
        Error: 3,
    };

    let currentSolution = writable({});
    let processingReason = writable('Checking server status...');
    let processingError = writable('');
    let processingAllowRetry = writable(false);
    let currentState = writable(StateEnum.Processing);

    const setProcessing = (reason: string, allowRetry = false) => {
        processingReason.set(reason);
        currentState.set(StateEnum.Processing);
        processingAllowRetry.set(allowRetry);
    };

    const setProcessingError = (error: string) => {
        processingError.set(error);
        currentState.set(StateEnum.Error);
    };

    const markAsUnavailable = () => {
        setProcessingError('Server is not available');
    };

    const loadSolution = async () => {
        setProcessing('Waiting for solution...');

        try {
            const { solution } = await fetch(`${DOMAIN}/solution`)
                .then(r => r.ok ? r.json() : Promise.reject(r));

            if (!solution) {
                setProcessingError('No solution found');
                return;
            }

            currentSolution.set(solution);
            currentState.set(StateEnum.Result);
        } catch (error) {
            markAsUnavailable();
        }
    };

    const reload = () => {
        loadInit();
    };

    const loadInit = async () => {
        if (typeof DOMAIN !== 'string') {
            setProcessingError('Server domain is not set, please check the configuration and compile the app again');
            return;
        }

        setProcessing('Checking server status...');

        try {
            const { is_working, current_solution } = await fetch(`${DOMAIN}/is_working`)
                .then(r => r.ok ? r.json() : Promise.reject(r));

            if (is_working) {
                loadSolution();
                return;
            }

            if (current_solution) {
                currentSolution.set(current_solution);
                currentState.set(StateEnum.Result);
                return;
            }

            currentState.set(StateEnum.Creating);
        } catch (error) {
            markAsUnavailable();
        }
    };

    const submitStart = async (jsonBody = null) => {
        setProcessing('Submitting request...');

        try {
            await fetch(`${DOMAIN}/start`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(jsonBody, null, 4),
            });

            loadSolution();
        } catch (error) {
            markAsUnavailable();
        }
    };

    const clearSolution = () => {
        currentSolution.set({});
        currentState.set(StateEnum.Creating);
    };

    loadInit();
</script>

<main>
    <div class="logo">
        Unfox
    </div>
    <div class="content">
        {#if $currentState === StateEnum.Creating}
            <Create
                onSubmit={submitStart}
            />
        {:else if $currentState === StateEnum.Processing}
            <Process
                text={$processingReason}
                allowRetry={$processingAllowRetry}
                onRetry={reload}
            />
        {:else if $currentState === StateEnum.Error}
            <ErrorPreview
                error={$processingError}
                onRetry={reload}
            />
        {:else if $currentState === StateEnum.Result}
            <Solution
                solution={$currentSolution}
                onCreateNew={clearSolution}
            />
        {/if}
    </div>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        width: 100%;
        background-color: #2c2c2c;
        border: 2px solid #3b3b3b;
        gap: 1rem;
    }

    .logo {
        font-size: 3rem;
        color: #fff;
    }

    .content {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
    }
</style>

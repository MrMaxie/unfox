<script lang="ts">
    import { IconAlertOctagon } from '@tabler/icons-svelte';
    import Button from '../lib/Button.svelte';

    type Props = {
        error: string;
        onRetry: () => void;
    };

    const { error, onRetry }: Props = $props();
</script>

<div class="error">
    <div class="error__icon">
        <IconAlertOctagon stroke={2} size={128} />
    </div>
    <div class="error__message">
        {error}
    </div>
    <Button onClick={onRetry}>
        Retry
    </Button>
</div>

<style>
    .error {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        gap: 1rem;
    }

    .error__icon {
        color: #eb2c2c;
    }
</style>

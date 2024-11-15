<script lang="ts">
    import Button from '../lib/Button.svelte';

    type Props = {
        text: string;
        allowRetry: boolean;
        onRetry: () => void;
    };

    let {
        text,
        allowRetry,
        onRetry,
    }: Props = $props();
</script>

<div class="loader-container">
    <div class="loader"></div>
    <div class="loader-text">{text}</div>
    <Button
        isDisabled={!allowRetry}
        isInvisible={!allowRetry}
        onClick={onRetry}
    >
        Retry
    </Button>
</div>

<style>
    .loader-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 2rem;
    }
    .loader {
        width: 40px;
        aspect-ratio: 1;
        position: relative;
    }
    .loader:before,
    .loader:after {
        content: "";
        position: absolute;
        top: 0;
        left: 0;
        margin: -8px 0 0 -8px;
        width: 16px;
        aspect-ratio: 1;
        background: #FABC3F;
        animation:
            l2-1 2s infinite,
            l2-2 1s infinite;
        box-shadow: 0 0 10px 1px #FABC3F;
    }
    .loader:after {
        background:#E85C0D;
        animation-delay: -1s,0s;
        box-shadow: 0 0 10px 1px #E85C0D;
    }
    @keyframes l2-1 {
        0%   {top:0   ;left:0}
        25%  {top:100%;left:0}
        50%  {top:100%;left:100%}
        75%  {top:0   ;left:100%}
        100% {top:0   ;left:0}
    }
    @keyframes l2-2 {
        40%,50% {transform: rotate(0.25turn) scale(0.5)}
        100%    {transform: rotate(0.5turn) scale(1)}
    }
</style>
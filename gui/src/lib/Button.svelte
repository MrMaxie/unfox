<script lang="ts">
    type Props = {
        children?: any;
        isDisabled?: boolean;
        isInvisible?: boolean;
        isInactive?: boolean;
        onClick?: () => void;
    };

    let {
        children,
        isDisabled = false,
        isInvisible = false,
        isInactive = false,
        onClick = () => {},
    }: Props = $props();
</script>

<button
    class="button"
    disabled={isDisabled}
    class:is-invisible={isInvisible}
    class:is-inactive={isInactive}
    onclick={onClick}>
    {@render children()}
</button>

<style>
    .button {
        padding: 0.4rem .8rem;
        border: 0 solid transparent;
        border-radius: 0.3rem;
        background-color: #FABC3F;
        color: #fff;
        font-size: .8rem;
        cursor: pointer;
        transition: background-color 0.3s, opacity .3s;
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        font-family: 'Manrope Variable', sans-serif;
        height: 37px;
    }
    .button.is-inactive {
        background-color: #3b3b3b;
    }
    .button:hover {
        background-color: #df9e1b;
    }
    .button:disabled {
        background-color: #3b3b3b;
        border-color: #3b3b3b;
        cursor: not-allowed;
    }
    .button.is-invisible {
        opacity: 0;
    }
</style>
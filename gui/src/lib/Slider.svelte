<script lang="ts">
    type Props = {
        value: number;
        stepsLabels: string[];
        onChange: (value: number) => void;
    };

    let { value, stepsLabels, onChange }: Props = $props();
</script>

<div class="slider-container">
    <div class="slider__labels">
        {#each stepsLabels as label, index}
            <div
                class="slider__label"
                class:is-active={index === value}
            >
                {label}
            </div>
        {/each}
    </div>
    <input
        type="range"
        class="slider"
        min={0}
        max={stepsLabels.length - 1}
        step={1}
        bind:value={value}
        oninput={() => onChange(value)}
    />
</div>

<style>
    .slider-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        justify-content: center;
        gap: .5rem;
    }

    .slider__labels {
        display: flex;
        position: relative;
    }

    .slider__label {
        font-size: 0.8rem;
        color: #878787;
        text-align: left;
        transition: color .2s, text-shadow .2s;
        flex: 1;
    }

    .slider__label:last-child {
        position: absolute;
        right: 0;
    }

    .slider__label.is-active {
        color: #FABC3F;
        text-shadow: 0 0 5px #FABC3F;
    }

    .slider {
        -webkit-appearance: none;
        appearance: none;
        width: 100%;
        height: 25px;
        background: transparent;
        outline: none;
        opacity: 0.7;
        -webkit-transition: .2s;
        transition: opacity .2s;
        position: relative;
    }

    .slider::before {
        content: '';
        position: absolute;
        top: 50%;
        left: 0;
        transform: translateY(-50%);
        width: 100%;
        height: 2px;
        background-color: #FABC3F;
        z-index: 2;
    }

    .slider:hover {
        opacity: 1;
    }

    .slider::-webkit-slider-thumb {
        -webkit-appearance: none;
        position: relative;
        appearance: none;
        width: 25px;
        height: 25px;
        cursor: pointer;
        border-radius: 100%;
        background-color: #2c2c2c;
        border: 3px solid #FABC3F;
        transition: border-width .2s;
        z-index: 5;
    }

    .slider::-webkit-slider-thumb:hover {
        border-width: 5px;
    }

    .slider::-moz-range-thumb {
        width: 25px;
        height: 25px;
        background: #FABC3F;
        cursor: pointer;
    }
</style>
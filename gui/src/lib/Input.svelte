<script lang="ts">
    import { normalizeNumber } from './normalizeNumber';

    type Props = ({
        type?: 'text';
        value: string;
        safeRange?: [number, number];
        onChange: (value: string) => void;
    } | {
        type: 'number';
        value: number;
        safeRange?: [number, number];
        onChange: (value: number) => void;
    });

    let {
        type = 'text',
        value,
        onChange,
        safeRange = [0, 100],
    }: Props = $props();

    let inputRef: HTMLInputElement;

    const handleChange = (event: any) => {
        // TypeScript cannot properly infer the type of the value, sad...

        if (type === 'number') {
            onChange(normalizeNumber(event.target.value, safeRange) as unknown as never);
            return;
        }

        onChange(event.target.value as unknown as never);
    };

    const handleBlur = () => {
        let value = inputRef.value;

        if (type === 'number') {
            value = normalizeNumber(value, safeRange).toString();
        }

        inputRef.value = value;
        onChange(value as unknown as never);
    };
</script>

<input
    class="input"
    bind:this={inputRef}
    bind:value={value}
    onblur={handleBlur}
    onchange={handleChange}
/>

<style>
    .input {
        border: 1px solid #3b3b3b;
        background-color: #2c2c2c;
        border-radius: 0.3rem;
        font-family: 'Manrope Variable', sans-serif;
        font-size: 0.8rem;
        padding: 0.4rem .8rem;
        height: 37px;
        outline: none;
        transition: border-color 0.3s;
    }

    .input:focus {
        border-color: #FABC3F;
    }
</style>
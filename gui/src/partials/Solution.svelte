<script lang="ts">
    import Button from '../lib/Button.svelte';
    import { Direction } from '../lib/constants';
    import { SolutionBoard } from '../lib/SolutionBoard';
  import Slider from '../lib/Slider.svelte';

    const board = new SolutionBoard();

    type Props = {
        solution: any;
        onCreateNew: () => void;
    };

    let { solution, onCreateNew }: Props = $props();

    board.fromRaw(solution);

    const createNew = () => {
        onCreateNew();
    };
</script>

<div class="dashboard">
    <div
        class="grid"
        style="grid-template-columns: repeat({$board._.width}, 1fr); grid-template-rows: repeat({$board._.height}, 1fr)"
    >
        {#each $board._.getAllTiles() as tile}
            <div
                class="cell"
                role="button"
                tabindex="0"
                class:is-odd-row={tile.y % 2 === 1}
                class:is-odd-column={tile.x % 2 === 1}
                class:is-moving={tile.isMoving}
                onkeypress={() => {}}
            >
                {#if tile.isGoal}
                    <div class="goal"></div>
                {/if}
                {#if tile.isWall}
                    <div class="wall"></div>
                {/if}
                <div
                    class="cell-content"
                    class:movement-up={tile.movement === 'up'}
                    class:movement-down={tile.movement === 'down'}
                    class:movement-left={tile.movement === 'left'}
                    class:movement-right={tile.movement === 'right'}
                >
                    {tile.pawnIcon}
                </div>
                <div class="debug-info">
                    {tile.x};{tile.y}
                </div>
                {#if tile.isEdgeActive(Direction.Right)}
                    <div
                        class="edge edge--lr"
                        class:is-active={true}
                    >
                        <div class="edge__edge-content"></div>
                    </div>
                {/if}
                {#if tile.isEdgeActive(Direction.Down)}
                    <div
                        class="edge edge--ud"
                        class:is-active={true}
                    >
                        <div class="edge__edge-content"></div>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
    <div class="actions">
        <div class="action-bar action-bar--steps">
            <Slider stepsLabels={$board._.getStepsNamesList()} value={$board._.activeStepIndex} onChange={$board._.setActiveStep} />
        </div>
        <div class="action-bar">
            <div class="action-bar__title">Actions</div>
            <div class="action-bar__content">
                <Button onClick={createNew}>Create New</Button>
            </div>
        </div>
    </div>
</div>

<style>
    .dashboard {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }

    .grid {
        flex: 1;
        display: grid;
        grid-template-columns: repeat(8, 1fr);
        grid-template-rows: repeat(8, 1fr);
    }

    .grid .cell {
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 2rem;
        user-select: none;
        outline: 1px solid transparent;
        transition: outline .3s;
        cursor: default;
        background-color: rgba(255, 255, 255, .01);
        position: relative;
    }

    .grid .cell .cell-content {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 7;
    }

    .grid .cell .edge {
        position: absolute;
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 100;
    }

    .grid .cell .edge--lr {
        width: 100%;
        height: 50%;
        width: 30px;
        top: 50%;
        left: 100%;
        transform: translate(-50%, -50%);
    }

    .grid .cell .edge--ud {
        height: 100%;
        width: 50%;
        height: 30px;
        top: 100%;
        left: 50%;
        transform: translate(-50%, -50%);
    }

    .grid .cell .edge__edge-content {
        background-color: #2c2c2c;
        border: 1px solid #FABC3F;
        content: '';
        width: 50%;
        height: 50%;
        transition: opacity .3s, border .3s, width .3s, height .3s;
        opacity: 1;
    }

    .grid .cell .debug-info {
        position: absolute;
        top: 1rem;
        left: 1rem;
        font-size: .8rem;
        color: rgba(255, 255, 255, .25);
        padding: 2px;
    }

    .grid .cell .goal {
        z-index: 0;
        position: absolute;
        top: 10px;
        left: 10px;
        right: 10px;
        bottom: 10px;
        border-radius: 10px;
        background: url('/public/goalFlag.svg') no-repeat center center;
        background-size: cover;
        opacity: .05;
    }

    .grid .cell .wall {
        z-index: 0;
        position: absolute;
        top: 10px;
        left: 10px;
        right: 10px;
        bottom: 10px;
        background-color: #171717;
        opacity: .3;
        border: 3px solid #000000;
    }

    .grid .cell.is-odd-row, .grid .cell.is-odd-column {
        background-color: rgba(255, 255, 255, .02);
    }

    .grid .cell.cell.is-odd-column.cell.is-odd-row {
        background-color: rgba(255, 255, 255, .03);
    }

    .grid .cell.is-moving {
        outline: 1px solid #FABC3F;
    }

    @keyframes movement-up {
        0% {
            transform: translateY(0);
        }
        50% {
            transform: translateY(-20px);
        }
        100% {
            transform: translateY(0);
        }
    }

    @keyframes movement-down {
        0% {
            transform: translateY(0);
        }
        50% {
            transform: translateY(20px);
        }
        100% {
            transform: translateY(0);
        }
    }

    @keyframes movement-left {
        0% {
            transform: translateX(0);
        }
        50% {
            transform: translateX(-20px);
        }
        100% {
            transform: translateX(0);
        }
    }

    @keyframes movement-right {
        0% {
            transform: translateX(0);
        }
        50% {
            transform: translateX(20px);
        }
        100% {
            transform: translateX(0);
        }
    }

    @keyframes arrow-fading {
        0% {
            opacity: .3;
        }
        50% {
            opacity: 1;
        }
        100% {
            opacity: .3;
        }
    }

    .movement-up::before,
    .movement-down::before,
    .movement-left::before,
    .movement-right::before {
        font-size: 3rem;
        position: absolute;
        top: 0;
        right: 0;
        left: 0;
        bottom: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: -1;
        color: #FABC3F;
    }

    .movement-up::before {
        content: '↑';
        animation: arrow-fading 2s infinite ease-in-out, movement-up 2s infinite ease-in-out;
    }

    .movement-down::before {
        content: '↓';
        animation: arrow-fading 2s infinite ease-in-out, movement-down 2s infinite ease-in-out;
    }

    .movement-left::before {
        content: '←';
        animation: arrow-fading 2s infinite ease-in-out, movement-left 2s infinite ease-in-out;
    }

    .movement-right::before {
        content: '→';
        animation: arrow-fading 2s infinite ease-in-out, movement-right 2s infinite ease-in-out;
    }

    .actions {
        display: flex;
        flex-direction: row;
        padding: 1rem;
        gap: 1rem;
    }

    .action-bar__title {
        text-align: left;
        font-size: .9rem;
    }

    .action-bar {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .action-bar--steps {
        flex: 1;
    }

    .action-bar__content {
        display: flex;
        flex-direction: row;
        gap: 0.5rem;
    }
</style>
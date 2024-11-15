<script lang="ts">
    import { IconChessKnight, IconChessKnightFilled, IconSitemapFilled, IconSitemap } from '@tabler/icons-svelte';
    import Button from '../lib/Button.svelte';
    import { writable } from 'svelte/store';
    import Input from '../lib/Input.svelte';
    import { Direction } from '../lib/constants';
    import { CreatorBoard, MAX_BOARD_SIZE, MIN_BOARD_SIZE } from '../lib/CreatorBoard';

    const board = new CreatorBoard();

    type Props = {
        onSubmit: (data: any) => void;
    };

    const Layers = {
        Pawns: 'pawns',
        Nodes: 'nodes',
    };

    const layer = writable(Layers.Pawns);

    let { onSubmit }: Props = $props();

    const submit = () => {
        onSubmit(board.toRaw());
    };

    const setLayer = (newLayer: string) => {
        layer.set(newLayer);
    };

    const clear = () => {
        $board._.clear();
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
                onclick={() => {
                    if ($layer === Layers.Pawns) {
                        tile.rotatePawn();
                    } else if ($layer === Layers.Nodes) {
                        tile.rotateType();
                    }
                }}
                class:is-odd-row={tile.y % 2 === 1}
                class:is-odd-column={tile.x % 2 === 1}
                onkeypress={() => {}}
            >
                {tile.pawnIcon}
                {tile.typeIcon}
                <div class="debug-info">
                    {tile.x};{tile.y}
                </div>
                {#if tile.isEdgePossible(Direction.Right)}
                    <div
                        class="edge edge--lr"
                        class:is-active={tile.isEdgeActive(Direction.Right)}
                        onclick={e => {
                            e.stopPropagation();
                            tile.toggleEdge(Direction.Right);
                        }}
                        onkeypress={() => {}}
                        tabindex="0"
                        role="button"
                    >
                        <div class="edge__edge-content"></div>
                    </div>
                {/if}
                {#if tile.isEdgePossible(Direction.Down)}
                    <div
                        class="edge edge--ud"
                        class:is-active={tile.isEdgeActive(Direction.Down)}
                        onclick={e => {
                            e.stopPropagation();
                            tile.toggleEdge(Direction.Down);
                        }}
                        onkeypress={() => {}}
                        tabindex="0"
                        role="button"
                    >
                        <div class="edge__edge-content"></div>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
    <div class="actions">
        <div class="action-bar">
            <div class="action-bar__title">Layers</div>
            <div class="action-bar__content">
                <Button onClick={() => setLayer(Layers.Pawns)} isInactive={$layer !== Layers.Pawns}>
                    {#if $layer === Layers.Pawns}
                        <IconChessKnightFilled stroke={1} />
                    {:else}
                        <IconChessKnight stroke={1} />
                    {/if}
                    Pawns
                </Button>
                <Button onClick={() => setLayer(Layers.Nodes)} isInactive={$layer !== Layers.Nodes}>
                    {#if $layer === Layers.Nodes}
                        <IconSitemapFilled stroke={1} />
                    {:else}
                        <IconSitemap stroke={1} />
                    {/if}
                    Nodes
                </Button>
            </div>
        </div>
        <div class="action-bar">
            <div class="action-bar__title">Size</div>
            <div class="action-bar__content">
                <Input
                    type="number"
                    value={$board._.width}
                    onChange={v => $board._.setWidth(v)}
                    safeRange={[MIN_BOARD_SIZE, MAX_BOARD_SIZE]}
                />
                &times;
                <Input
                    type="number"
                    value={$board._.height}
                    onChange={v => $board._.setHeight(v)}
                    safeRange={[MIN_BOARD_SIZE, MAX_BOARD_SIZE]}
                />
            </div>
        </div>
        <div class="spacer"></div>
        <div class="action-bar">
            <div class="action-bar__title">Actions</div>
            <div class="action-bar__content">
                <Button onClick={clear}>Clear</Button>
                <Button onClick={submit}>Submit</Button>
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
        cursor: pointer;
        background-color: rgba(255, 255, 255, .01);
        position: relative;
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
        border: 1px solid rgba(0, 0, 0, .1);
        content: '';
        width: 50%;
        height: 50%;
        transition: opacity .3s, border .3s, width .3s, height .3s;
        opacity: 0;
    }

    .grid .cell .edge.is-active .edge__edge-content {
        border: 1px solid #FABC3F;
        opacity: 1;
    }

    .grid .cell .edge:hover .edge__edge-content {
        border: 1px solid #FABC3F;
        width: 75%;
        height: 75%;
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

    .grid .cell.is-odd-row, .grid .cell.is-odd-column {
        background-color: rgba(255, 255, 255, .02);
    }

    .grid .cell.cell.is-odd-column.cell.is-odd-row {
        background-color: rgba(255, 255, 255, .03);
    }

    .grid .cell:hover {
        outline: 1px solid #FABC3F;
    }

    .actions {
        display: flex;
        flex-direction: row;
        padding: 1rem;
        gap: 1rem;
    }

    .actions .spacer {
        flex: 1;
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

    .action-bar__content {
        display: flex;
        flex-direction: row;
        gap: 0.5rem;
    }
</style>
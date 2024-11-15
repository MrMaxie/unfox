import type { Direction } from './constants';
import { SolutionTile, type SolutionTileRaw } from './SolutionTile';

export type SolutionStepRaw = {
    board: {
        tiles: Array<SolutionTileRaw>;
    };
    pawn_at_move_to: [number, number, Direction] | null;
};

export class SolutionStep {
    tiles: Array<SolutionTile> = [];

    fromRaw = (raw: SolutionStepRaw) => {
        const move = raw.pawn_at_move_to;

        this.tiles = raw.board.tiles.map((tileRaw) => {
            const tile = new SolutionTile(tileRaw.x, tileRaw.y);
            tile.tileType = tileRaw.tile_type;
            tile.pawn = tileRaw.pawn;
            tile.edges = tileRaw.edges;

            if (move && tile.x === move[0] && tile.y === move[1]) {
                tile.movingDirection = move[2];
            }

            return tile;
        });
    };

    fulfillMissingTiles = (width: number, height: number) => {
        const tiles = this.tiles;

        for (let x = 0; x < width; x++) {
            for (let y = 0; y < height; y++) {
                if (!tiles.some(tile => tile.x === x && tile.y === y)) {
                    tiles.push(new SolutionTile(x, y));
                }
            }
        }
    };

    getTile = (x: number, y: number) => {
        return this.tiles.find(tile => tile.x === x && tile.y === y);
    };
}
import { Direction, MovementIcon, PawnIcon, TileType, TileTypeIcon, type Pawn } from './constants';

export type SolutionTileRaw = {
    tile_type: TileType;
    pawn: Pawn | null;
    edges: number;
    x: number;
    y: number;
};

export class SolutionTile {
    tileType: TileType = TileType.Wall;
    pawn: Pawn | null = null;
    edges: number = 0;
    movingDirection: Direction | null = null;

    constructor(
        public x: number,
        public y: number,
    ) {}

    get pawnIcon() {
        return PawnIcon[this.pawn ?? ''] ?? '';
    };

    get movement() {
        switch (this.movingDirection) {
            case Direction.Up:
                return 'up';
            case Direction.Down:
                return 'down';
            case Direction.Left:
                return 'left';
            case Direction.Right:
                return 'right';
            default:
                return '';
        }
    };

    get isMoving() {
        return this.movingDirection !== null;
    };

    get isGoal() {
        return this.tileType === TileType.Goal;
    }

    get isWall() {
        return this.tileType === TileType.Wall;
    }

    isEdgeActive = (dir: Direction) => {
        return !!(this.edges & dir);
    };

    fromRaw = (raw: SolutionTileRaw) => {
        this.tileType = raw.tile_type;
        this.pawn = raw.pawn;
        this.edges = raw.edges;
    };
}
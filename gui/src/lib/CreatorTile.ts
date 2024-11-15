import {
    Direction,
    Pawn,
    PawnIcon,
    TileType,
    TileTypeIcon,
} from './constants';
import {
    type CreatorBoard
} from './CreatorBoard';

export type CreatorTileRaw = {
    tile_type: TileType;
    pawn: Pawn | null;
    edges: number;
    x: number;
    y: number;
};

export class CreatorTile {
    tileType: TileType = TileType.Wall;
    pawn: Pawn | null = null;
    edges: number = 0;

    constructor(
        private board: CreatorBoard,
        public x: number,
        public y: number,
    ) {}

    rotatePawn = () => {
        if (this.pawn === Pawn.Fox) {
            this.pawn = Pawn.Monster;
        } else if (this.pawn === Pawn.Monster) {
            this.pawn = null;
        } else {
            this.pawn = Pawn.Fox;
        }

        this.board.notifyChange();
    };

    rotateType = () => {
        if (this.tileType === TileType.Empty) {
            this.tileType = TileType.Goal;
        } else if (this.tileType === TileType.Goal) {
            this.tileType = TileType.Wall;
        } else {
            this.tileType = TileType.Empty;
        }

        this.board.notifyChange();
    };

    get pawnIcon() {
        return PawnIcon[this.pawn ?? ''] ?? '';
    };

    get typeIcon() {
        return TileTypeIcon[this.tileType] ?? '';
    };

    isEdgeActive = (dir: Direction) => {
        return !!(this.edges & dir);
    };

    toggleEdge = (dir: Direction) => {
        this.edges ^= dir;

        // sibling tile:
        const [sx, sy, sdir] = (() => {
            switch (dir) {
                case Direction.Up:
                    return [this.x, this.y - 1, Direction.Down];
                case Direction.Down:
                    return [this.x, this.y + 1, Direction.Up];
                case Direction.Left:
                    return [this.x - 1, this.y, Direction.Right];
                case Direction.Right:
                    return [this.x + 1, this.y, Direction.Left];
            }
        })();

        const sTile = this.board.getTile(sx, sy);

        if (sTile) {
            sTile.edges ^= sdir;
        }

        this.board.notifyChange();
    }

    isEdgePossible = (dir: Direction) => {
        if (this.tileType === TileType.Wall) {
            return false;
        }

        const [sx, sy] = (() => {
            switch (dir) {
                case Direction.Up:
                    return [this.x, this.y - 1];
                case Direction.Down:
                    return [this.x, this.y + 1];
                case Direction.Left:
                    return [this.x - 1, this.y];
                case Direction.Right:
                    return [this.x + 1, this.y];
            }
        })();

        const sTile = this.board.getTile(sx, sy);

        if (!sTile || sTile.tileType === TileType.Wall) {
            return false;
        }

        return true;
    };

    toRaw = (): CreatorTileRaw => ({
        tile_type: this.tileType,
        pawn: this.pawn,
        edges: this.edges,
        x: this.x,
        y: this.y,
    });

    fromRaw = (raw: CreatorTileRaw) => {
        this.tileType = raw.tile_type;
        this.pawn = raw.pawn;
        this.edges = raw.edges;
    };
}

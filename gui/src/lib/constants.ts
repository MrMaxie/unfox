export enum TileType {
    Empty,
    Goal,
    Wall,
}

export enum Direction {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

export enum Pawn {
    Fox,
    Monster,
}

export const TileTypeIcon = {
    [TileType.Empty]: '⬜',
    [TileType.Goal]: '🏁',
    [TileType.Wall]: '⬛',
};

export const MovementIcon = {
    [Direction.Up]: '⬆️',
    [Direction.Down]: '⬇️',
    [Direction.Left]: '⬅️',
    [Direction.Right]: '➡️',
    '': '',
};

export const PawnIcon = {
    '': '',
    [Pawn.Fox]: '🦊',
    [Pawn.Monster]: '👾',
};

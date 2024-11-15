import { normalizeNumber } from './normalizeNumber';
import { type CreatorTileRaw, CreatorTile } from './CreatorTile';

type CreatorBoardRaw = {
    tiles: Array<CreatorTileRaw>;
    width: number;
    height: number;
};

export const MIN_BOARD_SIZE = 3;
export const MAX_BOARD_SIZE = 32;

type CreatorBoardHolder = {
    _: CreatorBoard;
    time: number;
};

export class CreatorBoard {
    tiles: Array<CreatorTile> = [];

    width = 3;

    height = 3;

    lastBoardHolder: CreatorBoardHolder | null = null;

    private subscribers: Array<(value: CreatorBoardHolder) => CreatorBoardHolder> = [];

    constructor() {
        this.loadFromLocalStorage();
    }

    getBoardHolder = (delta = 0) => {
        let holder = this.lastBoardHolder;

        if (!holder) {
            this.lastBoardHolder = {
                _: this,
                time: 0,
            };
            holder = this.lastBoardHolder!;
        }

        if (!delta) {
            return holder;
        }

        const newHolder = {
            _: this,
            time: holder!.time + delta,
        };
        this.lastBoardHolder = newHolder;
        return newHolder;
    };

    subscribe = (callback: (value: CreatorBoardHolder) => CreatorBoardHolder) => {
        callback(this.getBoardHolder());

        this.subscribers.push(callback);

        return () => {
            this.subscribers = this.subscribers.filter(sub => sub !== callback);
        };
    };

    notifyChange = () => {
        const holder = this.getBoardHolder(1);
        this.subscribers.forEach(sub => sub(holder));
        this.saveToLocalStorage();
    };

    clear = () => {
        this.tiles = [];
        this.notifyChange();
    };

    setWidth = (width: number) => {
        this.width = normalizeNumber(width, [MIN_BOARD_SIZE, MAX_BOARD_SIZE]);
        this.tiles = this.tiles.filter(t => t.x < this.width && t.y < this.height);
        this.notifyChange();
    };

    setHeight = (height: number) => {
        this.height = normalizeNumber(height, [MIN_BOARD_SIZE, MAX_BOARD_SIZE]);
        this.tiles = this.tiles.filter(t => t.x < this.width && t.y < this.height);
        this.notifyChange();
    };

    getTile = (x: number, y: number) => {
        if (x < 0 || x >= this.width || y < 0 || y >= this.height) {
            return null;
        }

        let tile = this.tiles.find(t => t.x === x && t.y === y);

        if (!tile) {
            tile = new CreatorTile(this, x, y);
            this.tiles.push(tile);
        }

        return tile;
    };

    getAllTiles = () => {
        const tilesList: CreatorTile[] = [];

        for (let y = 0; y < this.height; y++) {
            for (let x = 0; x < this.width; x++) {
                const tile = this.getTile(x, y);

                if (tile) {
                    tilesList.push(tile);
                }
            }
        }

        return tilesList;
    };

    toRaw = (): CreatorBoardRaw => ({
        tiles: this.tiles.map(t => t.toRaw()),
        width: this.width,
        height: this.height,
    });

    saveToLocalStorage = () => {
        localStorage.setItem('board', JSON.stringify(this.toRaw()));
    };

    loadFromLocalStorage = () => {
        const raw = localStorage.getItem('board');

        try {
            if (raw) {
                this.fromRaw(JSON.parse(raw));
            }
        } catch (e) {
            localStorage.removeItem('board');
        }
    };

    fromRaw = (raw: CreatorBoardRaw) => {
        this.tiles = raw.tiles.map(t => {
            const tile = new CreatorTile(this, t.x, t.y);
            tile.fromRaw(t);
            return tile;
        });

        this.width = raw.width;
        this.height = raw.height;

        this.notifyChange();
    };
}

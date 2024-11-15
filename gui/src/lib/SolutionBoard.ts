import { SolutionStep, type SolutionStepRaw } from './SolutionStep';
import type { SolutionTile } from './SolutionTile';

type SolutionBoardRaw = {
    width: number;
    height: number;
    steps: Array<SolutionStepRaw>;
};

type SolutionBoardHolder = {
    _: SolutionBoard;
    time: number;
};

export class SolutionBoard {
    width = 3;
    height = 3;
    steps: Array<SolutionStep> = [];
    activeStepIndex = 0;

    lastBoardHolder: SolutionBoardHolder | null = null;

    private subscribers: Array<(value: SolutionBoardHolder) => SolutionBoardHolder> = [];

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

    subscribe = (callback: (value: SolutionBoardHolder) => SolutionBoardHolder) => {
        callback(this.getBoardHolder());

        this.subscribers.push(callback);

        return () => {
            this.subscribers = this.subscribers.filter(sub => sub !== callback);
        };
    };

    notifyChange = () => {
        const holder = this.getBoardHolder(1);
        this.subscribers.forEach(sub => sub(holder));
    };

    fromRaw = (raw: SolutionBoardRaw) => {
        this.width = raw.width;
        this.height = raw.height;
        this.steps = raw.steps.map((stepRaw) => {
            const step = new SolutionStep();
            step.fromRaw(stepRaw);
            step.fulfillMissingTiles(this.width, this.height);
            return step;
        });
    };

    getAllTiles = () => {
        const step = this.steps[this.activeStepIndex];
        const tiles: SolutionTile[] = [];

        for (let y = 0; y < this.height; y++) {
            for (let x = 0; x < this.width; x++) {
                tiles.push(step.getTile(x, y)!);
            }
        }

        return tiles;
    };

    getStepsNamesList = () => {
        return this.steps.map((step, index) => {
            if (index === 0) {
                return 'Start';
            } else if (index === this.steps.length - 1) {
                return 'End';
            }

            return String(index);
        });
    };

    setActiveStep = (index: number) => {
        this.activeStepIndex = index;
        this.notifyChange();
    };
}

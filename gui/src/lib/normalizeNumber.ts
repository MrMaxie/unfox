export const normalizeNumber = (value: string | number, range: [number, number]) => {
    let newValue = value;

    if (typeof newValue === 'string') {
        newValue = parseInt(newValue, 10);
    }

    if (isNaN(newValue)) {
        return range[0];
    }

    if (newValue < range[0]) {
        return range[0];
    }

    if (newValue > range[1]) {
        return range[1];
    }

    return newValue;
};
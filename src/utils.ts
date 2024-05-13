export function clamp(val: number, min: number, max: number) {
    return Math.min(Math.max(val, min), max);
}

export function capitalize(str: string) {
    return str[0].toUpperCase() + str.substring(1).toLowerCase();
}

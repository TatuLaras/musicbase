export function clamp(val: number, min: number, max: number) {
    return Math.min(Math.max(val, min), max);
}

export function capitalize(str: string) {
    return str[0].toUpperCase() + str.substring(1).toLowerCase();
}

export async function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

export function formatTime(s: number | undefined) {
    if (s === undefined) return '';
    const seconds = Math.floor(s);
    return `${Math.floor(s / 60)}:${(seconds % 60).toString().padStart(2, '0')}`;
}

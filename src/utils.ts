import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Album } from './ipc_types';

export function clamp(val: number, min: number, max: number) {
    return Math.min(Math.max(val, min), max);
}

export function wrap(val: number, max: number) {
    if (val < 0) return max + (val % max);
    return val % max;
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

export function songClass(i: number, queuePos: number) {
    if (i == queuePos) return 'current';
    if (i < queuePos) return 'past';

    return '';
}

export function shuffleArray<T>(array: T[]) {
    let currentIndex = array.length;

    // While there remain elements to shuffle...
    while (currentIndex != 0) {
        // Pick a remaining element...
        let randomIndex = Math.floor(Math.random() * currentIndex);
        currentIndex--;

        // And swap it with the current element.
        [array[currentIndex], array[randomIndex]] = [
            array[randomIndex],
            array[currentIndex],
        ];
    }
}

export function albumCover(
    album?: Album,
    preferred: 'original' | 'small' | 'tiny' = 'original',
) {
    if (!album) return '';
    let best = album.cover_path ?? '';
    if (preferred === 'small' || preferred === 'tiny')
        best = album.cover_path_small ?? best;
    if (preferred === 'tiny') best = album.cover_path_tiny ?? best;

    return convertFileSrc(best);
}

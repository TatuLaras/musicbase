import { invoke } from '@tauri-apps/api';

export namespace backend {
    export async function playSong(songId: number, queue: boolean) {
        await invoke('play_song', { songId, queue });
    }

    export async function replaceSong(songId: number) {
        await invoke('replace_song', { songId });
    }

    export async function seek(millisecs: number) {
        await invoke('seek', { millisecs: Math.round(millisecs) });
    }
}

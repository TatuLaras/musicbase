import { invoke } from '@tauri-apps/api';
import { Album, Directory, Playlist, Song, Tag } from './ipc_types';

export namespace backend {
    export async function playSong(songId: number, queue: boolean) {
        console.log('playSong command disabled');
        return;
        await invoke('play_song', { songId, queue });
    }

    export async function replaceSong(songId: number) {
        await invoke('replace_song', { songId });
    }

    export async function get_artist_albums(
        artistId: number,
    ): Promise<Album[]> {
        return (await invoke('get_artist_albums', { artistId })) as Album[];
    }

    export async function get_playlist(
        playlistId: number,
    ): Promise<Playlist | undefined> {
        return (await invoke('get_playlist', { playlistId })) as
            | Playlist
            | undefined;
    }

    export async function get_playlist_songs(
        playlistId: number,
    ): Promise<Song[]> {
        return (await invoke('get_playlist_songs', {
            playlistId,
        })) as Song[];
    }

    export async function create_playlist(name: string): Promise<Playlist> {
        return (await invoke('create_playlist', { name })) as Playlist;
    }

    export async function get_all_directories(): Promise<Directory[]> {
        return (await invoke('get_all_directories')) as Directory[];
    }

    export async function select_directory() {
        return await invoke('select_directory');
    }

    export async function delete_directory(directoryId: number) {
        return await invoke('delete_directory', { directoryId });
    }

    export async function get_album(
        albumId: number,
    ): Promise<Album | undefined> {
        return await invoke('get_album', { albumId });
    }

    export async function get_album_songs(albumId: number): Promise<Song[]> {
        return await invoke('get_album_songs', { albumId });
    }

    export async function select_cover(
        id: number,
        playlist: boolean,
    ): Promise<Song[]> {
        return await invoke('select_cover', {
            id,
            playlist,
        });
    }
    export async function create_tag(name: string): Promise<Tag> {
        return await invoke('create_tag', { name });
    }
}

import { invoke } from '@tauri-apps/api';
import { MainViewType } from '../../types';
import AlbumView, { AlbumViewData } from './AlbumView';
import { Album, Song } from '../../ipc_types';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import SettingsView from './SettingsView';
import { memo } from 'react';

export interface MainViewState {
    mainViewType: MainViewType;
    id?: number;
}

type Props = {
    mainViewState: MainViewState | null;
    onPlay: (queue: Song[], queuePos: number) => void;
    onQueue: (songs: Song[], start: boolean) => void;
};

function MainView({ mainViewState, onPlay, onQueue }: Props) {
    const content: { [key: string]: JSX.Element } = {
        album: mainViewState?.id ? (
            <AlbumView
                itemSource={getAlbum(mainViewState?.id)}
                onPlay={onPlay}
                onQueue={onQueue}
            />
        ) : (
            <></>
        ),
        playlist: <></>,
        playlistsByTag: <></>,
        albumsByTag: <></>,
        albumsByArtist: <></>,
        settings: <SettingsView />,
    };
    console.log(mainViewState?.mainViewType);

    // All the getSomething functions return an another function,
    // this will make it so we don't have to pass the id as a prop, we
    // just 'curry' the function.
    function getAlbum(id: number): () => Promise<AlbumViewData | null> {
        return async () => {
            const album = (await invoke('get_album', { albumId: id })) as
                | Album
                | undefined;

            const albumSongs = (await invoke('get_album_songs', {
                albumId: id,
            })) as Song[];

            const coverPath =
                album?.cover_path && album.cover_path.length > 0
                    ? convertFileSrc(album.cover_path)
                    : undefined;

            if (!album) return null;

            return {
                type: 'Album',
                title: album.name,
                songs: albumSongs,
                cover_path: coverPath,
                artist: album.artist,
                extraInfo: [album.year?.toString() ?? 'Year unknown'],
            };
        };
    }

    return (
        mainViewState && (
            <div id="main-view">
                {content[mainViewState.mainViewType] ?? <></>}
            </div>
        )
    );
}

export default memo(MainView);

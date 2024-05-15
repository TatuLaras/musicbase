import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { LibraryView } from '../../types';
import GridView, { GridItem } from './GridView';
import { memo } from 'react';

type Props = {
    view: LibraryView;
};

async function getAlbums(): Promise<GridItem[]> {
    const result = (await invoke('get_all_albums')) as Album[];

    return result.map((album) => {
        const image_url =
            album.cover_path && album.cover_path.length > 0
                ? convertFileSrc(album.cover_path)
                : undefined;

        return {
            id: album.album_id ?? 0,
            title: album.name,
            extra_info: album?.artist?.name ?? '',
            image_url,
            onSelected: function (id: number): void {
                console.log(`Select ${id}`);
            },
        };
    });
}

async function getArtists(): Promise<GridItem[]> {
    const result = (await invoke('get_all_artists')) as Artist[];
    return result.map((artist) => {
        const image_url =
            artist.artist_image_path && artist.artist_image_path.length > 0
                ? convertFileSrc(artist.artist_image_path)
                : undefined;

        return {
            id: artist.artist_id ?? 0,
            title: artist.name,
            extra_info: '',
            image_url,
            onSelected: function (id: number): void {
                console.log(`Select artist ${id}`);
            },
        };
    });
}

async function getPlaylists(): Promise<GridItem[]> {
    const result = (await invoke('get_all_playlists')) as Playlist[];
    return result.map((playlist) => {
        const image_url =
            playlist.cover_path && playlist.cover_path.length > 0
                ? convertFileSrc(playlist.cover_path)
                : undefined;

        return {
            id: playlist.playlist_id ?? 0,
            title: playlist.name,
            extra_info: playlist.tags.join(', '),
            image_url,
            onSelected: function (id: number): void {
                console.log(`Select artist ${id}`);
            },
        };
    });
}

function Library({ view }: Props) {
    const content: { [key: string]: JSX.Element } = {
        albums: <GridView item_source={getAlbums} />,
        artists: <GridView item_source={getArtists} circles={true} />,
        playlists: <GridView item_source={getPlaylists} />,
        tags: <h1>Tags</h1>,
    };

    return content[view] ?? <></>;
}

export default memo(Library);

import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { LibraryView } from '../../types';
import GridView, { GridItem } from './GridView';
import { memo } from 'react';
import Loading from '../Loading';

type Props = {
    view: LibraryView;
};

async function getAlbums(): Promise<GridItem[]> {
    const result = (await invoke('get_all_albums')) as Album[];

    return result.map((album) => {
        console.log(album);
        return {
            id: album.album_id ?? 0,
            title: album.name,
            extra_info: album?.artist?.name ?? '',
            image_url: convertFileSrc(album.cover_path ?? ''),
            onSelected: function (id: number): void {
                console.log(`Select ${id}`);
            },
        };
    });
}

async function getArtists(): Promise<GridItem[]> {
    const result = (await invoke('get_all_artists')) as Artist[];

    return result.map((artist) => {
        console.log(artist);
        return {
            id: artist.artist_id ?? 0,
            title: artist.name,
            extra_info: '',
            //  TODO: Älä tee ?? '' vaan joku kunnon placeholder tilalle
            image_url: convertFileSrc(artist.artist_image_path ?? ''),
            onSelected: function (id: number): void {
                console.log(`Select artist ${id}`);
            },
        };
    });
}

function Library({ view }: Props) {
    console.log('rerender');
    const content: { [key: string]: JSX.Element } = {
        albums: <GridView item_source={getAlbums} />,
        artists: <GridView item_source={getArtists} circles={true} />,
        playlists: <h1>Playlists</h1>,
        tags: <h1>Tags</h1>,
    };

    return content[view] ?? <></>;
}

export default memo(Library);

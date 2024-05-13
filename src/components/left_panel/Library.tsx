import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { LibraryView } from '../../types';
import GridView, { GridItem } from './GridView';
import { memo } from 'react';
import Loading from '../Loading';

type Props = {
    view: LibraryView;
};

async function getAlbums(): Promise<GridItem[]> {
    invoke('scan');
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

function Library({ view }: Props) {
    console.log('rerender');
    const content: { [key: string]: JSX.Element } = {
        albums: <GridView item_source={getAlbums} />,
        artists: <Loading />,
        playlists: <h1>Playlists</h1>,
        tags: <h1>Tags</h1>,
    };

    return content[view] ?? <></>;
}

export default memo(Library);

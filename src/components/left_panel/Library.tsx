import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { LibraryView } from '../../types';
import GridView, { GridItem } from './GridView';
import { memo, useEffect, useState } from 'react';
import ListView, { ListItem } from './ListView';
import Filters, { FilterState } from './Filters';
import { Album, Artist, Playlist, Tag } from '../../ipc_types';
import { MainViewState } from '../main_view/MainView';

type Props = {
    view: LibraryView;
    onMainViewSelected: (state: MainViewState) => void;
};

function Library({ view, onMainViewSelected }: Props) {
    const [filterState, setFilterState] = useState<FilterState>({
        searchQuery: '',
    });

    // Reset filter state on view change
    useEffect(() => {
        setFilterState({ searchQuery: '' });
    }, [view]);

    async function getAlbums(): Promise<GridItem[]> {
        const result = (await invoke('get_all_albums')) as Album[];

        return result.map((album) => {
            const imageUrl =
                album.cover_path && album.cover_path.length > 0
                    ? convertFileSrc(album.cover_path)
                    : undefined;

            return {
                id: album.album_id ?? 0,
                title: album.name,
                extraInfo: album?.artist?.name ?? '',
                imageUrl,
                onSelected: () =>
                    onMainViewSelected({
                        mainViewType: 'album',
                        id: album.album_id,
                    }),
            };
        });
    }

    async function getArtists(): Promise<GridItem[]> {
        const result = (await invoke('get_all_artists')) as Artist[];
        return result.map((artist) => {
            const imageUrl =
                artist.artist_image_path && artist.artist_image_path.length > 0
                    ? convertFileSrc(artist.artist_image_path)
                    : undefined;

            return {
                id: artist.artist_id ?? 0,
                title: artist.name,
                extraInfo: '',
                imageUrl,
                onSelected: function (id: number): void {
                    console.log(`Select artist ${id}`);
                },
            };
        });
    }

    async function getPlaylists(): Promise<GridItem[]> {
        const result = (await invoke('get_all_playlists')) as Playlist[];
        return result.map((playlist) => {
            const imageUrl =
                playlist.cover_path && playlist.cover_path.length > 0
                    ? convertFileSrc(playlist.cover_path)
                    : undefined;

            return {
                id: playlist.playlist_id ?? 0,
                title: playlist.name,
                extraInfo: playlist.tags.join(', '),
                imageUrl,
                onSelected: function (id: number): void {
                    console.log(`Select artist ${id}`);
                },
            };
        });
    }

    async function getTags(): Promise<ListItem[]> {
        const result = (await invoke('get_all_tags')) as Tag[];
        return result.map((tag) => {
            return {
                id: tag.tag_id ?? 0,
                title: tag.name,
                onSelected: function (id: number): void {
                    console.log(`Select tag ${id}`);
                },
            };
        });
    }

    const content: { [key: string]: JSX.Element } = {
        albums: (
            <>
                <Filters setFilterState={setFilterState} key={0} />
                <GridView itemSource={getAlbums} filterState={filterState} />
            </>
        ),
        artists: (
            <>
                <Filters setFilterState={setFilterState} key={1} />
                <GridView
                    itemSource={getArtists}
                    circles={true}
                    filterState={filterState}
                />
            </>
        ),
        playlists: (
            <>
                <Filters setFilterState={setFilterState} key={2} />
                <GridView itemSource={getPlaylists} filterState={filterState} />
            </>
        ),
        tags: (
            <>
                <Filters setFilterState={setFilterState} key={3} />
                <ListView item_source={getTags} filterState={filterState} />
            </>
        ),
    };

    return content[view] ?? <></>;
}

export default memo(Library);

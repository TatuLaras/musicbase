import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { LibraryView } from '../../types';
import GridView, { GridItem } from './GridView';
import { useEffect, useMemo, useState } from 'react';
import ListView, { ListItem } from './ListView';
import Filters, { FilterState } from './Filters';
import { Album, Artist, Playlist, Song, Tag } from '../../ipc_types';
import { MainViewState } from '../main_view/MainView';
import Button from '../Button';
import { Plus } from 'iconoir-react';
import { backend } from '../../ipc_commands';
import NamingModal from '../NamingModal';

type Props = {
    view: LibraryView;
    onMainViewSelected: (state: MainViewState) => void;
    onPlay: (queue: Song[], queuePos: number) => void;
};

function Library({ view, onMainViewSelected, onPlay }: Props) {
    const [filterState, setFilterState] = useState<FilterState>({
        searchQuery: '',
    });

    // Used to referesh the view when necessary
    const [changeThis, setChangeThis] = useState(0);

    const [showTagNamingModal, setShowTagNamingModal] = useState(false);
    const [showPlaylistNamingModal, setShowPlaylistNamingModal] =
        useState(false);

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
                onSelected: () =>
                    onMainViewSelected({
                        mainViewType: 'artist',
                        id: artist.artist_id,
                    }),
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
                onSelected: () =>
                    onMainViewSelected({
                        mainViewType: 'playlist',
                        id: playlist.playlist_id,
                    }),
            };
        });
    }

    async function getTags(): Promise<ListItem[]> {
        const result = (await invoke('get_all_tags')) as Tag[];
        return result.map((tag) => {
            return {
                id: tag.tag_id ?? 0,
                title: tag.name,
                onSelected: () =>
                    onMainViewSelected({
                        mainViewType: 'tag',
                        id: tag.tag_id,
                    }),
            };
        });
    }

    //  NOTE: Add state to the dependency array if you want the components to rerender according to them.
    //  Passing them as props is not enough.
    const content = useMemo(() => {
        return {
            albums: (
                <>
                    <Filters setFilterState={setFilterState} key={0} />
                    <GridView
                        itemSource={getAlbums}
                        filterState={filterState}
                        playButton={true}
                        onPlayButtonClicked={(albumId: number) => {
                            backend
                                .get_album_songs(albumId)
                                .then((albumSongs) => {
                                    if (albumSongs.length > 0)
                                        onPlay(albumSongs, 0);
                                });
                        }}
                    />
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
                    <NamingModal
                        show={showPlaylistNamingModal}
                        title="Provide a playlist name"
                        inputPlaceholder="Playlist name"
                        onDone={(result: string | null) => {
                            setShowPlaylistNamingModal(false);
                            if (!result) return;

                            backend.create_playlist(result).then((playlist) => {
                                // Open the newly created thing in main view
                                if (playlist)
                                    onMainViewSelected({
                                        mainViewType: 'playlist',
                                        id: playlist.playlist_id,
                                    });
                            });
                            setChangeThis((old) => old + 1);
                        }}
                    />
                    <Filters setFilterState={setFilterState} key={2} />
                    <Button
                        primary={true}
                        className="add-btn"
                        round={true}
                        title="Add playlist"
                        onClick={() => setShowPlaylistNamingModal(true)}
                    >
                        <Plus />
                    </Button>
                    <GridView
                        itemSource={getPlaylists}
                        filterState={filterState}
                        playButton={true}
                        onPlayButtonClicked={(playlistId: number) => {
                            backend
                                .get_playlist_songs(playlistId)
                                .then((playlistSongs) => {
                                    if (playlistSongs.length > 0)
                                        onPlay(playlistSongs, 0);
                                });
                        }}
                    />
                </>
            ),
            tags: (
                <>
                    <NamingModal
                        show={showTagNamingModal}
                        title="Provide a tag name"
                        inputPlaceholder="Tag name"
                        onDone={(result: string | null) => {
                            setShowTagNamingModal(false);
                            if (!result) return;

                            backend.create_tag(result).then((tag) => {
                                // Open the newly created thing in main view
                                if (tag)
                                    onMainViewSelected({
                                        mainViewType: 'tag',
                                        id: tag.tag_id,
                                    });
                            });
                            setChangeThis((old) => old + 1);
                        }}
                    />
                    <Filters setFilterState={setFilterState} key={3} />
                    <Button
                        primary={true}
                        className="add-btn"
                        round={true}
                        title="Add tag"
                        onClick={() => setShowTagNamingModal(true)}
                    >
                        <Plus />
                    </Button>
                    <ListView item_source={getTags} filterState={filterState} />
                </>
            ),
        };
    }, [filterState, changeThis, showTagNamingModal, showPlaylistNamingModal]);

    return content[view] ?? <></>;
}

export default Library;

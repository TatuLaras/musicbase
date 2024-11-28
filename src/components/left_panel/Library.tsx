import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { LibraryView } from '../../types';
import GridView, { GridItem } from './GridView';
import { useContext, useEffect, useMemo, useState } from 'react';
import ListView, { ListItem } from './ListView';
import Filters, { FilterState } from './Filters';
import { Album, Artist, Playlist, Tag } from '../../ipc_types';
import { MainViewContext } from '../main_view/MainView';
import Button from '../Button';
import { Plus } from 'iconoir-react';
import { backend } from '../../ipc_commands';
import NamingModal from '../NamingModal';
import { PlayerContext } from '../player/Player';
import { ContextMenuItem } from '../ContextMenu';
import FormModal from '../FormModal';
import PlaylistSelectModal, {
    PlaylistSelectModalState,
} from '../PlaylistSelectModal';

type Props = {
    view: LibraryView;
    forceRefresh?: number;
};

function Library({ view, forceRefresh = 0 }: Props) {
    const mainViewContext = useContext(MainViewContext);
    const onMainViewSelected = mainViewContext.onMainViewSelected;
    const onMainViewUpdate = mainViewContext.onMainViewUpdate;

    const playerContext = useContext(PlayerContext);

    const [filterState, setFilterState] = useState<FilterState>({
        searchQuery: '',
    });

    // Used to referesh the view when necessary
    const [changeThis, setChangeThis] = useState(0);

    const [showTagNamingModal, setShowTagNamingModal] = useState(false);
    const [showPlaylistNamingModal, setShowPlaylistNamingModal] =
        useState(false);

    const [playlistSelectModalState, setPlaylistSelectModalState] =
        useState<PlaylistSelectModalState>({ show: false });

    const [showPlaylistEditModal, setShowPlaylistEditModal] = useState(false);
    const [playlistEditModalData, setPlaylistEditModalData] =
        useState<Playlist | null>(null);

    // Reset filter state on view change
    useEffect(() => {
        setFilterState({ searchQuery: '' });
    }, [view]);

    function queueAlbum(albumId?: number, start: boolean = false) {
        if (!albumId) return;
        backend.get_album_songs(albumId).then((albumSongs) => {
            if (albumSongs.length > 0) playerContext.onQueue(albumSongs, start);
        });
    }

    function queuePlaylist(playlistId?: number, start: boolean = false) {
        if (!playlistId) return;
        backend.get_playlist_songs(playlistId).then((playlistSongs) => {
            if (playlistSongs.length > 0)
                playerContext.onQueue(playlistSongs, start);
        });
    }

    function addPlaylistToPlaylist(
        sourcePlaylistId?: number,
        targetPlaylistId?: number,
    ) {
        if (!sourcePlaylistId || !targetPlaylistId) return;
        backend.get_playlist_songs(sourcePlaylistId).then((playlistSongs) => {
            backend
                .add_songs_to_playlist(
                    playlistSongs
                        .filter((song) => song.song_id)
                        .map((song) => song.song_id!),
                    targetPlaylistId,
                )
                .then(onMainViewUpdate);
        });
    }

    function addAlbumToPlaylist(albumId?: number, playlistId?: number) {
        if (!albumId || !playlistId) return;
        backend.get_album_songs(albumId).then((albumSongs) => {
            backend
                .add_songs_to_playlist(
                    albumSongs
                        .filter((song) => song.song_id)
                        .map((song) => song.song_id!),
                    playlistId,
                )
                .then(onMainViewUpdate);
        });
    }

    async function getAlbums(): Promise<GridItem[]> {
        const result = (await invoke('get_all_albums')) as Album[];

        return result.map((album) => {
            const imageUrl =
                album.cover_path_small && album.cover_path_small.length > 0
                    ? convertFileSrc(album.cover_path_small)
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
                contextMenuItems: [
                    {
                        label: 'Add to',
                        subitems: [
                            {
                                label: 'Play queue',
                                onClick: () => queueAlbum(album.album_id),
                            },
                            {
                                label: 'Play queue (next)',
                                onClick: () => queueAlbum(album.album_id, true),
                            },
                            {
                                label: 'Playlist',
                                onClick: () => {
                                    setPlaylistSelectModalState({
                                        show: true,
                                        albumId: album.album_id,
                                    });
                                },
                            },
                        ],
                    },
                    { label: 'Select cover' },
                ],
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

    const getPlaylists = async (): Promise<GridItem[]> => {
        const playlists = await backend.get_all_playlists();

        return playlists.map((playlist) => {
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
                contextMenuItems: [
                    {
                        label: 'Add to',
                        subitems: [
                            {
                                label: 'Play queue',
                                onClick: () =>
                                    queuePlaylist(playlist.playlist_id),
                            },
                            {
                                label: 'Play queue (next)',
                                onClick: () =>
                                    queuePlaylist(playlist.playlist_id, true),
                            },
                            {
                                label: 'Playlist',
                                subitems: playlists.map((menuPlaylist) => {
                                    return {
                                        label: menuPlaylist.name,
                                        onClick: () =>
                                            addPlaylistToPlaylist(
                                                playlist.playlist_id,
                                                menuPlaylist.playlist_id,
                                            ),
                                    } as ContextMenuItem;
                                }),
                            },
                        ],
                    },
                    { label: 'Select cover' },
                    {
                        label: 'Edit information',
                        onClick: () => {
                            setShowPlaylistEditModal(true);
                            setPlaylistEditModalData(playlist);
                        },
                    },
                ],
            };
        });
    };

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
                    <PlaylistSelectModal
                        modalState={playlistSelectModalState}
                        onDone={(albumId, playlistId) => {
                            setPlaylistSelectModalState({ show: false });
                            if (!albumId || !playlistId) return;
                            addAlbumToPlaylist(albumId, playlistId);
                        }}
                    />
                    <Filters setFilterState={setFilterState} key={0} />
                    <GridView
                        itemSource={getAlbums}
                        filterState={filterState}
                        playButton={true}
                        showTutorial={true}
                        onPlayButtonClicked={(albumId: number) => {
                            backend
                                .get_album_songs(albumId)
                                .then((albumSongs) => {
                                    if (albumSongs.length > 0)
                                        playerContext.onPlay(albumSongs, 0);
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
                        title="Create playlist"
                        inputLabel="Playlist name"
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
                    <FormModal<Playlist>
                        title="Edit playlist"
                        show={showPlaylistEditModal}
                        fields={[
                            {
                                name: 'name',
                                required: true,
                                value: playlistEditModalData?.name,
                            },
                            {
                                name: 'desc',
                                value: playlistEditModalData?.desc,
                            },
                        ]}
                        onDone={(playlist) => {
                            setShowPlaylistEditModal(false);
                            if (!playlist) return;
                            playlist.playlist_id =
                                playlistEditModalData?.playlist_id;
                            backend
                                .edit_playlist(playlist)
                                .then(() => onMainViewUpdate());
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
                                        playerContext.onPlay(playlistSongs, 0);
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
    }, [
        filterState,
        showTagNamingModal,
        showPlaylistNamingModal,
        showPlaylistEditModal,
        forceRefresh,
        playlistSelectModalState,
    ]);

    return content[view] ?? <></>;
}

export default Library;

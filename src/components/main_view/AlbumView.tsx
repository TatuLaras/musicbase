import { CSSProperties, useContext, useEffect, useState } from 'react';
import { Album, Artist, Playlist, Song } from '../../ipc_types';
import ImagePlaceholder from '../ImagePlaceholder';
import { PlaySolid, Shuffle } from 'iconoir-react';
import Button from '../Button';
import { capitalize, formatTime, shuffleArray } from '../../utils';
import { backend } from '../../ipc_commands';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { PlayerContext } from '../player/Player';
import { ContextMenuCallbackContext, ContextMenuItem } from '../ContextMenu';
import FormModal from '../FormModal';
import { MainViewContext } from './MainView';

export interface AlbumViewData {
    type: 'album' | 'playlist';
    title: string;
    artist?: Artist;
    cover_path?: string;
    extraInfo: string[];
    songs: Song[];
}

type Props = {
    id: number;
    isPlaylist?: boolean;
};

export default function AlbumView({ id, isPlaylist = false }: Props) {
    const openContextMenu = useContext(ContextMenuCallbackContext);
    const playerContext = useContext(PlayerContext);
    const mainViewContext = useContext(MainViewContext);
    const onMainViewUpdate = mainViewContext.onMainViewUpdate;

    const [viewAlbumArt, setViewAlbumArt] = useState(false);
    const [viewData, setViewData] = useState<AlbumViewData | null>(null);

    const [showPlaylistEditModal, setShowPlaylistEditModal] = useState(false);
    const [playlistEditModalData, setPlaylistEditModalData] =
        useState<Playlist | null>(null);

    useEffect(getData, [id, isPlaylist]);

    const topPortionContextMenu: ContextMenuItem[] = [
        {
            label: 'Add to',
            subitems: [
                {
                    label: 'Play queue',
                    onClick: () =>
                        viewData?.songs &&
                        playerContext.onQueue(viewData?.songs),
                },
                {
                    label: 'Play queue (next)',
                    onClick: () =>
                        viewData?.songs &&
                        playerContext.onQueue(viewData?.songs, true),
                },
                {
                    label: 'Playlist',
                },
            ],
        },
        { label: 'Select cover', onClick: editCover },
    ];

    //  NOTE: Only playlists can be edited. This app is not a tag editor.
    if (isPlaylist)
        topPortionContextMenu.push({
            label: 'Edit information',
            onClick: () => {
                backend.get_playlist(id).then((playlist) => {
                    if (!playlist) return;
                    setShowPlaylistEditModal(true);
                    setPlaylistEditModalData(playlist);
                });
            },
        });

    function getData() {
        if (isPlaylist) {
            backend.get_playlist(id).then((playlist) => {
                if (!playlist) return;
                backend.get_playlist_songs(id).then((playlistSongs) =>
                    setViewData({
                        type: 'playlist',
                        title: playlist.name,
                        songs: playlistSongs,
                        cover_path:
                            playlist?.cover_path &&
                            playlist.cover_path.length > 0
                                ? convertFileSrc(playlist.cover_path)
                                : undefined,
                        extraInfo: [playlist.desc],
                    }),
                );
            });
            return;
        }

        backend.get_album(id).then((album) => {
            if (!album) return;
            backend.get_album_songs(id).then((albumSongs) =>
                setViewData({
                    type: 'album',
                    title: album.name,
                    songs: albumSongs,
                    cover_path:
                        album?.cover_path && album.cover_path.length > 0
                            ? convertFileSrc(album.cover_path)
                            : undefined,
                    artist: album.artist,
                    extraInfo: [album.year?.toString() ?? 'Year unknown'],
                }),
            );
        });
    }

    // Construct queue from the clicked song until the end of the album
    function play(index: number, shuffle: boolean = false) {
        if (!viewData?.songs || viewData.songs.length == 0) return;

        let songs = [...viewData.songs];
        if (shuffle) shuffleArray(songs);

        playerContext.onPlay(songs, index);
    }

    async function editCover() {
        await backend.select_cover(id, isPlaylist);
        getData();
    }

    return viewData ? (
        <div className="album-view">
            <div
                className={`cover-full ${viewAlbumArt ? 'show' : ''}`}
                onClick={() => setViewAlbumArt(false)}
            >
                {viewData.cover_path && (
                    <img
                        src={viewData.cover_path}
                        alt={`Cover for ${viewData.title}`}
                        draggable={false}
                    />
                )}
            </div>
            <div
                className="top-portion"
                onContextMenu={(e) =>
                    openContextMenu({
                        items: topPortionContextMenu,
                        mousePosX: e.clientX,
                        mousePosY: e.clientY,
                    })
                }
            >
                {viewData.cover_path ? (
                    <div
                        className="cover"
                        style={
                            {
                                '--img': `url(${viewData.cover_path})`,
                            } as CSSProperties
                        }
                        onClick={() => {
                            setViewAlbumArt(true);
                        }}
                    ></div>
                ) : (
                    <ImagePlaceholder edit={true} onEdit={editCover} />
                )}
                <div className="info">
                    <div className="type">{capitalize(viewData.type)}</div>
                    <h1>{viewData.title}</h1>
                    <div className="extra-info">
                        {viewData.extraInfo.map((info, i) => (
                            <div className="info-item" key={i}>
                                <div className="circle"></div>
                                <div className="text">{info}</div>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
            <div
                className="button-row"
                onContextMenu={(e) =>
                    openContextMenu({
                        items: topPortionContextMenu,
                        mousePosX: e.clientX,
                        mousePosY: e.clientY,
                    })
                }
            >
                {viewData?.songs && viewData.songs.length > 0 && (
                    <>
                        <Button
                            primary={true}
                            text="Play"
                            onClick={() => play(0)}
                        >
                            <PlaySolid />
                        </Button>
                        <Button text="Shuffle" onClick={() => play(0, true)}>
                            <Shuffle />
                        </Button>
                    </>
                )}
            </div>
            <div className="song-list">
                <div className="header">
                    <div className="number">#</div>
                    <div className="title">TITLE</div>
                    <div className="artist">ARTIST</div>
                    <div className="length">LENGTH</div>
                </div>
                {viewData.songs.map((song, i) => (
                    <div
                        className="song-item"
                        key={i}
                        onClick={() => play(i)}
                        onContextMenu={(e) => {
                            openContextMenu({
                                items: [
                                    {
                                        label: 'Add to',
                                        subitems: [
                                            {
                                                label: 'Play queue',
                                                onClick: () =>
                                                    playerContext.onQueue([
                                                        song,
                                                    ]),
                                            },
                                            {
                                                label: 'Play queue (next)',
                                                onClick: () =>
                                                    playerContext.onQueue(
                                                        [song],
                                                        true,
                                                    ),
                                            },
                                            { label: 'Playlist' },
                                        ],
                                    },
                                    { label: 'Edit information' },
                                ],
                                mousePosX: e.clientX,
                                mousePosY: e.clientY,
                            });
                        }}
                    >
                        <div className="number">
                            <div className="n">{i + 1}</div>
                            <div className="play">
                                <PlaySolid />
                            </div>
                        </div>
                        <div className="title">{song.name}</div>
                        <div className="artist">
                            {song.artist?.name ?? 'Unknown'}
                        </div>
                        <div className="length">
                            {formatTime(song.duration_s)}
                        </div>
                        <div className="buttons"></div>
                    </div>
                ))}
            </div>
            {isPlaylist ? (
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
            ) : null}
        </div>
    ) : null;
}

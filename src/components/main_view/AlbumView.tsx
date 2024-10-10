import { CSSProperties, useEffect, useState } from 'react';
import { Artist, Song } from '../../ipc_types';
import ImagePlaceholder from '../ImagePlaceholder';
import { MoreHoriz, PlaySolid, Shuffle } from 'iconoir-react';
import Button from '../Button';
import { capitalize, formatTime } from '../../utils';
import { backend } from '../../ipc_commands';
import { convertFileSrc } from '@tauri-apps/api/tauri';

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
    onPlay: (queue: Song[], queuePos: number) => void;
    onQueue: (songs: Song[], start: boolean) => void;
};

export default function AlbumView({
    id,
    onPlay,
    onQueue,
    isPlaylist = false,
}: Props) {
    const [viewAlbumArt, setViewAlbumArt] = useState(false);
    const [viewData, setViewData] = useState<AlbumViewData | null>(null);

    useEffect(getData, [id, isPlaylist]);

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
    function play(index: number) {
        if (viewData?.songs && viewData.songs.length > 0)
            onPlay(viewData.songs, index);
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
            <div className="top-portion">
                {viewData.cover_path ? (
                    <div
                        className="cover"
                        style={
                            {
                                '--img': `url(${viewData.cover_path})`,
                            } as CSSProperties
                        }
                        onClick={() => {
                            console.log('cover click');
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
            <div className="button-row">
                <div className="start">
                    {viewData?.songs && viewData.songs.length > 0 && (
                        <>
                            <Button
                                primary={true}
                                text="Play"
                                onClick={() => play(0)}
                            >
                                <PlaySolid />
                            </Button>
                            <Button text="Shuffle" onClick={() => play(0)}>
                                <Shuffle />
                            </Button>
                        </>
                    )}
                </div>
                <div className="end">
                    <button className="icon-btn">
                        <MoreHoriz />
                    </button>
                </div>
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
                        key={song.song_id}
                        onClick={() => play(i)}
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
        </div>
    ) : null;
}

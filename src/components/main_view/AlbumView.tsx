import { useEffect, useState } from 'react';
import { Artist, Song } from '../../ipc_types';
import Loading from '../Loading';
import ImagePlaceholder from '../ImagePlaceholder';
import { MoreHoriz, PlaySolid, Shuffle } from 'iconoir-react';
import Button from '../Button';
import { formatTime } from '../../utils';

export interface AlbumViewData {
    type: 'Album' | 'Playlist';
    title: string;
    artist?: Artist;
    cover_path?: string;
    extraInfo: string[];
    songs: Song[];
}

type Props = {
    itemSource: () => Promise<AlbumViewData | null>;
    onPlay: (queue: Song[], queuePos: number) => void;
    onQueue: (songs: Song[], start: boolean) => void;
};

export default function AlbumView({ itemSource, onPlay, onQueue }: Props) {
    const [album, setAlbum] = useState<AlbumViewData | null>(null);
    const [loading, setLoading] = useState(false);
    const [viewAlbumArt, setViewAlbumArt] = useState(false);

    // Get items from the async function provided
    useEffect(() => {
        setLoading(true);
        itemSource().then((res) => {
            setAlbum(res);
            setLoading(false);
        });
    }, [itemSource]);

    let content: JSX.Element | null = <Loading />;

    // Construct queue from the clicked song until the end of the album
    function play(index: number) {
        if (album?.songs) onPlay(album.songs, index);
    }

    if (!loading && album)
        content = (
            <div className="album-view">
                <div
                    className={`cover-full ${viewAlbumArt ? 'show' : ''}`}
                    onClick={() => setViewAlbumArt(false)}
                >
                    {album.cover_path && (
                        <img
                            src={album.cover_path}
                            alt={`Cover for ${album.title}`}
                            draggable={false}
                        />
                    )}
                </div>
                <div className="top-portion">
                    {album.cover_path ? (
                        <img
                            src={album.cover_path}
                            alt={`Cover for ${album.title}`}
                            draggable={false}
                            onClick={() => {
                                console.log('cover click');
                                setViewAlbumArt(true);
                            }}
                        />
                    ) : (
                        <ImagePlaceholder />
                    )}
                    <div className="info">
                        <div className="type">{album.type}</div>
                        <h1>{album.title}</h1>
                        <div className="extra-info">
                            {album.extraInfo.map((info, i) => (
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
                    {album.songs.map((song, i) => (
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
        );

    return content;
}

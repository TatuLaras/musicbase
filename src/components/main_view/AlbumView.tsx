import { useEffect, useState } from 'react';
import { Artist, Song } from '../../ipc_types';
import Loading from '../Loading';
import ImagePlaceholder from '../ImagePlaceholder';
import { MoreHoriz, Play, PlaySolid, Shuffle } from 'iconoir-react';
import Button from '../Button';
import { formatSongLength } from '../../utils';

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
};

export default function AlbumView({ itemSource }: Props) {
    const [item, setItem] = useState<AlbumViewData | null>(null);
    const [loading, setLoading] = useState(false);

    // Get grid items from the async function provided
    useEffect(() => {
        setLoading(true);
        itemSource().then((res) => {
            setItem(res);
            setLoading(false);
        });
    }, [itemSource]);

    let content: JSX.Element | null = <Loading />;

    if (!loading && item)
        content = (
            <div className="album-view">
                <div className="top-portion">
                    {item.cover_path ? (
                        <img
                            src={item.cover_path}
                            alt={`Cover for ${item.title}`}
                            draggable={false}
                        />
                    ) : (
                        <ImagePlaceholder />
                    )}
                    <div className="info">
                        <div className="type">{item.type}</div>
                        <h1>{item.title}</h1>
                        <div className="extra-info">
                            {item.extraInfo.map((info, i) => (
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
                        <Button primary={true} text="Play">
                            <PlaySolid />
                        </Button>
                        <Button text="Shuffle">
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
                    {item.songs.map((song, i) => (
                        <div className="song-item" key={song.song_id}>
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
                                {formatSongLength(song.duration_s)}
                            </div>
                            <div className="buttons"></div>
                        </div>
                    ))}
                </div>
            </div>
        );

    return content;
}

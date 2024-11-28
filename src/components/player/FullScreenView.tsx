import { CSSProperties, Dispatch, SetStateAction, useCallback } from 'react';
import { Song } from '../../ipc_types';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { songClass } from '../../utils';
import { Maximize } from 'iconoir-react';

interface Props {
    fullscreen: boolean;
    setFullscreen: Dispatch<SetStateAction<boolean>>;
    song: Song;
    queue: Song[];
    queuePos: number;
    totalTime: number;
    elapsedTime: number;
}

export default function FullscreenView({
    fullscreen,
    setFullscreen,
    song,
    queue,
    queuePos,
    totalTime,
    elapsedTime,
}: Props) {
    const img = {
        '--img': `url(${convertFileSrc(song.album?.cover_path ?? '')})`,
    } as CSSProperties;
    const queueSlice = queue.slice(queuePos + 1, queuePos + 6);
    const remainingSongs = useCallback(
        () => queue.length - queuePos - 1,
        [queue, queuePos],
    );

    const progress = `${Math.min(elapsedTime / totalTime, 1) * 100}%`;

    return (
        <div
            className={fullscreen ? 'fullscreen' : ''}
            style={{ '--progress': progress } as CSSProperties}
        >
            <div className="full-screen-bg" style={img}></div>
            <div
                className={`full-screen-view ${fullscreen ? 'enabled' : ''}`}
                style={img}
            >
                <div className="progress"></div>
                <button
                    className="icon-btn"
                    onClick={() => setFullscreen(false)}
                >
                    <Maximize />
                </button>
                <div className="left">
                    <div className="cover"></div>
                </div>
                <div className="right">
                    <div className="info">
                        <h1 className="name">{song.name}</h1>
                        <div className="artist text-disabled">
                            {song.artist?.name}
                        </div>
                    </div>
                    <div className="queue">
                        {queueSlice.map((song, i) => (
                            <div
                                className={`song ${songClass(i, queuePos)} i-${remainingSongs() <= 5 ? 'p' : queueSlice.length - i - 1}`}
                                key={i}
                                style={
                                    {
                                        '--img': `url(${convertFileSrc(song.album?.cover_path ?? '')})`,
                                    } as CSSProperties
                                }
                            >
                                <div className="cover"></div>
                                <div className="info">
                                    <div className="name">{song.name}</div>
                                    <div className="artist">
                                        {song.artist?.name}
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}

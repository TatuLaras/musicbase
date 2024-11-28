import {
    CSSProperties,
    Dispatch,
    SetStateAction,
    createContext,
    useCallback,
    useContext,
    useEffect,
    useRef,
    useState,
} from 'react';
import ProgressBar from './ProgressBar';
import {
    LineSpace,
    Maximize,
    PauseSolid,
    PlaySolid,
    Repeat,
    Settings,
    Shuffle,
    SkipNextSolid,
    SkipPrevSolid,
    Xmark,
} from 'iconoir-react';
import { MainViewContext } from '../main_view/MainView';
import { Song } from '../../ipc_types';
import { backend } from '../../ipc_commands';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import ImagePlaceholder from '../ImagePlaceholder';
import { albumCover, songClass, wrap } from '../../utils';
import FullscreenView from './FullScreenView';
import SafeImage from '../SafeImage';

export interface PlayerContextState {
    onPlay: (queue: Song[], queuePos: number) => void;
    onQueue: (songs: Song[], start?: boolean) => void;
}

export const PlayerContext = createContext<PlayerContextState>({
    onPlay: () => {},
    onQueue: () => {},
});

type Props = {
    queue: Song[];
    queuePos: number;
    setQueue: Dispatch<SetStateAction<Song[]>>;
    setQueuePos: Dispatch<SetStateAction<number>>;
    shouldReset: boolean;
    setShouldReset: Dispatch<SetStateAction<boolean>>;
};

export default function Player({
    queue,
    setQueue,
    queuePos,
    setQueuePos,
    shouldReset,
    setShouldReset,
}: Props) {
    const onMainViewSelected = useContext(MainViewContext).onMainViewSelected;

    const [songStartTime, setSongStartTime] = useState(0);
    const [totalTime, setTotalTime] = useState(0);
    const [currentSong, setCurrentSong] = useState<Song | null>(null);
    const [playing, setPlaying] = useState(false);
    const [time, setTime] = useState(0);
    const [loadedNextSong, setLoadedNextSong] = useState<number | null>(null);
    const [enableQueuePanel, setEnableQueuePanel] = useState(false);
    const [disabled, setDisabled] = useState(true);
    const [shuffle, setShuffle] = useState(false);
    const [repeat, setRepeat] = useState(false);
    const [fullscreen, setFullscreen] = useState(false);

    const currentSongQueueItem = useRef<HTMLDivElement | null>(null);
    useEffect(() => {
        if (queueItem(1)?.song_id != loadedNextSong)
            console.log('TODO: Should replace the next song');

        console.log('Queue changed: ', queue);
    }, [queue]);

    useEffect(() => {
        if (shouldReset) reset();
    }, [shouldReset]);

    // Safer way to get items from the queue.
    // (as opposed to queue[someindex])
    const queueItem = useCallback(
        (offset: number = 0) => {
            const index = queuePos + offset;
            if (index >= queue.length) return null;
            return queue[index];
        },
        [queuePos, queue],
    );

    useEffect(() => {
        if (!currentSongQueueItem.current) return;
        currentSongQueueItem.current.scrollIntoView({
            behavior: 'smooth',
            block: 'center',
        });
    }, [queuePos]);

    function reset() {
        const current = queueItem();
        const next = queueItem(1);

        if (!current) return;

        // Start immediately playing the first item and queue the next
        backend.playSong(current.song_id!, false);

        // if (next) {
        //     backend.playSong(next.song_id!, true);
        //     setLoadedNextSong(next.song_id!);
        // }

        setSongStartTime(Date.now());
        setTotalTime(current.duration_s ?? 0);
        setCurrentSong(current);
        setPlaying(true);
        setShouldReset(false);
        setDisabled(false);
    }

    const nextSong = useCallback(() => {
        const current = queueItem(1);
        const next = queueItem(2);

        if (!current) {
            if (repeat) {
                setQueuePos(0);
                setShouldReset(true);
            } else {
                setPlaying(false);
            }

            return;
        }

        // Add next song to queue
        if (next) backend.playSong(next.song_id!, true);

        setCurrentSong(current);
        setSongStartTime(Date.now());
        setTotalTime(current.duration_s ?? 0);
        setPlaying(true);
        setCurrentSong(current);
        setQueuePos((old) => old + 1);
    }, [repeat, queueItem]);

    function seek(time: number) {
        setSongStartTime(Date.now() - time * 1000);
    }

    // Updates the time and calls nextSong() when needed
    useEffect(() => {
        const interval = setInterval(() => {
            if (!playing) return;

            const elapsedTime = (Date.now() - songStartTime) / 1000;

            if (elapsedTime >= totalTime) nextSong();
            else setTime(elapsedTime);
        }, 250);
        return () => clearInterval(interval);
    }, [songStartTime, playing, totalTime]);

    return (
        <>
            {currentSong && (
                <FullscreenView
                    song={currentSong}
                    fullscreen={fullscreen}
                    setFullscreen={setFullscreen}
                    queue={queue}
                    queuePos={queuePos}
                    totalTime={totalTime}
                    elapsedTime={time}
                />
            )}
            <div className={`queue-panel ${enableQueuePanel ? 'enable' : ''}`}>
                <div className="content">
                    {queue.map((song, i) => (
                        <div
                            className={`song ${songClass(i, queuePos)}`}
                            onClick={() => {
                                setQueuePos(i);
                                setShouldReset(true);
                            }}
                            ref={
                                i == queuePos ? currentSongQueueItem : undefined
                            }
                            key={i}
                        >
                            <SafeImage
                                src={albumCover(song.album, 'tiny')}
                            >
                                <div className="icon">
                                    <div className="inner">
                                        <PlaySolid />
                                    </div>
                                </div>
                            </SafeImage>
                            <div className="info">
                                <div className="name">{song.name}</div>
                                <div className="artist">
                                    {song.artist?.name}
                                </div>
                            </div>
                            <button
                                className="icon-btn"
                                onClick={(e) => {
                                    setQueue((old) => {
                                        const copy = [...old];
                                        copy.splice(i, 1);
                                        return copy;
                                    });
                                    e.stopPropagation();
                                }}
                            >
                                <Xmark />
                            </button>
                        </div>
                    ))}
                </div>
            </div>
            <div className={`player ${disabled ? 'disabled' : ''}`}>
                <div className="current-song side">
                    {currentSong && (
                        <>
                            {currentSong?.album?.cover_path ? (
                                <img
                                    src={convertFileSrc(
                                        currentSong?.album?.cover_path,
                                    )}
                                    alt={`Cover for ${currentSong?.name}`}
                                    draggable={false}
                                />
                            ) : (
                                <ImagePlaceholder />
                            )}
                            <div className="details">
                                <div className="title-row">
                                    <div className="title">
                                        {currentSong?.name}
                                    </div>
                                </div>
                                <div className="artist">
                                    {currentSong?.artist?.name}
                                </div>
                            </div>
                        </>
                    )}
                </div>
                <div className="controls">
                    <div className="buttons">
                        <button
                            className={`shuffle ${shuffle ? 'selected' : ''}`}
                            onClick={() => setShuffle((old) => !old)}
                        >
                            <Shuffle />
                        </button>
                        <button
                            className="prev"
                            onClick={() => {
                                setQueuePos((old) =>
                                    wrap(old - 1, queue.length),
                                );
                                setShouldReset(true);
                            }}
                        >
                            <SkipPrevSolid />
                        </button>
                        <button
                            className="play"
                            onClick={() =>
                                disabled ? null : setPlaying((old) => !old)
                            }
                        >
                            {playing && !disabled ? (
                                <PauseSolid />
                            ) : (
                                <PlaySolid />
                            )}
                        </button>
                        <button
                            className="next"
                            onClick={() => {
                                setQueuePos((old) =>
                                    wrap(old + 1, queue.length),
                                );
                                setShouldReset(true);
                            }}
                        >
                            <SkipNextSolid />
                        </button>
                        <button
                            className={`repeat ${repeat ? 'selected' : ''}`}
                            onClick={() => setRepeat((old) => !old)}
                        >
                            <Repeat />
                        </button>
                    </div>
                    <ProgressBar
                        totalTime={totalTime}
                        elapsedTime={time}
                        onTimeSet={seek}
                    />
                </div>
                <div className="options side">
                    <button onClick={() => setFullscreen(true)}>
                        <Maximize />
                    </button>
                    <button
                        onClick={() =>
                            disabled ? null : setEnableQueuePanel((old) => !old)
                        }
                    >
                        <LineSpace />
                    </button>
                    <button
                        onClick={() => {
                            onMainViewSelected({ mainViewType: 'settings' });
                        }}
                        className="exempt"
                    >
                        <Settings />
                    </button>
                </div>
            </div>
        </>
    );
}

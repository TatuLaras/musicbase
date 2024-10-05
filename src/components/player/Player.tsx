import {
    CSSProperties,
    Dispatch,
    SetStateAction,
    useCallback,
    useEffect,
    useState,
} from 'react';
import ProgressBar from './ProgressBar';
import {
    LineSpace,
    PauseSolid,
    PlaySolid,
    Repeat,
    Settings,
    Shuffle,
    SkipNextSolid,
    SkipPrevSolid,
} from 'iconoir-react';
import { MainViewState } from '../main_view/MainView';
import { Song } from '../../ipc_types';
import { backend } from '../../ipc_commands';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import ImagePlaceholder from '../ImagePlaceholder';

type Props = {
    onMainViewSelected: (state: MainViewState) => void;
    queue: Song[];
    queuePos: number;
    setQueuePos: Dispatch<SetStateAction<number>>;
    shouldReset: boolean;
    setShouldReset: Dispatch<SetStateAction<boolean>>;
};

export default function Player({
    onMainViewSelected,
    queue,
    queuePos,
    setQueuePos,
    shouldReset,
    setShouldReset,
}: Props) {
    const [songStartTime, setSongStartTime] = useState(0);
    const [totalTime, setTotalTime] = useState(0);
    const [currentSong, setCurrentSong] = useState<Song | null>(null);
    const [playing, setPlaying] = useState(false);
    const [time, setTime] = useState(0);
    const [loadedNextSong, setLoadedNextSong] = useState<number | null>(null);
    const [enableQueuePanel, setEnableQueuePanel] = useState(false);

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

    function reset() {
        const current = queueItem();
        const next = queueItem(1);

        if (!current) return;

        // Start immediately playing the first item and queue the next
        backend.playSong(current.song_id!, false);

        if (next) {
            backend.playSong(next.song_id!, true);
            setLoadedNextSong(next.song_id!);
        }

        setSongStartTime(Date.now());
        setTotalTime(current.duration_s ?? 0);
        setCurrentSong(current);
        setPlaying(true);
        setShouldReset(false);
    }

    function nextSong() {
        const current = queueItem(1);
        const next = queueItem(2);

        if (!current) return;

        // Add next song to queue
        if (next) backend.playSong(next.song_id!, true);

        setCurrentSong(current);
        setSongStartTime(Date.now());
        setTotalTime(current.duration_s ?? 0);
        setPlaying(true);
        setCurrentSong(current);
        setQueuePos((old) => old + 1);
    }

    function seek(time: number) {
        setSongStartTime(Date.now() - time * 1000);
        backend.seek(time * 1000);
    }

    // Updates the time and calls nextSong() when needed
    useEffect(() => {
        const interval = setInterval(() => {
            if (!playing) return;

            const elapsedTime = (Date.now() - songStartTime) / 1000;
            console.log(elapsedTime);

            if (elapsedTime >= totalTime) nextSong();
            else setTime(elapsedTime);
        }, 250);
        return () => clearInterval(interval);
    }, [songStartTime, playing, totalTime]);

    return (
        <>
            <div className={`queue-panel ${enableQueuePanel ? 'enable' : ''}`}>
                <div className="content">
                    {queue.map((song, i) => (
                        <div className="song">
                            <div
                                className="cover"
                                style={
                                    {
                                        '--img': `url(${convertFileSrc(song.album?.cover_path ?? '')})`,
                                    } as CSSProperties
                                }
                            >
                                <div className="icon">
                                    <div className="inner">
                                        <PlaySolid />
                                    </div>
                                </div>
                            </div>
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
            <div className="player">
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
                        <button className="shuffle selected">
                            <Shuffle />
                        </button>
                        <button className="prev">
                            <SkipPrevSolid />
                        </button>
                        <button className="play">
                            {playing ? <PauseSolid /> : <PlaySolid />}
                        </button>
                        <button className="next">
                            <SkipNextSolid />
                        </button>
                        <button className="repeat selected">
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
                    <button
                        onClick={() => {
                            onMainViewSelected({ mainViewType: 'settings' });
                        }}
                    >
                        <Settings />
                    </button>
                    <button
                        onClick={() => {
                            setEnableQueuePanel(old => !old);
                        }}
                    >
                        <LineSpace />
                    </button>
                </div>
            </div>
        </>
    );
}

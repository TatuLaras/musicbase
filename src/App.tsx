import { useCallback, useState } from 'react';
import LeftPanel from './components/left_panel/LeftPanel';
import MainView, { MainViewState } from './components/main_view/MainView';
import Player from './components/player/Player';
import { Song } from './ipc_types';

function App() {
    const [mainViewState, setMainViewState] = useState<MainViewState | null>(
        null,
    );

    const [queue, setQueue] = useState<Song[]>([]);
    const [queuePos, setQueuePos] = useState<number>(0);
    const [shouldReset, setShouldReset] = useState<boolean>(false);

    // Add items to the queue
    const onQueue = useCallback((songs: Song[], start: boolean = false) => {
        setQueue((old) =>
            start ? old.splice(queuePos + 1, 0, ...songs) : [...old, ...songs],
        );
    }, []);

    const onPlay = useCallback((newQueue: Song[], newQueuePos: number) => {
        setQueue(newQueue);
        setQueuePos(newQueuePos);
        setShouldReset(true);
    }, []);

    const onMainViewSelected = useCallback((state: MainViewState) => {
        setMainViewState(state);
    }, []);

    return (
        <>
            <div className="root-wrapper">
                <LeftPanel
                    onMainViewSelected={onMainViewSelected}
                    onPlay={onPlay}
                />
                <MainView
                    mainViewState={mainViewState}
                    onQueue={onQueue}
                    onPlay={onPlay}
                    onMainViewSelected={onMainViewSelected}
                />
            </div>
            <Player
                onMainViewSelected={onMainViewSelected}
                queue={queue}
                queuePos={queuePos}
                setQueue={setQueue}
                setQueuePos={setQueuePos}
                shouldReset={shouldReset}
                setShouldReset={setShouldReset}
            />
        </>
    );
}

export default App;

import { useState } from 'react';
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
    function onQueue(songs: Song[], start: boolean = false) {
        setQueue((old) =>
            start ? old.splice(queuePos + 1, 0, ...songs) : [...old, ...songs],
        );
    }

    // Set queue and start playing immediately
    function onPlay(queue: Song[], queuePos: number) {
        setQueue(queue);
        setQueuePos(queuePos);
        setShouldReset(true);
    }

    return (
        <>
            <div className="root-wrapper">
                <LeftPanel
                    onMainViewSelected={(state) => setMainViewState(state)}
                />
                <MainView
                    mainViewState={mainViewState}
                    onQueue={onQueue}
                    onPlay={onPlay}
                />
            </div>
            <Player
                onMainViewSelected={(state) => setMainViewState(state)}
                queue={queue}
                queuePos={queuePos}
                setQueuePos={setQueuePos}
                shouldReset={shouldReset}
                setShouldReset={setShouldReset}
            />
        </>
    );
}

export default App;

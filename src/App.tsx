import { createContext, useEffect, useState } from 'react';
import LeftPanel from './components/left_panel/LeftPanel';
import MainView, {
    MainViewContext,
    MainViewState,
} from './components/main_view/MainView';
import Player from './components/player/Player';
import { Song } from './ipc_types';
import ContextMenu, {
    ContextMenuCallbackContext,
    ContextMenuState,
} from './components/ContextMenu';
import { PlayerContext } from './components/player/Player';
import { listen } from '@tauri-apps/api/event';

interface QueueRequest {
    songs: Song[];
    start: boolean;
}

export const WebserverContext = createContext<string | null>(null);

function App() {
    const [mainViewState, setMainViewState] = useState<MainViewState | null>(
        null,
    );
    const [mainViewForceRefresh, setMainViewForceRefresh] = useState(0);

    const [queue, setQueue] = useState<Song[]>([]);
    const [queuePos, setQueuePos] = useState<number>(0);
    const [shouldReset, setShouldReset] = useState<boolean>(false);

    const [contextMenuState, setContextMenuState] = useState<ContextMenuState>({
        items: [],
        mousePosX: 0,
        mousePosY: 0,
    });
    const [showContextMenu, setShowContextMenu] = useState(false);

    const [queueRequest, setQueueRequest] = useState<QueueRequest | null>(null);
    const [webserverAddress, setWebserverAddress] = useState<string | null>(
        null,
    );

    //  NOTE: We can't read current state from within a function called through PlayerContext,
    //  hence this approach
    useEffect(() => {
        if (!queueRequest) return;

        setQueue((old) => {
            if (!queueRequest.start) return [...old, ...queueRequest.songs];
            return [
                ...old.slice(0, queuePos + 1),
                ...queueRequest.songs,
                ...old.slice(queuePos + 1),
            ];
        });

        setQueueRequest(null);
    }, [queueRequest, queuePos]);

    const onQueue = (songs: Song[], start: boolean = false) => {
        alert('onQueue');
        setQueueRequest({ songs, start });
    };

    const onPlay = (newQueue: Song[], newQueuePos: number) => {
        setQueue(newQueue);
        setQueuePos(newQueuePos);
        setShouldReset(true);
    };

    const onMainViewSelected = (state: MainViewState) => {
        setMainViewState(state);
    };

    const onMainViewUpdate = () => {
        setMainViewForceRefresh((old) => old + 1);
    };

    useEffect(() => {
        // Hide context menu when clicking anywhere
        const click = () => setShowContextMenu(false);
        window.addEventListener('click', click, true);

        // Disable default context menu from showing up
        const context = (e: any) => e.preventDefault();
        window.addEventListener('contextmenu', context);

        const announce = listen('announce', (e) => {
            setWebserverAddress(e.payload as string);
        });

        return () => {
            window.removeEventListener('click', click);
            window.removeEventListener('context', context);
            announce.then((f) => f());
        };
    }, []);

    return (
        <ContextMenuCallbackContext.Provider
            value={(state) => {
                setContextMenuState(state);
                setShowContextMenu(true);
            }}
        >
            <PlayerContext.Provider
                value={{
                    onPlay,
                    onQueue,
                }}
            >
                <MainViewContext.Provider
                    value={{
                        onMainViewSelected,
                        onMainViewUpdate,
                    }}
                >
                    <WebserverContext.Provider value={webserverAddress}>
                        <ContextMenu
                            state={contextMenuState}
                            show={showContextMenu}
                        />
                        <div className="root-wrapper">
                            <LeftPanel />
                            <MainView
                                mainViewState={mainViewState}
                                key={mainViewForceRefresh}
                            />
                        </div>
                        <Player
                            queue={queue}
                            queuePos={queuePos}
                            setQueue={setQueue}
                            setQueuePos={setQueuePos}
                            shouldReset={shouldReset}
                            setShouldReset={setShouldReset}
                        />
                    </WebserverContext.Provider>
                </MainViewContext.Provider>
            </PlayerContext.Provider>
        </ContextMenuCallbackContext.Provider>
    );
}

export default App;

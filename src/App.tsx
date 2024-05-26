import { useState } from 'react';
import LeftPanel from './components/left_panel/LeftPanel';
import MainView, { MainViewState } from './components/main_view/MainView';
import Player from './components/player/Player';

function App() {
    const [mainViewState, setMainViewState] = useState<MainViewState | null>(
        null,
    );

    return (
        <>
            <div className="root-wrapper">
                <LeftPanel
                    onMainViewSelected={(state) => setMainViewState(state)}
                />
                <MainView mainViewState={mainViewState} />
            </div>
            <Player />
        </>
    );
}

export default App;

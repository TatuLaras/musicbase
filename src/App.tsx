import { useState } from 'react';
import LeftPanel from './components/left_panel/LeftPanel';
import MainView, { MainViewState } from './components/main_view/MainView';

function App() {
    const [mainViewState, setMainViewState] = useState<MainViewState | null>(
        null,
    );
    return (
        <div id="root">
            <LeftPanel
                onMainViewSelected={(state) => setMainViewState(state)}
            />
            <MainView mainViewState={mainViewState} />
        </div>
    );
}

export default App;

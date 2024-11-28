import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { IconoirProvider } from 'iconoir-react';

import './styles/normalize.css';
import './styles/left_panel.css';
import './styles/global.css';
import './styles/Poppins.css';
import './styles/panels.css';
import './styles/widgets.css';
import './styles/grid.css';
import './styles/loading.css';
import './styles/list.css';
import './styles/filters.css';
import './styles/search_bar.css';
import './styles/main_view.css';
import './styles/settings_view.css';
import './styles/album.css';
import './styles/player.css';
import './styles/full_screen_view.css';
import './styles/modal.css';
import './styles/context_menu.css';
import './styles/playlist_select.css';
import { invoke } from '@tauri-apps/api';

invoke('init_ipc_socket');

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <IconoirProvider
            iconProps={{
                strokeWidth: 2,
            }}
        >
            <App />
        </IconoirProvider>
    </React.StrictMode>,
);

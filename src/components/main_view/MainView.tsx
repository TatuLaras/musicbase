import { MainViewType } from '../../types';
import AlbumView from './AlbumView';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import SettingsView from './SettingsView';
import { createContext, useContext, useState } from 'react';
import GridView, { GridItem } from '../left_panel/GridView';
import { backend } from '../../ipc_commands';
import Filters, { FilterState } from '../left_panel/Filters';

export interface MainViewState {
    mainViewType: MainViewType;
    id?: number;
}

export interface MainViewContextState {
    onMainViewSelected: (state: MainViewState) => void;
    onMainViewUpdate: () => void;
}

export const MainViewContext = createContext<MainViewContextState>({
    onMainViewSelected: () => {},
    onMainViewUpdate: () => {},
});

type Props = {
    mainViewState: MainViewState | null;
};

export default function MainView({ mainViewState }: Props) {
    const onMainViewSelected = useContext(MainViewContext).onMainViewSelected;

    const [filterState, setFilterState] = useState<FilterState>({
        searchQuery: '',
    });

    function getAlbums(artist_id: number): () => Promise<GridItem[]> {
        return async () => {
            const result = await backend.get_artist_albums(artist_id);

            return result.map((album) => {
                return {
                    id: album.album_id ?? 0,
                    title: album.name,
                    extraInfo: album?.artist?.name ?? '',
                    imageUrl:
                        album.cover_path && album.cover_path.length > 0
                            ? convertFileSrc(album.cover_path)
                            : undefined,
                    onSelected: () =>
                        onMainViewSelected({
                            mainViewType: 'album',
                            id: album.album_id,
                        }),
                };
            });
        };
    }

    const content: { [key: string]: JSX.Element } = {
        album: mainViewState?.id ? <AlbumView id={mainViewState.id} /> : <></>,
        playlist: mainViewState?.id ? (
            <AlbumView id={mainViewState.id} isPlaylist={true} />
        ) : (
            <></>
        ),
        tag: <>{mainViewState?.id}</>,
        settings: <SettingsView />,
        artist: mainViewState?.id ? (
            <>
                <Filters setFilterState={setFilterState} />
                <GridView
                    itemSource={getAlbums(mainViewState?.id)}
                    filterState={filterState}
                />
            </>
        ) : (
            <></>
        ),
    };
    return (
        mainViewState && (
            <div id="main-view">
                {content[mainViewState.mainViewType] ?? <></>}
            </div>
        )
    );
}

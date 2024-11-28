import { convertFileSrc } from '@tauri-apps/api/tauri';
import { backend } from '../ipc_commands';
import Modal from './Modal';
import SearchBar from './SearchBar';
import GridView, { GridItem } from './left_panel/GridView';
import { FilterState } from './left_panel/Filters';
import { useState } from 'react';

interface Props {
    modalState: PlaylistSelectModalState;
    onDone?: (albumId: number | null, playlistId: number | null) => void;
}

export interface PlaylistSelectModalState {
    show: boolean;
    albumId?: number;
}

// A modal that promps a user to select a playlist to add an item to
export default function PlaylistSelectModal({
    modalState,
    onDone = () => {},
}: Props) {
    const [filterState, setFilterState] = useState<FilterState>({
        searchQuery: '',
    });

    const getPlaylists = async (): Promise<GridItem[]> => {
        const playlists = await backend.get_all_playlists();

        return playlists.map((playlist) => {
            const imageUrl =
                playlist.cover_path && playlist.cover_path.length > 0
                    ? convertFileSrc(playlist.cover_path)
                    : undefined;

            return {
                id: playlist.playlist_id ?? 0,
                title: playlist.name,
                extraInfo: playlist.tags.join(', '),
                imageUrl,
                onSelected: () =>
                    onDone(
                        modalState.albumId ?? null,
                        playlist.playlist_id ?? null,
                    ),
            };
        });
    };

    return (
        <Modal
            title="Add to playlist"
            show={modalState.show}
            onCancel={() => onDone(null, null)}
            onConfirm={() => onDone(null, null)}
            className="playlist-select"
            confirmText=""
        >
            <>
                <SearchBar
                    onUpdate={(value) => {
                        setFilterState((old) => {
                            return { ...old, searchQuery: value };
                        });
                    }}
                />
                <GridView
                    listMode={true}
                    itemSource={getPlaylists}
                    filterState={filterState}
                    limit={6}
                />
            </>
        </Modal>
    );
}

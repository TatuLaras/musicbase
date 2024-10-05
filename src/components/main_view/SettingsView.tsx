import { invoke } from '@tauri-apps/api';
import { Plus, TrashSolid } from 'iconoir-react';
import { useEffect, useState } from 'react';
import { Directory } from '../../ipc_types';
import { open } from '@tauri-apps/api/dialog';
import { convertFileSrc } from '@tauri-apps/api/tauri';

type Props = {};

export default function SettingsView({}: Props) {
    const [directories, setDirectories] = useState<Directory[]>([]);
    async function selectDirectory() {
        // invoke('select_directory').then(updateDirectoryList);
        // Open a selection dialog for image files
        const selected = await open({
            directory: true,
            recursive: true,
        });

        if (!selected) return;
    }

    async function updateDirectoryList() {
        setDirectories((await invoke('get_all_directories')) as Directory[]);
    }

    useEffect(() => {
        updateDirectoryList();
    }, []);

    return (
        <div className="settings-view">
            <h2 className="pad-bottom">Music directories</h2>
            <div className="setting">
                <div className="dir-list">
                    {directories.map((dir) => (
                        <div className="item" key={dir.directory_id}>
                            <div className="dir">{dir.path}</div>
                            <button className="delete">
                                <TrashSolid />
                            </button>
                        </div>
                    ))}
                </div>
                <button className="add" onClick={selectDirectory}>
                    <Plus />
                </button>
            </div>

            <h2>Other</h2>
        </div>
    );
}

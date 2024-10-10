import { Plus, TrashSolid } from 'iconoir-react';
import { useEffect, useState } from 'react';
import { Directory } from '../../ipc_types';
import { backend } from '../../ipc_commands';

type Props = {};

export default function SettingsView({}: Props) {
    const [directories, setDirectories] = useState<Directory[]>([]);
    async function selectDirectory() {
        backend.select_directory().then(updateDirectoryList);
    }

    async function updateDirectoryList() {
        setDirectories(await backend.get_all_directories());
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
                            <button
                                className="delete"
                                onClick={() => {
                                    if (!dir.directory_id) return;
                                    backend.delete_directory(dir.directory_id);
                                    updateDirectoryList();
                                }}
                            >
                                <TrashSolid />
                            </button>
                        </div>
                    ))}
                    {directories.length == 0 && (
                        <div className="text-disabled">Ei kansioita.</div>
                    )}
                </div>
                <button className="add" onClick={selectDirectory}>
                    <Plus />
                </button>
            </div>

            <h2>Other</h2>
        </div>
    );
}

import { useState } from 'react';
import { libraryViews } from '../types';
import { capitalize } from '../utils';

type Props<T extends string> = {
    onViewSelected: (view: T) => void;
    options: readonly T[];
};

// To use this we need provide a string type ("hello" | "world" etc.)
// and an array of the options of that type
export default function BulletButtons<T extends string>({
    onViewSelected,
    options,
}: Props<T>) {
    const [selectedBullet, setSelectedBullet] = useState<T>(options[0]);

    function select(view: T) {
        setSelectedBullet(view);
        onViewSelected(view);
    }

    return (
        <div className="bullet-buttons">
            {libraryViews.map((view, i) => (
                <button
                    key={i}
                    className={selectedBullet === view ? 'selected' : ''}
                    onClick={() => select(view as T)}
                >
                    {capitalize(view)}
                </button>
            ))}
        </div>
    );
}

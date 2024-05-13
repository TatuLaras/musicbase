import { MusicNoteSolid } from 'iconoir-react';

export default function Loading() {
    return (
        <div className="loading">
            <div className="larger">
                <MusicNoteSolid />
            </div>
            <div className="smaller">
                <MusicNoteSolid />
            </div>
        </div>
    );
}

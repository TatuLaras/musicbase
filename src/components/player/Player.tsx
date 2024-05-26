import { useState } from 'react';
import ProgressBar from './ProgressBar';
import {
    Play,
    PlaySolid,
    Repeat,
    Shuffle,
    SkipNextSolid,
    SkipPrevSolid,
} from 'iconoir-react';

type Props = {};

export default function Player({}: Props) {
    const [time, setTime] = useState(40);
    return (
        <div className="player">
            <div className="buttons">
                <button className="shuffle selected">
                    <Shuffle />
                </button>
                <button className="prev">
                    <SkipPrevSolid />
                </button>
                <button className="play">
                    <PlaySolid />
                </button>
                <button className="next">
                    <SkipNextSolid />
                </button>
                <button className="repeat selected">
                    <Repeat />
                </button>
            </div>
            <ProgressBar
                totalTime={60 * 60}
                elapsedTime={time}
                onTimeSet={(time: number) => {
                    console.log(time);
                    setTime(time);
                }}
            />
        </div>
    );
}

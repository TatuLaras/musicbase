import { CSSProperties, useEffect, useRef, useState } from 'react';
import { clamp, formatTime } from '../../utils';

type Props = {
    totalTime: number;
    elapsedTime: number;
    onTimeSet: (time: number) => void;
};

export default function ProgressBar({
    totalTime,
    elapsedTime,
    onTimeSet,
}: Props) {
    const [dragging, setDragging] = useState(false);
    const [mousePosX, setMousePosX] = useState(0);
    const [tempTime, setTempTime] = useState<number | undefined>(undefined);
    const progressBarRef = useRef<HTMLDivElement | null>(null);

    function startDrag(_e: MouseEvent) {
        setDragging(true);
        calculateTempTime();
    }

    function stopDrag() {
        setDragging(false);
    }

    function mouseMove(e: MouseEvent) {
        setMousePosX(e.clientX);
    }

    function calculateTempTime() {
        const rect = progressBarRef.current?.getBoundingClientRect();
        if (!rect) return;

        const relX = mousePosX - rect?.left;

        let percentage = relX / rect.width;
        percentage = clamp(percentage, 0, 1);
        setTempTime(totalTime * percentage);
    }

    useEffect(() => {
        window.addEventListener('mouseup', stopDrag);
        window.addEventListener('mousemove', mouseMove);

        return () => {
            window.removeEventListener('mouseup', stopDrag);
            window.removeEventListener('mousemove', mouseMove);
        };
    }, []);

    useEffect(() => {
        if (!dragging) return;

        calculateTempTime();
    }, [mousePosX, dragging, totalTime]);

    useEffect(() => {
        if (tempTime !== undefined) onTimeSet(tempTime);
        setTempTime(undefined);
    }, [dragging]);

    const time = tempTime !== undefined ? tempTime : elapsedTime;
    const progress = `${Math.min(time / totalTime, 1) * 100}%`;

    return (
        <div
            className={`progress-bar ${dragging ? 'dragged' : ''}`}
            style={{ '--progress': progress } as CSSProperties}
        >
            <div className="time-number elapsed">{formatTime(time)}</div>
            <div className="bar" onMouseDown={startDrag} ref={progressBarRef}>
                <div className="outer">
                    <div className="inner">
                        <div className="handle"></div>
                    </div>
                </div>
            </div>
            <div className="time-number total">{formatTime(totalTime)}</div>
        </div>
    );
}

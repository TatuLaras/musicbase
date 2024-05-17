import { CSSProperties, MouseEvent, useEffect, useState } from 'react';
import { AlbumOpen } from 'iconoir-react';
import { clamp } from '../../utils';
import { LibraryView, libraryViews } from '../../types';
import BulletButtons from '../BulletButtons';
import Library from './Library';
import { MainViewState } from '../main_view/MainView';

type Props = {
    onMainViewSelected: (state: MainViewState) => void;
};

export default function LeftPanel({ onMainViewSelected }: Props) {
    const minWidth = 400;
    const maxWidth = clamp(1000, minWidth, window.innerWidth - 100);
    const [dragging, setDragging] = useState(false);
    const [dragStart, setDragStart] = useState(0);
    const [mousePos, setMousePos] = useState(0);
    const [panelWidth, setPanelWidth] = useState(400);
    const [panelWidthOffset, setPanelWidthOffset] = useState(0);
    const [libraryView, setLibraryView] = useState<LibraryView>(
        libraryViews[0],
    );

    function startDrag(e: MouseEvent) {
        setDragging(true);
        setDragStart(e.clientX);
    }

    function stopDrag() {
        setDragging(false);
    }

    function mouseMove(e: MouseEvent) {
        setMousePos(e.clientX);
    }

    useEffect(() => {
        if (!dragging) return;
        const minOffset = minWidth - panelWidth;
        const maxOffset = maxWidth - panelWidth;
        setPanelWidthOffset(clamp(mousePos - dragStart, minOffset, maxOffset));
    }, [dragging, dragStart, mousePos]);

    useEffect(() => {
        if (dragging) return;
        setPanelWidth((old) => old + panelWidthOffset);
        setPanelWidthOffset(0);
    }, [dragging, panelWidthOffset]);

    useEffect(() => {
        window.addEventListener('mouseup', stopDrag);
        window.addEventListener('mousemove', mouseMove);

        return () => {
            window.removeEventListener('mouseup', stopDrag);
            window.removeEventListener('mousemove', mouseMove);
        };
    }, []);

    return (
        <div
            id="left-panel"
            style={
                {
                    '--w': panelWidth + panelWidthOffset + 'px',
                } as CSSProperties
            }
        >
            <div className="content">
                <div className="panel-title">
                    <div className="icon">
                        <AlbumOpen />
                    </div>
                    <div className="text">Library</div>
                </div>
                <BulletButtons<LibraryView>
                    onViewSelected={(view) => setLibraryView(view)}
                    options={libraryViews}
                />
                <Library
                    view={libraryView}
                    onMainViewSelected={onMainViewSelected}
                />
            </div>
            <div className="grabbable" onMouseDown={startDrag}></div>
        </div>
    );
}

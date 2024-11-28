import { useContext, useEffect, useState } from 'react';
import Loading from '../Loading';
import { FilterState } from './Filters';
import Button from '../Button';
import { PlaySolid } from 'iconoir-react';
import SafeImage from '../SafeImage';
import { ContextMenuCallbackContext, ContextMenuItem } from '../ContextMenu';

export interface GridItem {
    id: number;
    title: string;
    extraInfo: string;
    imageUrl?: string;
    onSelected: (id: number) => void;
    contextMenuItems?: ContextMenuItem[];
}

function passesFilter(
    filterState: FilterState | undefined,
    gridItem: GridItem,
): boolean {
    if (!filterState) return true;

    if (
        !gridItem.title
            .toLowerCase()
            .includes(filterState.searchQuery.toLowerCase()) &&
        !gridItem.extraInfo
            .toLowerCase()
            .includes(filterState.searchQuery.toLowerCase())
    )
        return false;

    return true;
}

type Props = {
    itemSource: () => Promise<GridItem[]>;
    circles?: boolean;
    filterState?: FilterState;
    playButton?: boolean;
    onPlayButtonClicked?: (id: number) => void;
    showTutorial?: boolean;
    listMode?: boolean;
    limit?: number;
};

export default function GridView({
    itemSource,
    circles = false,
    filterState = undefined,
    playButton = false,
    onPlayButtonClicked = () => {},
    showTutorial = false,
    listMode = false,
    limit,
}: Props) {
    const [items, setItems] = useState<GridItem[]>([]);
    const [loading, setLoading] = useState(false);

    const openContextMenu = useContext(ContextMenuCallbackContext);

    // Get grid items from the async function provided
    useEffect(() => {
        setLoading(true);
        itemSource().then((res) => {
            setItems(res);
            setLoading(false);
        });
    }, [itemSource]);

    let filteredItems = items.filter((item) => passesFilter(filterState, item));

    if (limit) filteredItems = filteredItems.slice(0, limit);

    return loading ? (
        <Loading />
    ) : (
        <div
            className={`grid-view ${circles ? 'circles' : ''} ${listMode ? 'list-mode' : ''}`}
        >
            {items.length == 0 && showTutorial && (
                <div className="text-disabled">
                    Go to settings to add music directories.
                </div>
            )}
            {filteredItems.map((item) => (
                <div
                    key={item.id}
                    className="item"
                    onClick={() => item.onSelected(item.id)}
                    onContextMenu={(e) => {
                        e.preventDefault();
                        if (item.contextMenuItems)
                            openContextMenu({
                                items: item.contextMenuItems,
                                mousePosX: e.clientX,
                                mousePosY: e.clientY,
                            });
                    }}
                >
                    <SafeImage src={item.imageUrl}>
                        {playButton ? (
                            <div className="play-button">
                                <Button
                                    onClick={(e) => {
                                        onPlayButtonClicked(item.id);
                                        e.stopPropagation();
                                    }}
                                    round={true}
                                    primary={true}
                                >
                                    <PlaySolid />
                                </Button>
                            </div>
                        ) : undefined}
                    </SafeImage>
                    <div className="info">
                        <div className="title">{item.title}</div>
                        <div className="extra-info">{item.extraInfo}</div>
                    </div>
                </div>
            ))}
        </div>
    );
}

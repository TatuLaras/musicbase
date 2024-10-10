import { useEffect, useState } from 'react';
import Loading from '../Loading';
import { FilterState } from './Filters';
import Button from '../Button';
import { PlaySolid } from 'iconoir-react';
import SafeImage from '../SafeImage';

export interface GridItem {
    id: number;
    title: string;
    extraInfo: string;
    imageUrl?: string;
    onSelected: (id: number) => void;
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
};

export default function GridView({
    itemSource,
    circles = false,
    filterState = undefined,
    playButton = false,
    onPlayButtonClicked = () => {},
}: Props) {
    //  TODO: Make custom hook to avoid repetition
    const [items, setItems] = useState<GridItem[]>([]);
    const [loading, setLoading] = useState(false);

    // Get grid items from the async function provided
    useEffect(() => {
        setLoading(true);
        itemSource().then((res) => {
            setItems(res);
            setLoading(false);
        });
    }, [itemSource]);

    return loading ? (
        <Loading />
    ) : (
        <div className={`grid-view ${circles ? 'circles' : ''}`}>
            {items
                .filter((item) => passesFilter(filterState, item))
                .map((item) => (
                    <div
                        key={item.id}
                        className="item"
                        onClick={() => item.onSelected(item.id)}
                        onContextMenu={(e) => {
                            //  TODO: Context menus
                            e.preventDefault();
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
                        <div className="title">{item.title}</div>
                        <div className="extra-info">{item.extraInfo}</div>
                    </div>
                ))}
        </div>
    );
}

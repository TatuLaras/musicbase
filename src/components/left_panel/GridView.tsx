import { useEffect, useState } from 'react';
import Loading from '../Loading';
import { MusicNote } from 'iconoir-react';
import { FilterState } from './Filters';
import ImagePlaceholder from '../ImagePlaceholder';

export interface GridItem {
    id: number;
    title: string;
    extraInfo: string;
    imageUrl?: string;
    onSelected: (id: number) => void;
}

type Props = {
    itemSource: () => Promise<GridItem[]>;
    circles?: boolean;
    filterState?: FilterState;
};

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

export default function GridView({
    itemSource,
    circles = false,
    filterState = undefined,
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
                    >
                        {item.imageUrl ? (
                            <img
                                src={item.imageUrl}
                                alt={`Cover for ${item.title}`}
                                draggable={false}
                            />
                        ) : (
                            !item.imageUrl && <ImagePlaceholder />
                        )}
                        <div className="title">{item.title}</div>
                        <div className="extra-info">{item.extraInfo}</div>
                    </div>
                ))}
        </div>
    );
}

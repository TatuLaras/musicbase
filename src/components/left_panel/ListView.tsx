import { useEffect, useState } from 'react';
import Loading from '../Loading';
import { FilterState } from './Filters';

export interface ListItem {
    id: number;
    title: string;
    onSelected: (id: number) => void;
}

function passesFilter(
    filterState: FilterState | undefined,
    gridItem: ListItem,
): boolean {
    if (!filterState) return true;

    if (
        !gridItem.title
            .toLowerCase()
            .includes(filterState.searchQuery.toLowerCase())
    )
        return false;

    return true;
}

type Props = {
    item_source: () => Promise<ListItem[]>;
    filterState?: FilterState;
};

export default function ListView({
    item_source,
    filterState = undefined,
}: Props) {
    const [items, setItems] = useState<ListItem[]>([]);
    const [loading, setLoading] = useState(false);

    // Get grid items from the async function provided
    useEffect(() => {
        setLoading(true);
        item_source().then((res) => {
            setItems(res);
            setLoading(false);
        });
    }, [item_source]);

    return loading ? (
        <Loading />
    ) : (
        <div className={`list-view`}>
            {items
                .filter((item) => passesFilter(filterState, item))
                .map((item) => (
                    <div
                        key={item.id}
                        className="item"
                        onClick={() => item.onSelected(item.id)}
                    >
                        {item.title}
                    </div>
                ))}
        </div>
    );
}

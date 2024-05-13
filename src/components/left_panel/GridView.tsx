import { useEffect, useState } from 'react';
import Loading from '../Loading';

export interface GridItem {
    id: number;
    title: string;
    extra_info: string;
    image_url: string;
    onSelected: (id: number) => void;
}

type Props = {
    item_source: () => Promise<GridItem[]>;
};

export default function GridView({ item_source }: Props) {
    const [items, setItems] = useState<GridItem[]>([]);
    const [loading, setLoading] = useState(false);
    // Get grid items from the async function provided
    useEffect(() => {
        setLoading(true);
        item_source().then((res) => {
            setItems(res);
            setLoading(false);
        });
    }, [item_source]);

    return loading ? <Loading /> : (
        <div className="grid-view">
            {items.map((item) => (
                <div
                    key={item.id}
                    className="item"
                    onClick={() => item.onSelected(item.id)}
                >
                    <img src={item.image_url} alt={`Image for ${item.title}`} />
                    <div className="title">{item.title}</div>
                    <div className="extra-info">{item.extra_info}</div>
                </div>
            ))}
        </div>
    );
}

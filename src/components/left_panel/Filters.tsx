import { Label, Search, Sort, Xmark } from 'iconoir-react';
import { Dispatch, SetStateAction, useRef, useState } from 'react';

export interface FilterState {
    searchQuery: string;
}

type Props = {
    setFilterState: Dispatch<SetStateAction<FilterState>>;
};

export default function Filters({ setFilterState }: Props) {
    const [searchHasStuff, setSearchHasStuff] = useState(false);
    const searchInputRef = useRef<HTMLInputElement | null>(null);

    return (
        <div className="filters">
            <div className="left">
                <button className="search-icon">
                    <Search />
                </button>
                <input
                    ref={searchInputRef}
                    type="text"
                    name="search"
                    id="search-input"
                    placeholder="Search"
                    onChange={(e) => {
                        setSearchHasStuff(e.target.value.length > 0);

                        setFilterState((old: FilterState) => {
                            return { ...old, searchQuery: e.target.value };
                        });
                    }}
                />
                <button
                    className={`empty ${searchHasStuff ? 'show' : ''}`}
                    onClick={() => {
                        if (searchInputRef.current)
                            searchInputRef.current.value = '';
                        setFilterState((old: FilterState) => {
                            return { ...old, searchQuery: '' };
                        });
                        setSearchHasStuff(false);
                    }}
                >
                    <Xmark />
                </button>
            </div>
            <div className="right">
                <button>
                    <Sort />
                </button>
            </div>
        </div>
    );
}

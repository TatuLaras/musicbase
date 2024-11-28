import { Search, Xmark } from 'iconoir-react';
import { useRef, useState } from 'react';

interface Props {
    onUpdate: (value: string) => void;
}

// A search input field component, calls onUpdate each time it changes
export default function SearchBar({ onUpdate }: Props) {
    const [searchHasStuff, setSearchHasStuff] = useState(false);
    const searchInputRef = useRef<HTMLInputElement | null>(null);

    return (
        <div className="search-bar">
            <button className="search-icon">
                <Search />
            </button>
            <input
                ref={searchInputRef}
                type="text"
                name="search"
                className="search-input"
                placeholder="Search"
                onChange={(e) => {
                    setSearchHasStuff(e.target.value.length > 0);
                    onUpdate(e.target.value);
                }}
            />
            <button
                className={`empty ${searchHasStuff ? 'show' : ''}`}
                onClick={() => {
                    if (searchInputRef.current)
                        searchInputRef.current.value = '';
                    onUpdate('');
                    setSearchHasStuff(false);
                }}
            >
                <Xmark />
            </button>
        </div>
    );
}

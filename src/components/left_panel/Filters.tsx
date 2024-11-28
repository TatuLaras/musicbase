import { Sort } from 'iconoir-react';
import { Dispatch, SetStateAction } from 'react';
import SearchBar from '../SearchBar';

export interface FilterState {
    searchQuery: string;
}

type Props = {
    setFilterState: Dispatch<SetStateAction<FilterState>>;
};

export default function Filters({ setFilterState }: Props) {
    return (
        <div className="filters">
            <div className="left">
                <SearchBar
                    onUpdate={(value) => {
                        setFilterState((old) => {
                            return { ...old, searchQuery: value };
                        });
                    }}
                />
            </div>
            <div className="right">
                <button>
                    <Sort />
                </button>
            </div>
        </div>
    );
}

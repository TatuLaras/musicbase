import { Edit, MusicNote } from 'iconoir-react';

interface Props {
    edit?: boolean;
    onEdit?: () => void;
}

export default function ImagePlaceholder({
    edit = false,
    onEdit = () => {},
}: Props) {
    return (
        <div
            className={`img-placeholder ${edit ? 'edit' : ''}`}
            onClick={() => {
                if (edit) onEdit();
            }}
        >
            {edit ? <Edit /> : <MusicNote />}
        </div>
    );
}

import { MouseEvent } from 'react';

type Props = {
    text: string;
    children?: JSX.Element;
    primary?: boolean;
    onClick?: (e: MouseEvent) => void;
};

export default function Button({
    text,
    children,
    primary = false,
    onClick = (_) => {},
}: Props) {
    return (
        <button
            className={`btn ${primary ? 'primary' : 'secondary'}`}
            onClick={onClick}
        >
            {children}
            <div className="text">{text}</div>
        </button>
    );
}

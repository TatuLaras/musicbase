import { MouseEvent } from 'react';

type Props = {
    text?: string;
    children?: JSX.Element;
    primary?: boolean;
    onClick?: (e: MouseEvent) => void;
    className?: string;
    round?: boolean;
    title?: string;
};

export default function Button({
    text = '',
    children,
    primary = false,
    onClick = () => {},
    className = '',
    round = false,
    title = '',
}: Props) {
    let classes = className;

    if (round) classes += ' round';

    if (primary) classes += ' primary';
    else classes += ' secondary';

    return (
        <button className={`btn ${classes}`} onClick={onClick} title={title}>
            {children}
            {!round && <div className="text">{text}</div>}
        </button>
    );
}

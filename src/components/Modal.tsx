import Button from './Button';

interface Props {
    title: string;
    text?: string;
    show?: boolean;
    children?: JSX.Element;
    onConfirm?: () => void;
    onCancel?: () => void;
    cancelText?: string;
    confirmText?: string;
}

export default function Modal({
    title,
    text,
    show,
    children,
    onConfirm,
    onCancel,
    cancelText = 'Cancel',
    confirmText = 'OK',
}: Props) {
    return (
        <div className={`modal ${show ? 'show' : ''}`} onClick={onCancel}>
            <div className="inner" onClick={(e) => e.stopPropagation()}>
                <div className="title">{title}</div>
                {text && <div className="text">{text}</div>}
                <div className="content">{children}</div>
                <div className="buttons">
                    <Button text={cancelText} onClick={onCancel}></Button>
                    <Button
                        text={confirmText}
                        primary={true}
                        onClick={onConfirm}
                    ></Button>
                </div>
            </div>
        </div>
    );
}

import { useEffect } from 'react';
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
    className?: string;
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
    className = '',
}: Props) {
    useEffect(() => {
        const keydown = (e: any) => {
            if (onConfirm && e.key == 'Enter') onConfirm();
            // if (onCancel && e.key == 'Escape') onCancel();
        };
        window.addEventListener('keydown', keydown);
        return () => window.removeEventListener('keydown', keydown);
    }, [onConfirm, onCancel]);

    return (
        <div
            className={`modal ${show ? 'show' : ''} ${className}`}
            onClick={onCancel}
        >
            <div className="inner" onClick={(e) => e.stopPropagation()}>
                <div className="title">{title}</div>
                {text && <div className="text">{text}</div>}
                <div className="content">{children}</div>
                <div className="buttons">
                    {cancelText.length > 0 && (
                        <Button text={cancelText} onClick={onCancel}></Button>
                    )}
                    {confirmText.length > 0 && (
                        <Button
                            text={confirmText}
                            primary={true}
                            onClick={onConfirm}
                        ></Button>
                    )}
                </div>
            </div>
        </div>
    );
}

import { useEffect, useRef, useState } from 'react';
import Modal from './Modal';

interface Props {
    title: string;
    text?: string;
    show?: boolean;
    inputPlaceholder?: string;
    onDone?: (result: string | null) => void;
}
export default function NamingModal({
    title,
    text,
    show = false,
    inputPlaceholder = '',
    onDone = () => {},
}: Props) {
    const inputRef = useRef<HTMLInputElement | null>(null);
    const [inputValue, setInputValue] = useState('');

    useEffect(() => {
        if (show) setInputValue('');
    }, [show]);
    return (
        <Modal
            title={title}
            text={text}
            show={show}
            onConfirm={() => {
                const inputValue = inputRef.current?.value;
                if (!inputValue || inputValue?.trim().length == 0) onDone(null);
                else onDone(inputValue);
            }}
            onCancel={() => {
                onDone(null);
            }}
        >
            <input
                type="text"
                ref={inputRef}
                placeholder={inputPlaceholder}
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
            />
        </Modal>
    );
}

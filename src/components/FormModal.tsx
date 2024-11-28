import { useCallback, useEffect, useState } from 'react';
import Modal from './Modal';
import { capitalize } from '../utils';

export interface FormField {
    name: string;
    label?: string;
    required?: boolean;
    value?: string;
}

interface Props<T> {
    title: string;
    fields: FormField[];
    text?: string;
    show?: boolean;
    onDone?: (result: T | null) => void;
}

export default function FormModal<T>({
    title,
    fields,
    text,
    show = false,
    onDone = () => {},
}: Props<T>) {
    const [fieldValues, setFieldValues] = useState<{ [name: string]: string }>(
        {},
    );
    const [nagAboutField, setNagAboutField] = useState<string | null>(null);

    const submit = useCallback(() => {
        // Check that all required fields are filled
        for (let field of fields) {
            if (
                !(field.name in fieldValues) ||
                (field.required && fieldValues[field.name].trim().length == 0)
            ) {
                setNagAboutField(field.name);
                return;
            }
        }

        onDone(fieldValues as T);
    }, [fieldValues]);

    useEffect(() => {
        if (!show) return;
        let vals: { [name: string]: any } = {};
        fields.forEach((field) => {
            vals[field.name] = field.value ?? '';
        });
        setFieldValues(vals);
        setNagAboutField(null);
    }, [fields, show]);

    return (
        <Modal
            title={title}
            text={text}
            show={show}
            onConfirm={submit}
            onCancel={() => onDone(null)}
        >
            <>
                {fields.map((field) => (
                    <div
                        className={`field ${nagAboutField === field.name ? 'nag' : ''}`}
                        key={field.name}
                    >
                        <label htmlFor={field.name}>
                            {field.label ?? capitalize(field.name)}:
                        </label>
                        <input
                            type="text"
                            value={
                                !fieldValues[field.name] ||
                                fieldValues[field.name].length == 0
                                    ? (field.value ?? '')
                                    : fieldValues[field.name]
                            }
                            id={field.name}
                            placeholder={field.label ?? capitalize(field.name)}
                            onChange={(e) =>
                                setFieldValues((old) => {
                                    const newFieldValues = { ...old };
                                    newFieldValues[field.name] = e.target.value;
                                    return newFieldValues;
                                })
                            }
                        />
                    </div>
                ))}
            </>
        </Modal>
    );
}

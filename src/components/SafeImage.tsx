import { CSSProperties } from 'react';
import ImagePlaceholder from './ImagePlaceholder';

interface Props {
    src?: string;
    children?: JSX.Element;
}
export default function SafeImage({ src, children }: Props) {
    return (
        <div className="safe-image">
            <ImagePlaceholder />
            <div
                className="image"
                style={{ '--img': `url(${src})` } as CSSProperties}
            ></div>
            {children}
        </div>
    );
}

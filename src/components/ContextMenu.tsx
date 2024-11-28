import { NavArrowRight } from 'iconoir-react';
import { CSSProperties, createContext } from 'react';

//  NOTE:  Only one level of subitems supported.
export interface ContextMenuItem {
    label: string;
    onClick?: () => void;
    subitems?: ContextMenuItem[];
}

export interface ContextMenuState {
    items: ContextMenuItem[];
    mousePosX: number;
    mousePosY: number;
}

// We're using a context to provide a callback to all components,
// which activates the right-click menu
export const ContextMenuCallbackContext = createContext<
    (state: ContextMenuState) => void
>(() => {});

interface Props {
    state: ContextMenuState;
    show?: boolean;
}

//  TODO: Subitems
export default function ContextMenu({ state, show = false }: Props) {
    const style = {
        '--x': `${state.mousePosX}px`,
        '--y': `${state.mousePosY}px`,
    } as CSSProperties;

    // Recursively generate context menu
    const contextMenu = (items: ContextMenuItem[]): JSX.Element => {
        return (
            <>
                {items.map((item, i) => (
                    <div className="item" key={i} onClick={item.onClick}>
                        <div className="label">{item.label}</div>
                        {item.subitems && (
                            <>
                                <NavArrowRight />
                                <div className="subitems">
                                    {contextMenu(item.subitems)}
                                </div>
                            </>
                        )}
                    </div>
                ))}
            </>
        );
    };

    return (
        <div className={`context-menu ${show ? 'show' : ''}`} style={style}>
            {contextMenu(state.items)}
        </div>
    );
}

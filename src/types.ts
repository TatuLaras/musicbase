// We do this weird thing to get the views as a list and a type at the same time
export const libraryViews = ['albums', 'artists', 'playlists', 'tags'] as const;
export type LibraryView = (typeof libraryViews)[number];

export type MainViewType = 'settings' | 'album' | 'artist' | 'playlist' | 'tag';

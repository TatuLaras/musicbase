export interface Artist {
    artist_id?: number;
    name: string;
    artist_image_path?: string;
}

export interface Album {
    album_id?: number;
    name: string;
    artist?: Artist;
    cover_path?: string;
    cover_path_small?: string;
    cover_path_tiny?: string;
    year?: number;
    total_tracks?: number;
    total_discs?: number;
}

export interface Song {
    song_id?: number;
    name: string;
    file_path: string;
    track?: number;
    disc?: number;
    duration_s?: number;
    quality: number; // Quality;
    genre?: string;
    artist?: Artist;
    album?: Album;
}

export interface Playlist {
    playlist_id?: number;
    name: string;
    desc: string;
    cover_path?: string;
    cover_path_small?: string;
    cover_path_tiny?: string;
    created?: string;
    tags: string[];
}

export interface Tag {
    tag_id?: number;
    name: string;
}

export interface Directory {
    directory_id?: number;
    path: string;
}

export interface Duration {
    secs: number;
    nanos: number;
}

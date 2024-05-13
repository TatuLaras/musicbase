interface Artist {
    artist_id?: number;
    name: string;
}

interface Album {
    album_id?: number;
    name: string;
    artist?: Artist;
    cover_path?: string;
    year?: number;
    total_tracks?: number;
    total_discs?: number;
}

interface Song {
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

interface Playlist {
    playlist_id?: number;
    name: string;
    desc: string;
    cover_path?: string;
    created?: string;
    tags: string[];
}

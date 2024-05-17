use once_cell::sync::Lazy;

use crate::{
    models::{
        base_metadata::{Album, Artist, Song},
        user_generated::{Playlist, PlaylistSong, Tag},
        Quality,
    },
    param::{asc, desc, eq, gt, gte, like, lt, search, Order},
    test_utils::get_mock_db,
};

static SAMPLE_ARTISTS: Lazy<[Artist; 4]> = Lazy::new(|| {
    [
        Artist {
            artist_id: None,
            name: "Björk".into(),
            artist_image_path: None,
        },
        Artist {
            artist_id: None,
            name: "Anssi Kela".into(),
            artist_image_path: None,
        },
        Artist {
            artist_id: Some(123),
            name: "Radiohead".into(),
            artist_image_path: None,
        },
        Artist {
            artist_id: Some(123),
            name: "SQL Injector '`\"".into(),
            artist_image_path: None,
        },
    ]
});

static SAMPLE_ALBUMS: Lazy<[Album; 3]> = Lazy::new(|| {
    [
        Album {
            album_id: None,
            name: "Suuria Kuvioita".into(),
            artist: Some(SAMPLE_ARTISTS[0].clone()),
            cover_path: None,
            year: Some(2003),
            total_tracks: Some(10),
            total_discs: Some(1),
        },
        Album {
            album_id: None,
            name: "Homogenic".into(),
            artist: Some(SAMPLE_ARTISTS[1].clone()),
            cover_path: Some("path/to/cover/".into()),
            year: Some(1997),
            total_tracks: Some(10),
            total_discs: Some(1),
        },
        Album {
            album_id: None,
            name: "Empty album".into(),
            artist: None,
            cover_path: None,
            year: None,
            total_tracks: None,
            total_discs: None,
        },
    ]
});

static SAMPLE_SONGS: Lazy<[Song; 4]> = Lazy::new(|| {
    [
        Song {
            song_id: None,
            name: "Suuria Kuvioita".into(),
            track: Some(7),
            duration_s: Some(220.0),
            quality: Quality::Lossless,
            genre: Some("Rock".into()),
            artist: Some(SAMPLE_ARTISTS[1].clone()),
            album: Some(SAMPLE_ALBUMS[0].clone()),
            file_path: "/path/to/song/file".into(),
            disc: Some(1),
        },
        Song {
            song_id: None,
            name: "Empty song".into(),
            track: None,
            duration_s: None,
            quality: Quality::Lossy,
            genre: None,
            artist: None,
            album: None,
            file_path: "/path/to/other/song".into(),
            disc: None,
        },
        Song {
            song_id: None,
            name: "Like the wind".into(),
            track: Some(1),
            duration_s: Some(190.0),
            quality: Quality::Lossy,
            genre: Some("Lost wave".into()),
            artist: None,
            album: None,
            file_path: "/path/".into(),
            disc: Some(1),
        },
        Song {
            song_id: None,
            name: "Bachelorette".into(),
            track: Some(4),
            duration_s: Some(220.0),
            quality: Quality::Lossless,
            genre: Some("Art pop".into()),
            artist: Some(SAMPLE_ARTISTS[0].clone()),
            album: Some(SAMPLE_ALBUMS[1].clone()),
            file_path: "/path/to/song/file/bachelorette.flac".into(),
            disc: Some(1),
        },
    ]
});

static SAMPLE_PLAYLISTS: Lazy<[Playlist; 3]> = Lazy::new(|| {
    [
        Playlist {
            playlist_id: None,
            name: "Alternative rock".into(),
            desc: "Rock that is alternative".into(),
            cover_path: Some("/path/to/cover".into()),
            created: None,
            tags: vec!["chill".into(), "summer".into()],
        },
        Playlist {
            playlist_id: None,
            name: "Flipping rocking tunes".into(),
            desc: "Tunes that are flipping rocking".into(),
            cover_path: None,
            created: None,
            tags: Vec::new(),
        },
        Playlist {
            playlist_id: None,
            name: "Bangers".into(),
            desc: "Yipii".into(),
            cover_path: None,
            created: None,
            tags: vec!["summer".into(), "Epic".into()],
        },
    ]
});

static SAMPLE_TAGS: Lazy<[Tag; 3]> = Lazy::new(|| {
    [
        Tag {
            tag_id: None,
            name: "Summer".into(),
        },
        Tag {
            tag_id: None,
            name: "Chill".into(),
        },
        Tag {
            tag_id: None,
            name: "Epic".into(),
        },
    ]
});

#[test]
fn insert_and_retrieve_artist() {
    let db = get_mock_db();

    for mut artist in SAMPLE_ARTISTS.clone().into_iter() {
        db.insert(&mut artist).expect("Insert");
    }
    let db_artists = db
        .get_all::<Artist>(Order::Asc("artist.artist_id".to_string()))
        .expect("Retrieval");

    for (i, db_artist) in db_artists.clone().into_iter().enumerate() {
        assert_eq!(SAMPLE_ARTISTS[i].name, db_artists[i].name);
        if let Some(id) = db_artist.artist_id {
            assert!(id > 0);
        } else {
            panic!("No ID");
        }
    }
}

#[test]
fn last_insert_row_id_artist() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut artist in SAMPLE_ARTISTS.clone().into_iter() {
        counter += 1;
        db.insert(&mut artist).expect("Insert");
        assert_eq!(artist.artist_id.unwrap(), counter);
    }

    let last_id = db.last_id().expect("Last ID retrieval");
    assert_eq!(last_id as usize, SAMPLE_ARTISTS.len());
}

#[test]
fn last_insert_row_id_playlist() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut playlist in SAMPLE_PLAYLISTS.clone().into_iter() {
        counter += 1;
        db.insert(&mut playlist).expect("Insert");
        assert_eq!(playlist.playlist_id.unwrap(), counter);
    }

    let last_id = db.last_id().expect("Last ID retrieval");
    assert_eq!(last_id as usize, SAMPLE_PLAYLISTS.len());
}

#[test]
fn last_insert_row_id_album() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut album in SAMPLE_ALBUMS.clone().into_iter() {
        counter += 1;
        db.insert(&mut album).expect("Insert");
        assert_eq!(album.album_id.unwrap(), counter);
    }

    let last_id = db.last_id().expect("Last ID retrieval");
    assert_eq!(last_id as usize, SAMPLE_ALBUMS.len());
}

#[test]
fn last_insert_row_id_song() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut song in SAMPLE_SONGS.clone().into_iter() {
        counter += 1;
        db.insert(&mut song).expect("Insert");
        assert_eq!(song.song_id.unwrap(), counter);
    }

    let last_id = db.last_id().expect("Last ID retrieval");
    assert_eq!(last_id as usize, SAMPLE_SONGS.len());
}

#[test]
#[should_panic]
fn empty_artist() {
    let db = get_mock_db();
    db.insert(&mut Artist {
        artist_id: None,
        name: "".into(),
        artist_image_path: None,
    })
    .expect("Expected error");
}

#[test]
#[should_panic]
fn empty_album() {
    let db = get_mock_db();
    db.insert(&mut Album {
        album_id: None,
        name: "".into(),
        artist: None,
        cover_path: None,
        year: None,
        total_tracks: None,
        total_discs: None,
    })
    .expect("Expected error");
}

#[test]
#[should_panic]
fn empty_song() {
    let db = get_mock_db();
    db.insert(&mut Song {
        song_id: None,
        name: "".into(),
        file_path: "".into(),
        track: None,
        disc: None,
        duration_s: None,
        quality: Quality::Lossy,
        genre: None,
        artist: None,
        album: None,
    })
    .expect("Expected error");
}

#[test]
fn artist_exists() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut artist in SAMPLE_ARTISTS.clone().into_iter() {
        counter += 1;
        db.insert(&mut artist).expect("Insert");
        let mut artist_without_id = Artist {
            artist_id: None,
            name: artist.name,
            artist_image_path: None,
        };
        assert!(db.exists(&mut artist_without_id).expect("Exists check"));
        if let Some(id) = artist_without_id.artist_id {
            assert_eq!(counter, id);
        } else {
            panic!("No ID");
        }
    }

    assert!(!db
        .exists(&mut Artist {
            artist_id: None,
            name: "Does not exist".into(),
            artist_image_path: None,
        })
        .expect("Exists check"));
}

#[test]
fn insert_and_retrieve_album() {
    let db = get_mock_db();

    for mut album in SAMPLE_ALBUMS.clone().into_iter() {
        for _ in 0..2 {
            if let Some(artist) = &mut album.artist {
                db.insert(artist).expect("Artist insert");
            }
            db.insert(&mut album).expect("Album insert");
        }
    }

    let db_albums = db.get_all::<Album>(Order::Default).expect("Retrieval");

    for (i, db_album) in db_albums.clone().into_iter().enumerate() {
        assert_eq!(SAMPLE_ALBUMS[i].name, db_album.name);
        assert_eq!(SAMPLE_ALBUMS[i].total_tracks, db_album.total_tracks);
        assert_eq!(SAMPLE_ALBUMS[i].total_discs, db_album.total_discs);
        assert_eq!(SAMPLE_ALBUMS[i].cover_path, db_album.cover_path);
        assert_eq!(SAMPLE_ALBUMS[i].year, db_album.year);
        assert_eq!(SAMPLE_ALBUMS[i].artist.is_some(), db_album.artist.is_some());

        if let Some(sample_artist) = &SAMPLE_ALBUMS[i].artist {
            if let Some(artist) = &db_albums[i].artist {
                assert_eq!(sample_artist.name, artist.name);
            }
        }

        if let Some(id) = db_album.album_id {
            assert!(id > 0);
        } else {
            panic!("No ID");
        }
    }
}

#[test]
fn album_exists() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut album in SAMPLE_ALBUMS.clone().into_iter() {
        counter += 1;
        if let Some(artist) = &mut album.artist {
            db.insert(artist).expect("Artist insert");
        }
        db.insert(&mut album).expect("Insert");
        assert!(db.exists(&mut album).expect("Exists check"));
        if let Some(id) = album.album_id {
            assert_eq!(counter, id);
        } else {
            panic!("No ID");
        }
    }

    assert!(!db
        .exists(&mut Album {
            album_id: None,
            name: "Does not exist".to_string(),
            artist: None,
            cover_path: None,
            year: None,
            total_tracks: None,
            total_discs: None,
        })
        .expect("Exists check"));
}

#[test]
fn insert_and_retrieve_song() {
    let db = get_mock_db();

    for mut song in SAMPLE_SONGS.clone().into_iter() {
        for _ in 0..2 {
            if let Some(artist) = &mut song.artist {
                db.insert(artist).unwrap();
            }

            if let Some(album) = &mut song.album {
                if let Some(album_artist) = &mut album.artist {
                    db.insert(album_artist).unwrap();
                }
                db.insert(album).unwrap();
            }
            db.insert(&mut song).unwrap();
        }
    }

    let db_songs = db.get_all::<Song>(Order::Default).unwrap();

    // The comparing of nested fields gets pretty messy... this is just a test function tho
    // so no worries
    for (i, sample_song) in SAMPLE_SONGS.clone().into_iter().enumerate() {
        let db_song = &db_songs[i];

        assert_eq!(db_song.name, sample_song.name);
        assert_eq!(db_song.duration_s, sample_song.duration_s);
        assert_eq!(db_song.quality, sample_song.quality);
        assert_eq!(db_song.genre, sample_song.genre);
        assert_eq!(db_song.album.is_some(), sample_song.album.is_some());

        if let Some(db_album) = &db_song.album {
            if let Some(sample_album) = &sample_song.album {
                assert_eq!(db_album.name, sample_album.name);
                assert_eq!(db_album.cover_path, sample_album.cover_path);
                assert_eq!(db_album.year, sample_album.year);
                assert_eq!(db_album.total_tracks, sample_album.total_tracks);
                assert_eq!(db_album.total_discs, sample_album.total_discs);
                assert_eq!(db_album.artist.is_some(), sample_album.artist.is_some());

                if let Some(sample_artist) = &db_album.artist {
                    if let Some(db_artist) = &sample_album.artist {
                        assert_eq!(sample_artist.name, db_artist.name);
                    }
                }
            }
        }

        assert_eq!(db_song.artist.is_some(), sample_song.artist.is_some());

        if let Some(sample_artist) = &db_song.artist {
            if let Some(artist) = &sample_song.artist {
                assert_eq!(sample_artist.name, artist.name);
            }
        }
    }
}

#[test]
fn insert_full_song() {
    let db = get_mock_db();

    for mut song in SAMPLE_SONGS.clone().into_iter() {
        for _ in 0..2 {
            db.insert_full(&mut song).expect("Song full insert");
        }
    }

    let db_songs = db.get_all::<Song>(Order::Default).expect("Retrieval");

    for (i, sample_song) in SAMPLE_SONGS.clone().into_iter().enumerate() {
        let db_song = &db_songs[i];

        assert_eq!(db_song.name, sample_song.name);
        assert_eq!(db_song.duration_s, sample_song.duration_s);
        assert_eq!(db_song.quality, sample_song.quality);
        assert_eq!(db_song.genre, sample_song.genre);
        assert_eq!(db_song.album.is_some(), sample_song.album.is_some());

        if let Some(db_album) = &db_song.album {
            if let Some(sample_album) = &sample_song.album {
                assert_eq!(db_album.name, sample_album.name);
                assert_eq!(db_album.cover_path, sample_album.cover_path);
                assert_eq!(db_album.year, sample_album.year);
                assert_eq!(db_album.total_tracks, sample_album.total_tracks);
                assert_eq!(db_album.total_discs, sample_album.total_discs);
                assert_eq!(db_album.artist.is_some(), sample_album.artist.is_some());

                if let Some(sample_artist) = &db_album.artist {
                    if let Some(db_artist) = &sample_album.artist {
                        assert_eq!(sample_artist.name, db_artist.name);
                    }
                }
            }
        }

        assert_eq!(db_song.artist.is_some(), sample_song.artist.is_some());

        if let Some(sample_artist) = &db_song.artist {
            if let Some(artist) = &sample_song.artist {
                assert_eq!(sample_artist.name, artist.name);
            }
        }
    }
}

#[test]
fn song_exists() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut song in SAMPLE_SONGS.clone().into_iter() {
        counter += 1;
        db.insert_full(&mut song).expect("Song full insert");
        let mut song_to_search = Song {
            song_id: None,
            name: "".into(),
            file_path: song.file_path,
            track: None,
            disc: None,
            duration_s: None,
            quality: song.quality,
            genre: None,
            artist: None,
            album: None,
        };
        assert!(db.exists(&mut song_to_search).expect("Exists check"));
        assert_eq!(song_to_search.song_id.expect(""), counter);
    }

    assert!(!db
        .exists(&mut Album {
            album_id: None,
            name: "Does not exist".to_string(),
            artist: None,
            cover_path: None,
            year: None,
            total_tracks: None,
            total_discs: None,
        })
        .expect("Exists check"));
}

#[test]
fn double_insert_id_artist() {
    let db = get_mock_db();
    for mut artist in SAMPLE_ARTISTS.clone().into_iter() {
        db.insert(&mut artist).expect("First insert");
    }

    let mut test_artist = SAMPLE_ARTISTS[1].clone();
    assert!(test_artist.artist_id.is_none());
    db.insert(&mut test_artist).expect("Second insert");
    assert_eq!(test_artist.artist_id, Some(2));
}

#[test]
fn double_insert_id_album() {
    let db = get_mock_db();
    for mut album in SAMPLE_ALBUMS.clone().into_iter() {
        db.insert(&mut album).expect("First insert");
    }

    let mut test_album = SAMPLE_ALBUMS[1].clone();
    assert!(test_album.album_id.is_none());
    db.insert(&mut test_album).expect("Second insert");
    assert_eq!(test_album.album_id, Some(2));
}

#[test]
fn double_insert_id_songs() {
    let db = get_mock_db();
    for mut song in SAMPLE_SONGS.clone().into_iter() {
        db.insert_full(&mut song).expect("First insert");
    }

    let mut test_song = SAMPLE_SONGS[1].clone();
    assert!(test_song.song_id.is_none());
    db.insert_full(&mut test_song).expect("Second insert");
    assert_eq!(test_song.song_id, Some(2));
}

#[test]
fn sorting() {
    let db = get_mock_db();
    let mut artist_names: Vec<String> = Vec::new();

    for mut song in SAMPLE_SONGS.clone().into_iter() {
        db.insert_full(&mut song).expect("Song insert");
    }

    for mut artist in SAMPLE_ARTISTS.clone().into_iter() {
        db.insert(&mut artist).expect("Artist insert");
        artist_names.push(artist.name);
    }

    for mut album in SAMPLE_ALBUMS.clone().into_iter() {
        db.insert_full(&mut album).expect("Album insert");
    }

    // Default orders by artist_id ascending
    let db_songs_default = db.get_all::<Song>(Order::Default).unwrap();
    let db_artist_name = db.get_all::<Artist>(desc("artist.name")).unwrap();
    let db_album_artist_id = db.get_all::<Album>(asc("album.artist_id")).unwrap();
    let db_artist_default = db.get_all::<Artist>(Order::Default).unwrap();
    let db_album_default = db.get_all::<Album>(Order::Default).unwrap();

    let mut prev = 0;
    for song in &db_songs_default {
        if let Some(id) = song.song_id {
            assert!(id > prev);
            prev = id;
        } else {
            panic!("No ID");
        }
    }

    artist_names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    artist_names.reverse();

    for (i, artist) in db_artist_name.clone().into_iter().enumerate() {
        assert_eq!(artist_names[i], artist.name);
    }

    let mut prev = 0;
    for album in &db_album_artist_id {
        if let Some(artist) = &album.artist {
            if let Some(id) = artist.artist_id {
                assert!(id > prev);
                prev = id;
            } else {
                panic!("No ID");
            }
        }
    }

    let mut prev = 0;
    for artist in &db_artist_default {
        if let Some(id) = artist.artist_id {
            assert!(id > prev);
            prev = id;
        } else {
            panic!("No ID");
        }
    }

    let mut prev = 0;
    for album in &db_album_default {
        if let Some(id) = album.album_id {
            assert!(id > prev);
            prev = id;
        } else {
            panic!("No ID");
        }
    }
}

#[test]
fn insert_and_retrieve_playlist() {
    let db = get_mock_db();

    let mut counter = 0;
    for mut playlist in SAMPLE_PLAYLISTS.clone().into_iter() {
        for _ in 0..2 {
            counter += 1;
            db.insert_full(&mut playlist).unwrap();
            assert_eq!(playlist.playlist_id.unwrap(), counter);
        }
    }

    let db_playlists = db.get_all::<Playlist>(Order::Default).unwrap();

    for (i, sample_playlist) in SAMPLE_PLAYLISTS.clone().into_iter().enumerate() {
        let base = i * 2;

        for j in 0..2 {
            let index = base + j;
            let db_playlist = &db_playlists[index];
            assert_eq!(db_playlist.name, sample_playlist.name);
            assert_eq!(db_playlist.desc, sample_playlist.desc);
            assert_eq!(db_playlist.cover_path, sample_playlist.cover_path);
            assert!(db_playlist.created.clone().unwrap().len() > 0);
            assert_eq!(db_playlist.tags, sample_playlist.tags);
            assert!(db_playlist.playlist_id.unwrap() > 0);
        }
    }
}

#[test]
fn playlist_song_exists() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut playlist in SAMPLE_PLAYLISTS.clone().into_iter() {
        counter += 1;
        db.insert(&mut playlist).expect("Insert");
        assert_eq!(counter, playlist.playlist_id.unwrap());
        for mut song in SAMPLE_SONGS.clone().into_iter() {
            db.insert(&mut song).unwrap();

            let mut playlist_song = PlaylistSong {
                playlist_song_id: None,
                song_id: song.song_id.unwrap(),
                playlist_id: playlist.playlist_id.unwrap(),
            };

            db.insert(&mut playlist_song).unwrap();
            playlist_song.playlist_song_id.unwrap();

            let mut playlist_song = PlaylistSong {
                playlist_song_id: None,
                song_id: playlist_song.song_id,
                playlist_id: playlist_song.playlist_id,
            };

            assert!(db.exists(&mut playlist_song).expect("Exists check"));
            playlist_song.playlist_song_id.unwrap();
        }
    }

    assert!(!db
        .exists(&mut Playlist {
            playlist_id: None,
            name: "Does not exist".into(),
            desc: "Does not exist".into(),
            cover_path: None,
            created: None,
            tags: Vec::new(),
        })
        .expect("Exists check"));
}

#[test]
fn empty_exists() {
    let db = get_mock_db();
    for mut artist in SAMPLE_ARTISTS.clone().into_iter() {
        db.insert(&mut artist).expect("Insert");
    }
    assert!(!db
        .exists(&mut Artist {
            artist_id: None,
            name: "".into(),
            artist_image_path: None,
        })
        .unwrap());
}

#[test]
fn get_by() {
    let db = get_mock_db();
    for mut song in SAMPLE_SONGS.clone().into_iter() {
        db.insert_full(&mut song).unwrap();
    }

    for mut playlist in SAMPLE_PLAYLISTS.clone().into_iter() {
        db.insert(&mut playlist).unwrap();
    }

    assert_eq!(
        db.get_by::<Song>(eq("genre", "Lost wave"), Order::Default)
            .unwrap()[0]
            .name,
        "Like the wind".to_string()
    );
    assert_eq!(
        db.get_by::<Song>(eq("artist.name", "Anssi Kela"), Order::Default)
            .unwrap()[0]
            .name,
        "Suuria Kuvioita".to_string()
    );
    assert_eq!(
        db.get_by::<Song>(gte("track", "5"), Order::Default)
            .unwrap()[0]
            .song_id
            .unwrap(),
        1
    );

    assert_eq!(
        db.get_by::<Album>(lt("year", "2000"), Order::Default)
            .unwrap()[0]
            .album_id
            .unwrap(),
        2
    );

    assert_eq!(
        db.get_by::<Artist>(gt("album.artist_id", "1"), Order::Default)
            .unwrap()[0]
            .name,
        "Anssi Kela".to_string()
    );

    assert_eq!(
        db.get_by::<Playlist>(like("desc", "flipping"), Order::Default)
            .unwrap()[0]
            .name,
        "Flipping rocking tunes".to_string()
    );

    assert_eq!(
        db.get_by::<Playlist>(like("desc", "FLippiNG"), Order::Default)
            .unwrap()[0]
            .name,
        "Flipping rocking tunes".to_string()
    );

    assert_eq!(
        db.get_by::<Playlist>(search("desc", "FLIPPING"), Order::Default)
            .unwrap()[0]
            .name,
        "Flipping rocking tunes".to_string()
    );

    assert_eq!(
        db.get_by::<Song>(
            eq("quality", &(Quality::Lossless as i64).to_string()),
            asc("song.name"),
        )
        .unwrap()[0]
            .name,
        "Bachelorette".to_string()
    );
}

#[test]
fn insert_and_retrieve_tag() {
    let db = get_mock_db();

    let mut counter = 0;
    for mut tag in SAMPLE_TAGS.clone().into_iter() {
        counter += 1;
        for _ in 0..2 {
            db.insert(&mut tag).unwrap();
            assert_eq!(tag.tag_id.unwrap(), counter);
        }
    }

    let db_tags = db.get_all::<Tag>(Order::Default).unwrap();

    for (i, sample_tag) in SAMPLE_TAGS.clone().into_iter().enumerate() {
        let db_playlist = &db_tags[i];

        assert_eq!(db_playlist.name, sample_tag.name);
        assert_eq!(db_playlist.tag_id.unwrap() - 1, i as i64);
    }
}

#[test]
fn tag_exists() {
    let db = get_mock_db();

    let mut counter = 0;
    for mut tag in SAMPLE_TAGS.clone().into_iter() {
        counter += 1;
        db.insert(&mut tag).expect("Insert");
        let mut tag_without_id = Tag {
            tag_id: None,
            name: tag.name,
        };
        assert!(db.exists(&mut tag_without_id).expect("Exists check"));
        if let Some(id) = tag_without_id.tag_id {
            assert_eq!(counter, id);
        } else {
            panic!("No ID");
        }
    }

    assert!(!db
        .exists(&mut Tag {
            tag_id: None,
            name: "Does not exist".into(),
        })
        .expect("Exists check"));
}

#[test]
fn album_songs() {
    let db = get_mock_db();

    for mut song in SAMPLE_SONGS.clone().into_iter() {
        db.insert_full(&mut song).unwrap();
    }

    let res = db
        .get_by::<Song>(eq("song.album_id", "1"), Order::Asc("track".to_string()))
        .unwrap();

    assert!(res.len() > 0);
    assert_eq!(
        res[0].album.clone().unwrap().name,
        SAMPLE_ALBUMS[0].clone().name
    );
}

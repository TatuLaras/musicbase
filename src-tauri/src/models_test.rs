use core::panic;

use once_cell::sync::Lazy;

use crate::{
    database::{self, ConnectionWrapper},
    models::{Album, Artist, Quality, Song},
};

static SAMPLE_ARTISTS: Lazy<[Artist; 4]> = Lazy::new(|| {
    [
        Artist {
            id: None,
            name: "Björk".into(),
        },
        Artist {
            id: None,
            name: "Anssi Kela".into(),
        },
        Artist {
            id: Some(123),
            name: "Radiohead".into(),
        },
        Artist {
            id: Some(123),
            name: "SQL Injector '`\"".into(),
        },
    ]
});

static SAMPLE_ALBUMS: Lazy<[Album; 3]> = Lazy::new(|| {
    [
        Album {
            id: None,
            name: "Suuria Kuvioita".into(),
            artist: Some(SAMPLE_ARTISTS[0].clone()),
            cover_path: None,
            year: Some(2003),
            total_tracks: Some(10),
            total_discs: Some(1),
        },
        Album {
            id: None,
            name: "Homogenic".into(),
            artist: Some(SAMPLE_ARTISTS[1].clone()),
            cover_path: Some("path/to/cover/".into()),
            year: Some(1997),
            total_tracks: Some(10),
            total_discs: Some(1),
        },
        Album {
            id: None,
            name: "Empty album".into(),
            artist: None,
            cover_path: None,
            year: None,
            total_tracks: None,
            total_discs: None,
        },
    ]
});

static SAMPLE_SONGS: Lazy<[Song; 3]> = Lazy::new(|| {
    [
        Song {
            id: None,
            name: "Suuria Kuvioita".into(),
            track: Some(7),
            duration_s: Some(220.0),
            quality: Quality::Lossless,
            genre: Some("Rock".into()),
            artist: Some(SAMPLE_ARTISTS[0].clone()),
            album: Some(SAMPLE_ALBUMS[0].clone()),
            file_path: "/path/to/song/file".into(),
            disc: Some(1),
        },
        Song {
            id: None,
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
            id: None,
            name: "Like the wind".into(),
            track: Some(1),
            duration_s: Some(190.0),
            quality: Quality::Lossy,
            genre: Some("Rock".into()),
            artist: None,
            album: None,
            file_path: "/path/".into(),
            disc: Some(1),
        },
    ]
});

fn get_mock_db() -> ConnectionWrapper {
    let db = ConnectionWrapper {
        conn: sqlite::open(":memory:").expect("Connection failed"),
    };
    db.create_schema();
    db
}

#[test]
fn insert_and_retrieve_artist() {
    let db = get_mock_db();

    for mut artist in SAMPLE_ARTISTS.clone().into_iter() {
        db.insert(&mut artist).expect("Insert");
    }
    let db_artists = db.get_all_artists().expect("Retrieval");

    for (i, db_artist) in db_artists.clone().into_iter().enumerate() {
        assert_eq!(SAMPLE_ARTISTS[i].name, db_artists[i].name);
        println!("{:?}", db_artist);
        if let Some(id) = db_artist.id {
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
        assert_eq!(artist.id.unwrap(), counter);
    }

    let last_id = database::last_id(&db.conn).expect("Last ID retrieval");
    assert_eq!(last_id as usize, SAMPLE_ARTISTS.len());
}

#[test]
fn last_insert_row_id_album() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut album in SAMPLE_ALBUMS.clone().into_iter() {
        counter += 1;
        db.insert(&mut album).expect("Insert");
        assert_eq!(album.id.unwrap(), counter);
    }

    let last_id = database::last_id(&db.conn).expect("Last ID retrieval");
    assert_eq!(last_id as usize, SAMPLE_ALBUMS.len());
}

#[test]
fn last_insert_row_id_song() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut song in SAMPLE_SONGS.clone().into_iter() {
        counter += 1;
        db.insert(&mut song).expect("Insert");
        assert_eq!(song.id.unwrap(), counter);
    }

    let last_id = database::last_id(&db.conn).expect("Last ID retrieval");
    assert_eq!(last_id as usize, SAMPLE_SONGS.len());
}

#[test]
#[should_panic]
fn empty_artist() {
    let db = get_mock_db();
    db.insert(&mut Artist {
        id: None,
        name: "".into(),
    })
    .expect("Expected error");
}

#[test]
#[should_panic]
fn empty_album() {
    let db = get_mock_db();
    db.insert(&mut Album {
        id: None,
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
        id: None,
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
            id: None,
            name: artist.name,
        };
        assert!(db.exists(&mut artist_without_id).expect("Exists check"));
        if let Some(id) = artist_without_id.id {
            assert_eq!(counter, id);
        } else {
            panic!("No ID");
        }
    }

    assert!(!db
        .exists(&mut Artist {
            id: None,
            name: "Does not exist".into(),
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

    let db_albums = db.get_all_albums().expect("Retrieval");

    for (i, db_album) in db_albums.clone().into_iter().enumerate() {
        println!("{:?}", db_album);
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

        if let Some(id) = db_album.id {
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
        if let Some(id) = album.id {
            assert_eq!(counter, id);
        } else {
            panic!("No ID");
        }
    }

    assert!(!db
        .exists(&mut Album {
            id: None,
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
                db.insert(artist).expect("Artist insert");
            }

            if let Some(album) = &mut song.album {
                if let Some(album_artist) = &mut album.artist {
                    db.insert(album_artist).expect("Album artist insert");
                }
                db.insert(album).expect("Album insert");
            }
            db.insert(&mut song).expect("Album insert");
        }
    }

    let db_songs = db.get_all_songs().expect("Retrieval");

    // The comparing of nested fields gets pretty messy... this is just a test function tho
    // so no worries
    for (i, sample_song) in SAMPLE_SONGS.clone().into_iter().enumerate() {
        let db_song = &db_songs[i];
        println!("{:?}", db_song);

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

    let db_songs = db.get_all_songs().expect("Retrieval");

    for (i, sample_song) in SAMPLE_SONGS.clone().into_iter().enumerate() {
        let db_song = &db_songs[i];
        println!("{:?}", db_song);

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
            id: None,
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
        assert_eq!(song_to_search.id.expect(""), counter);
    }

    assert!(!db
        .exists(&mut Album {
            id: None,
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
        db.insert_full(&mut artist).expect("First insert");
    }

    let mut test_artist = SAMPLE_ARTISTS[1].clone();
    assert!(test_artist.id.is_none());
    db.insert_full(&mut test_artist).expect("Second insert");
    assert_eq!(test_artist.id, Some(2));
}

#[test]
fn double_insert_id_album() {
    let db = get_mock_db();
    for mut album in SAMPLE_ALBUMS.clone().into_iter() {
        db.insert(&mut album).expect("First insert");
    }

    let mut test_album = SAMPLE_ALBUMS[1].clone();
    assert!(test_album.id.is_none());
    db.insert(&mut test_album).expect("Second insert");
    assert_eq!(test_album.id, Some(2));
}

#[test]
fn double_insert_id_songs() {
    let db = get_mock_db();
    for mut song in SAMPLE_SONGS.clone().into_iter() {
        db.insert_full(&mut song).expect("First insert");
    }

    let mut test_song = SAMPLE_SONGS[1].clone();
    assert!(test_song.id.is_none());
    db.insert_full(&mut test_song).expect("Second insert");
    assert_eq!(test_song.id, Some(2));
}

use core::panic;

use once_cell::sync::Lazy;

use crate::{
    database::{self, ConnectionWrapper},
    models::{Album, Artist},
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
fn last_insert_row_id() {
    let db = get_mock_db();
    let mut counter = 0;
    for mut artist in SAMPLE_ARTISTS.clone().into_iter() {
        counter += 1;
        db.insert(&mut artist).expect("Insert");
        if let Some(id) = artist.id {
            assert_eq!(id, counter);
        } else {
            panic!("No ID");
        }
    }

    let last_id = database::last_id(&db.conn).expect("Last ID retrieval");
    assert_eq!(last_id as usize, SAMPLE_ARTISTS.len());
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
        assert_eq!(SAMPLE_ALBUMS[i].name, db_albums[i].name);
        assert_eq!(SAMPLE_ALBUMS[i].total_tracks, db_albums[i].total_tracks);
        assert_eq!(SAMPLE_ALBUMS[i].total_discs, db_albums[i].total_discs);
        assert_eq!(SAMPLE_ALBUMS[i].cover_path, db_albums[i].cover_path);
        assert_eq!(SAMPLE_ALBUMS[i].year, db_albums[i].year);

        if let Some(sample_artist) = &SAMPLE_ALBUMS[i].artist {
            if let Some(artist) = &db_albums[i].artist {
                assert_eq!(sample_artist.name, artist.name);
            } else {
                panic!("Inconsistent album artist");
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

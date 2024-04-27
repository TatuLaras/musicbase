use core::panic;

use once_cell::sync::Lazy;

use crate::{
    database::ConnectionWrapper,
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
            name: "Suuria Kuvioita".into(),
            artist: Some(SAMPLE_ARTISTS[0].clone()),
            cover_path: None,
            year: Some(2003),
            tracks: Some(10),
        },
        Album {
            name: "Homogenic".into(),
            artist: Some(SAMPLE_ARTISTS[1].clone()),
            cover_path: Some("path/to/cover/".into()),
            year: Some(1997),
            tracks: Some(10),
        },
        Album {
            name: "".into(),
            artist: None,
            cover_path: None,
            year: None,
            tracks: None,
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

    for artist in SAMPLE_ARTISTS.clone().into_iter() {
        db.insert(&artist).expect("Insert Failed");
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
#[should_panic]
fn empty_artist() {
    let db = get_mock_db();
    db.insert(&Artist {
        id: None,
        name: "".into(),
    })
    .expect("Expected error");
}

#[test]
fn artist_exists() {
    let db = get_mock_db();
    for artist in SAMPLE_ARTISTS.clone().into_iter() {
        db.insert(&artist).expect("Insert");
    }

    assert!(db.exists(&SAMPLE_ARTISTS[0]).expect("Exists check"));

    assert!(!db
        .exists(&Artist {
            id: None,
            name: "Does not exist".into(),
        })
        .expect("Exists check"));
}

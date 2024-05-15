use crate::{
    content_scanner::scan_for_new_content,
    models::{Album, Artist, Quality, Song},
    param::Order,
    test_utils::get_mock_db,
};

#[test]
fn works() {
    let expected_songs = vec![
        Song {
            song_id: Some(1),
            name: "mp3 track 1".into(),
            file_path: "test_audio/sample1.mp3".into(),
            track: Some(1),
            disc: None,
            duration_s: None,
            quality: Quality::Lossy,
            genre: Some("Nothing".into()),
            artist: Some(Artist {
                artist_id: Some(1),
                name: "mp3 artist".into(),
                artist_image_path: None,
            }),
            album: Some(Album {
                album_id: Some(1),
                name: "mp3 album ".into(),
                artist: Some(Artist {
                    artist_id: Some(1),
                    name: "mp3 artist".into(),
                    artist_image_path: None,
                }),
                cover_path: None,
                year: Some(2024),
                total_tracks: None,
                total_discs: None,
            }),
        },
        Song {
            song_id: Some(2),
            name: "flac track 2".into(),
            file_path: "test_audio/sample4.flac".into(),
            track: Some(2),
            disc: None,
            duration_s: Some(0.0),
            quality: Quality::Lossless,
            genre: Some("Nothing pop".into()),
            artist: Some(Artist {
                artist_id: Some(2),
                name: "flac artist".into(),
                artist_image_path: None,
            }),
            album: Some(Album {
                album_id: Some(2),
                name: "flac album 2".into(),
                artist: Some(Artist {
                    artist_id: Some(2),
                    name: "flac artist".into(),
                    artist_image_path: None,
                }),
                cover_path: None,
                year: Some(1990),
                total_tracks: None,
                total_discs: None,
            }),
        },
        Song {
            song_id: Some(3),
            name: "mp3 track 2".into(),
            file_path: "test_audio/sample2.mp3".into(),
            track: Some(2),
            disc: None,
            duration_s: None,
            quality: Quality::Lossy,
            genre: Some("Nothing".into()),
            artist: Some(Artist {
                artist_id: Some(1),
                name: "mp3 artist".into(),
                artist_image_path: None,
            }),
            album: Some(Album {
                album_id: Some(1),
                name: "mp3 album ".into(),
                artist: Some(Artist {
                    artist_id: Some(1),
                    name: "mp3 artist".into(),
                    artist_image_path: None,
                }),
                cover_path: None,
                year: Some(2024),
                total_tracks: None,
                total_discs: None,
            }),
        },
        Song {
            song_id: Some(4),
            name: "flac track 1".into(),
            file_path: "test_audio/sample3.flac".into(),
            track: Some(1),
            disc: None,
            duration_s: Some(0.0),
            quality: Quality::Lossless,
            genre: Some("Nothing rock".into()),
            artist: Some(Artist {
                artist_id: Some(2),
                name: "flac artist".into(),
                artist_image_path: None,
            }),
            album: Some(Album {
                album_id: Some(3),
                name: "flac album".into(),
                artist: Some(Artist {
                    artist_id: Some(2),
                    name: "flac artist".into(),
                    artist_image_path: None,
                }),
                cover_path: None,
                year: Some(1980),
                total_tracks: None,
                total_discs: None,
            }),
        },
    ];

    let db = get_mock_db();
    scan_for_new_content("test_audio/", &db, None).unwrap();

    let db_songs = db.get_all::<Song>(Order::Default).unwrap();
    for target in db_songs {
        println!("{:?}", target);
        assert!(expected_songs.contains(&target))
    }
}

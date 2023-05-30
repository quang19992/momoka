use crate::database::sync::Synchronizer;
use std::sync::Arc;

pub fn synchronizers() -> Arc<Vec<Synchronizer>> {
    Arc::new(vec![master(), v_1(), v_2()])
}

fn master() -> Synchronizer {
    Synchronizer::Simple(vec![
        r#"CREATE TABLE sync_data (field TEXT PRIMARY KEY, value TEXT);"#.to_owned(),
        r#"
        CREATE TYPE image (
            source TEXT,
            identifier TEXT,
        );
        "#.to_owned(),
        r#"
        CREATE TABLE category (
            category_id BIGINT PRIMARY KEY,
            name TEXT,
            description TEXT,
            thumbnail IMAGE
        );
        "#.to_owned(),
        r#"
        CREATE TABLE title (
            title_id BIGINT PRIMARY KEY,
            name TEXT,
            description TEXT,
            category FROZEN<SET<BIGINT>>
        );
        "#.to_owned(),
        r#"
        CREATE TABLE publication (
            title_id BIGINT,
            publication_id BIGINT,
            name TEXT,
            metadata FROZEN<MAP<TEXT, TEXT>>,
            PRIMARY KEY ((title_id), publication_id)
        );
        "#.to_owned(),
        r#"CREATE INDEX ON publication (publication_id);"#.to_owned(),
        r#"
        CREATE TABLE book (
            publication_id BIGINT,
            book_id BIGINT,
            publish_date DATE,
            cover IMAGE,
            is_reprint BOOLEAN,
            metadata FROZEN<MAP<TEXT, TEXT>>,
            PRIMARY KEY ((publication_id), book_id)
        );
        "#.to_owned(),
        r#"CREATE INDEX ON book (book_id);"#.to_owned(),
    ])
}

fn v_1() -> Synchronizer {
    Synchronizer::Simple(vec![
        "CREATE TABLE sync_data (field TEXT PRIMARY KEY, value TEXT);".to_owned(),
    ])
}

/// Added phrase, title, category, publication and book model.
/// Added image type.
fn v_2() -> Synchronizer {
    Synchronizer::Simple(vec![
        r#"
        CREATE TYPE image (
            source TEXT,
            identifier TEXT,
        );
        "#.to_owned(),
        r#"
        CREATE TABLE category (
            category_id BIGINT PRIMARY KEY,
            name TEXT,
            description TEXT,
            thumbnail IMAGE
        );
        "#.to_owned(),
        r#"
        CREATE TABLE title (
            title_id BIGINT PRIMARY KEY,
            name TEXT,
            description TEXT,
            category FROZEN<SET<BIGINT>>
        );
        "#.to_owned(),
        r#"
        CREATE TABLE publication (
            title_id BIGINT,
            publication_id BIGINT,
            name TEXT,
            metadata FROZEN<MAP<TEXT, TEXT>>,
            PRIMARY KEY ((title_id), publication_id)
        );
        "#.to_owned(),
        r#"CREATE INDEX ON publication (publication_id);"#.to_owned(),
        r#"
        CREATE TABLE book (
            publication_id BIGINT,
            book_id BIGINT,
            publish_date DATE,
            cover IMAGE,
            is_reprint BOOLEAN,
            metadata FROZEN<MAP<TEXT, TEXT>>,
            PRIMARY KEY ((publication_id), book_id)
        );
        "#.to_owned(),
        r#"CREATE INDEX ON book (book_id);"#.to_owned(),
    ])
}

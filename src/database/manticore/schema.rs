use crate::database::sync::Synchronizer;
use std::sync::Arc;

pub fn synchronizers() -> Arc<Vec<Synchronizer>> {
    Arc::new(vec![master(), v_1()])
}

fn master() -> Synchronizer {
    Synchronizer::Simple(vec![
        "CREATE TABLE sync_data (field TEXT, value TEXT attribute)".to_owned(),
    ])
}

fn v_1() -> Synchronizer {
    Synchronizer::Simple(vec![
        "CREATE TABLE sync_data (field TEXT, value TEXT attribute)".to_owned(),
    ])
}

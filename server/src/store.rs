use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::types::{answer::{Answer, AnswerId}, question::{QuestidId, Question}};

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestidId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestidId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub teacher_id: usize,
    pub id: Option<usize>,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(json: web::Json<Course>) -> Self {
        let course: Course = json.into_inner();
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name,
            time: course.time,
        }
    }
}

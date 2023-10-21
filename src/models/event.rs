pub struct Event {
    pub id: i32,
    pub name: String,
    pub organisation_id: i32,
    pub created_at: NaiveDateTime,
    pub _types: Option<Vec<String>>,
}

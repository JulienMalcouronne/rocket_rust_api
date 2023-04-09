#[get("/rustaceans")]
pub fn get_rustacean() {}

#[get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32) {}

#[post("/rustaceans")]
pub fn create_rustaceans() {}

#[put("/rustaceans/<id>")]
pub fn update_rustacean(id: i32) {}

#[delete("/rustaceans/<id>")]
pub fn delete_rustacean(id: i32) {}

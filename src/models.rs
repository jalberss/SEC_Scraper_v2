use super::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub acc_number: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub acc_number: &'a str,
}

#[derive(Insertable)]
#[table_name = "test_posts"]
pub struct NewPost<'a> {
    pub acc_number: &'a str,
}

extern crate rusqlite;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use rusqlite::Connection;

#[derive(Serialize, Deserialize)]
struct Post {
    uid: i32,
    content: String,
    sender_id: i32,
    updated_at: i32,
    children: Option<Vec<Post>>
}

impl Post {

    fn roots(db_path: &str) -> Vec<Post> {
        let connection = Connection::open(db_path).unwrap();
        let mut statement = connection.prepare("SELECT uid FROM `posts` WHERE `uid` NOT IN (SELECT `child_uid` FROM `post_post`);").unwrap();
        let posts = statement.query_map(&[], |row| {
            Post::get(row.get(0), db_path)
        }).unwrap();
        let mut array: Vec<Post> = vec![];
        for post in posts {
            if let Ok(p) = post {
                if let Some(pp) = p {
                    array.push(pp);
                }
            }
        }
        array
    }

    fn get(uid: i32, db_path: &str) -> Option<Post> {
        let connection = Connection::open(db_path).unwrap();
        let mut statement = connection.prepare("SELECT * FROM `posts` WHERE `uid` = :uid;").unwrap();
        let mut posts = statement.query_map_named(&[(":uid", &uid)], |row| {
            Post {
                uid: row.get(0),
                content: row.get(1),
                sender_id: row.get(2),
                updated_at: row.get(3),
                children: None
            }
        }).unwrap();
        if let Ok(mut post) = posts.next().unwrap() {
            post.dig(db_path);
            Some(post)
        } else { None }
    }

    fn dig(&mut self, db_path: &str) {
        let mut children = vec![];
        let connection = Connection::open(db_path).unwrap();
        let mut statement = connection.prepare("SELECT `child_uid` FROM `post_post` WHERE `parent_uid` = :1 ORDER BY `child_uid` ASC;").unwrap();
        let children_results = statement.query_map(&[&self.uid], |row| {
            Post::get(row.get(0), db_path)
        }).unwrap();
        for child in children_results {
            if let Ok(post) = child {
                if let Some(p) = post {
                    children.push(p);
                }
            }
        }
        self.children = Some(children);
    }
}

fn main() {
    let roots = Post::roots("/Users/shane/Desktop/zeg_bot.db");
    println!("{}", serde_json::to_string(&roots).unwrap());
}

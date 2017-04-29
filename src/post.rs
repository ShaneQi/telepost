extern crate rusqlite;

use self::rusqlite::Connection;
use self::rusqlite::Error;

#[derive(Serialize, Deserialize)]
pub struct Post {
    uid: i32,
    sender_name: String,
    content: String,
    sender_id: i32,
    updated_at: i32,
    post_type: i32,
    children: Option<Vec<Post>>,
}

impl Post {
    pub fn roots(db_path: &str) -> Result<Vec<Post>, Error> {
        let connection = try!(Connection::open(db_path));
        let mut statement = try!(connection.prepare("SELECT uid FROM `posts` WHERE `uid` NOT IN (SELECT `child_uid` FROM `post_post`) AND `uid` IN (SELECT `parent_uid` FROM `post_post`) ORDER BY `uid` DESC LIMIT 100;"));
        let posts = try!(statement.query_map(&[], |row| Post::get(row.get(0), db_path)));
        let mut posts_vec: Vec<Post> = vec![];
        for post in posts {
            if let Some(post) = post.unwrap_or(None) {
                posts_vec.push(post);
            }
        }
        Ok(posts_vec)
    }

    fn get(uid: i32, db_path: &str) -> Option<Post> {
        let connection = Connection::open(db_path).unwrap();
        let mut statement = connection
            .prepare("SELECT * FROM `posts` WHERE `uid` = :uid;")
            .unwrap();
        let mut posts = statement
            .query_map_named(&[(":uid", &uid)], |row| {
                let uid: i32 = row.get(2);
                let name = match uid {
                    59586902 => "王日天",
                    70475313 => "Jake",
                    79113659 => "毒药",
                    80548625 => "Shane",
                    52478211 => "邦斯",
                    83566437 => "特伦",
                    136698753 => "Adam",
                    88643592 => "YJ",
                    35132256 => "黄日天",
                    158956669 => "诗雨",
                    85225615 => "果汁",
                    50901580 => "Idol",
                    _ => "",
                };
                Post {
                    uid: row.get(0),
                    sender_name: name.to_string(),
                    content: row.get(1),
                    sender_id: uid,
                    updated_at: row.get(3),
                    post_type: row.get(4),
                    children: None,
                }
            })
            .unwrap();
        if let Ok(mut post) = posts.next().unwrap() {
            post.dig(db_path);
            Some(post)
        } else {
            None
        }
    }

    fn dig(&mut self, db_path: &str) {
        let mut children = vec![];
        let connection = match Connection::open(db_path) {
            Ok(connection) => connection,
            Err(err) => panic!(err.to_string()),
        };
        let mut statement = match connection.prepare("SELECT `child_uid` FROM `post_post` WHERE `parent_uid` = :1 ORDER BY `child_uid` ASC;") {
            Ok(statement) => statement,
            Err(err) => panic!(err.to_string())
        };
        let children_results =
            match statement.query_map(&[&self.uid], |row| Post::get(row.get(0), db_path)) {
                Ok(results) => results,
                Err(err) => panic!(err.to_string()),
            };
        for child in children_results {
            if let Some(post) = child.unwrap_or(None) {
                children.push(post);
            }
        }
        self.children = Some(children);
    }
}
extern crate rusqlite;

use self::rusqlite::Connection;

#[derive(Serialize, Deserialize)]
pub struct Post {
    uid: i32,
    sender_name: String,
    content: String,
    sender_id: i32,
    updated_at: i32,
    children: Option<Vec<Post>>,
}

impl Post {
    pub fn roots(db_path: &str) -> Vec<Post> {
        let connection = Connection::open(db_path).unwrap();
        let mut statement = connection.prepare("SELECT uid FROM `posts` WHERE `uid` NOT IN (SELECT `child_uid` FROM `post_post`) ORDER BY `uid` DESC LIMIT 100;").unwrap();
        let posts = statement
            .query_map(&[], |row| Post::get(row.get(0), db_path))
            .unwrap();
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
        let connection = Connection::open(db_path).unwrap();
        let mut statement = connection.prepare("SELECT `child_uid` FROM `post_post` WHERE `parent_uid` = :1 ORDER BY `child_uid` ASC;").unwrap();
        let children_results = statement
            .query_map(&[&self.uid], |row| Post::get(row.get(0), db_path))
            .unwrap();
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
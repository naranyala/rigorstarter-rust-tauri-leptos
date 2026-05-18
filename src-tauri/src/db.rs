use rusqlite::Connection;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct TodoItem {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}

pub fn open(path: &Path) -> Result<Connection, String> {
    Connection::open(path).map_err(|e| e.to_string())
}

pub fn migrate(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )
    .map_err(|e| e.to_string())
}

pub fn insert(conn: &Connection, title: &str) -> Result<TodoItem, String> {
    conn.execute("INSERT INTO todos (title) VALUES (?1)", [title])
        .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    get_by_id(conn, id)
}

pub fn get_by_id(conn: &Connection, id: i64) -> Result<TodoItem, String> {
    conn.query_row(
        "SELECT id, title, completed, created_at FROM todos WHERE id = ?1",
        [id],
        |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get::<_, i32>(2)? != 0,
                created_at: row.get(3)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

pub fn list_all(conn: &Connection) -> Result<Vec<TodoItem>, String> {
    let mut stmt = conn
        .prepare("SELECT id, title, completed, created_at FROM todos ORDER BY id DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get::<_, i32>(2)? != 0,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| e.to_string())?);
    }
    Ok(items)
}

pub fn toggle(conn: &Connection, id: i64) -> Result<TodoItem, String> {
    conn.execute(
        "UPDATE todos SET completed = CASE WHEN completed THEN 0 ELSE 1 END WHERE id = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    get_by_id(conn, id)
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM todos WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        conn
    }

    #[test]
    fn test_insert_and_list() {
        let conn = test_conn();
        insert(&conn, "Test todo").unwrap();
        let items = list_all(&conn).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Test todo");
        assert!(!items[0].completed);
    }

    #[test]
    fn test_toggle() {
        let conn = test_conn();
        let item = insert(&conn, "Toggle me").unwrap();
        assert!(!item.completed);
        let toggled = toggle(&conn, item.id).unwrap();
        assert!(toggled.completed);
        let toggled_back = toggle(&conn, item.id).unwrap();
        assert!(!toggled_back.completed);
    }

    #[test]
    fn test_delete() {
        let conn = test_conn();
        let item = insert(&conn, "Delete me").unwrap();
        assert_eq!(list_all(&conn).unwrap().len(), 1);
        delete(&conn, item.id).unwrap();
        assert!(list_all(&conn).unwrap().is_empty());
    }

    #[test]
    fn test_list_order_desc() {
        let conn = test_conn();
        insert(&conn, "First").unwrap();
        insert(&conn, "Second").unwrap();
        insert(&conn, "Third").unwrap();
        let items = list_all(&conn).unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].title, "Third");
        assert_eq!(items[2].title, "First");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = test_conn();
        let result = get_by_id(&conn, 999);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_title() {
        let conn = test_conn();
        let item = insert(&conn, "").unwrap();
        assert!(item.title.is_empty());
        assert!(!item.completed);
    }

    #[test]
    fn test_toggle_nonexistent() {
        let conn = test_conn();
        let result = toggle(&conn, 999);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_nonexistent() {
        let conn = test_conn();
        let result = delete(&conn, 999);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_toggles() {
        let conn = test_conn();
        let item = insert(&conn, "Stress").unwrap();
        for _ in 0..10 {
            let toggled = toggle(&conn, item.id).unwrap();
            assert_eq!(toggled.id, item.id);
        }
    }

    #[test]
    fn test_migrate_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        migrate(&conn).unwrap();
        migrate(&conn).unwrap();
        insert(&conn, "works").unwrap();
        assert_eq!(list_all(&conn).unwrap().len(), 1);
    }
}

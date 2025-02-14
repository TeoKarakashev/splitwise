use rusqlite::{Connection, Result, params, OptionalExtension};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: i64,
    pub description: String,
    pub amount: f64,
    pub payee_id: i64,
    pub is_settled: bool,
    pub payee_name: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("splitwise.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS payments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                amount REAL NOT NULL,
                payee_id INTEGER NOT NULL,
                is_settled BOOLEAN DEFAULT 0,
                FOREIGN KEY(payee_id) REFERENCES users(id)
            )",
            [],
        )?;
        Ok(Database { conn })
    }

    pub fn add_user(&self, name: &str) -> Result<i64> {
        if let Ok(user) = self.get_user_by_name(name) {
            return Ok(user.id);
        }
        self.conn.execute("INSERT INTO users (name) VALUES (?1)", params![name])?;
        Ok(self.conn.last_insert_rowid())
    }
    

    pub fn get_user_by_name(&self, name: &str) -> Result<User> {
        self.conn.query_row(
            "SELECT id, name FROM users WHERE name = ?1",
            params![name],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        )
    }

    pub fn add_payment(&self, description: &str, split_amount: f64, friend_id: i64) -> Result<()> {
    
        self.conn.execute(
            "INSERT INTO payments (description, amount, payee_id) VALUES (?1, ?2, ?3)",
            params![description, split_amount, friend_id],
        )?;
        Ok(())
    }
    

    pub fn settle_payment(&self, user_name: &str, amount: f64) -> Result<()> {
        let user_id: Option<i64> = self.conn.query_row(
            "SELECT id FROM users WHERE name = ?1",
            [user_name],
            |row| row.get(0),
        ).optional()?;
    
        if let Some(user_id) = user_id {
            self.conn.execute(
                "INSERT INTO payments (description, amount, payee_id, is_settled) VALUES (?1, ?2, ?3, 0)",
                [format!("Settlement with {}", user_name), (-amount).to_string(), user_id.to_string()],
            )?;
        }
        Ok(())
    }
    
    
    

    pub fn get_all_payments(&self) -> Result<Vec<Payment>> {
        let mut stmt = self.conn.prepare(
            "SELECT payments.id, payments.description, payments.amount, payments.payee_id, payments.is_settled, users.name 
             FROM payments
             JOIN users ON payments.payee_id = users.id"
        )?;
    
        let payments = stmt.query_map([], |row| {
            Ok(Payment {
                id: row.get(0)?,
                description: row.get(1)?,
                amount: row.get(2)?,
                payee_id: row.get(3)?,
                is_settled: row.get(4)?,
                payee_name: row.get(5)?, 
            })
        })?
        .collect::<Result<Vec<_>>>()?;
    
        Ok(payments)
    }
    

    pub fn get_balances_with_users(&self) -> Result<Vec<(String, f64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT u.name, SUM(p.amount) AS balance
             FROM payments p
             JOIN users u ON p.payee_id = u.id
             GROUP BY u.name"
        )?;
        let balances = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, 
                row.get::<_, f64>(1)?,   
            ))
        })?
        .collect::<Result<Vec<_>>>()?;
        Ok(balances)
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;

    struct TestDb {
        db: Database,
        added_users: Vec<i64>,
    }

    impl TestDb {
        fn new() -> Self {
            let db = Database::new().expect("Failed to connect to DB");
            Self {
                db,
                added_users: Vec::new(),
            }
        }

        fn add_user(&mut self, name: &str) -> i64 {
            let user_id = self.db.add_user(name).expect("Failed to add user");
            self.added_users.push(user_id);
            user_id
        }

        fn add_payment(&mut self, description: &str, amount: f64, payee_id: i64) {
            self.db
                .add_payment(description, amount, payee_id)
                .expect("Failed to add payment");
        }

        fn cleanup(&mut self) {
            let conn = &mut self.db.conn; 
        
            let tx = conn.transaction().expect("Failed to start transaction");
        
            for &user_id in &self.added_users {
                tx.execute(
                    "DELETE FROM payments WHERE payee_id = ?1",
                    params![user_id],
                )
                .expect("Failed to delete payments for test user");
            }
        
            for &user_id in &self.added_users {
                tx.execute(
                    "DELETE FROM users WHERE id = ?1",
                    params![user_id],
                )
                .expect("Failed to delete test user");
            }
        
            tx.commit().expect("Failed to commit cleanup transaction");
        }
    }        

    #[test]
    fn test_add_user() {
        let mut test_db = TestDb::new();
        test_db.cleanup();
        let user_id = test_db.add_user("Alice");
        assert!(user_id > 0, "User ID should be greater than zero");
        test_db.cleanup();
    }

    #[test]
    fn test_add_payment() {
        let mut test_db = TestDb::new();
        test_db.cleanup();
        let user_id = test_db.add_user("Bob");
        
        test_db.add_payment("Lunch", 20.0, user_id);

        let payments = test_db.db.get_all_payments().unwrap();
        assert_eq!(payments.len(), 1);
        assert_eq!(payments[0].amount, 20.0);
        assert_eq!(payments[0].description, "Lunch");

        test_db.cleanup();
    }

    #[test]
    fn test_settle_payment() {
        let mut test_db = TestDb::new();
        test_db.cleanup();
        let user_id = test_db.add_user("Charlie");
        
        test_db.add_payment("Dinner", 30.0, user_id);
        test_db
            .db
            .settle_payment("Charlie", 30.0)
            .expect("Failed to settle payment");

        let balances = test_db.db.get_balances_with_users().unwrap();
        assert_eq!(balances[0].1, 0.0, "Balance should be zero after settlement");

        test_db.cleanup();
    }
}


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
                payee_name: row.get(5)?, // Assuming you add a 'payee_name' field to your Payment struct
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
                row.get::<_, String>(0)?, // User name
                row.get::<_, f64>(1)?,   // Net balance
            ))
        })?
        .collect::<Result<Vec<_>>>()?;
        Ok(balances)
    }
    
}
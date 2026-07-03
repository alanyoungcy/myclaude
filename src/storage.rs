use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPrompt {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub created_at: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path();
        println!("Opening database at: {:?}", db_path);
        
        let conn = Connection::open(&db_path)?;
        println!("Database connection opened successfully");
        
        // Create tables
        println!("Creating tables...");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS system_prompts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                prompt TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        
        println!("Database tables created successfully");
        Ok(Self { conn })
    }
    
    fn get_db_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("myclaude");
        std::fs::create_dir_all(&path).ok();
        path.push("database.db");
        println!("Database path: {:?}", path);
        path
    }
    
    // Conversation methods
    pub fn create_conversation(&self, title: &str) -> Result<Conversation> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT INTO conversations (id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            [&id, title, &now, &now],
        )?;
        
        Ok(Conversation {
            id,
            title: title.to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }
    
    pub fn get_conversations(&self) -> Result<Vec<Conversation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, created_at, updated_at FROM conversations ORDER BY updated_at DESC"
        )?;
        
        let conversations = stmt.query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?;
        
        conversations.collect()
    }
    
    pub fn get_conversation(&self, id: &str) -> Result<Option<Conversation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, created_at, updated_at FROM conversations WHERE id = ?1"
        )?;
        
        let mut conversations = stmt.query_map([id], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?;
        
        if let Some(conv) = conversations.next() {
            Ok(Some(conv?))
        } else {
            Ok(None)
        }
    }
    
    pub fn delete_conversation(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM conversations WHERE id = ?1", [id])?;
        Ok(())
    }
    
    pub fn update_conversation_timestamp(&self, id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE conversations SET updated_at = ?1 WHERE id = ?2",
            [&now, id],
        )?;
        Ok(())
    }
    
    // Message methods
    pub fn add_message(&self, conversation_id: &str, role: &str, content: &str) -> Result<ChatMessage> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT INTO messages (id, conversation_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            [&id, conversation_id, role, content, &now],
        )?;
        
        self.update_conversation_timestamp(conversation_id)?;
        
        Ok(ChatMessage {
            id,
            conversation_id: conversation_id.to_string(),
            role: role.to_string(),
            content: content.to_string(),
            created_at: now,
        })
    }
    
    pub fn get_messages(&self, conversation_id: &str) -> Result<Vec<ChatMessage>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, conversation_id, role, content, created_at FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC"
        )?;
        
        let messages = stmt.query_map([conversation_id], |row| {
            Ok(ChatMessage {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        
        messages.collect()
    }
    
    // System prompt methods
    pub fn save_system_prompt(&self, name: &str, prompt: &str) -> Result<SystemPrompt> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "INSERT INTO system_prompts (id, name, prompt, created_at) VALUES (?1, ?2, ?3, ?4)",
            [&id, name, prompt, &now],
        )?;
        
        Ok(SystemPrompt {
            id,
            name: name.to_string(),
            prompt: prompt.to_string(),
            created_at: now,
        })
    }
    
    pub fn get_system_prompts(&self) -> Result<Vec<SystemPrompt>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, prompt, created_at FROM system_prompts ORDER BY created_at DESC"
        )?;
        
        let prompts = stmt.query_map([], |row| {
            Ok(SystemPrompt {
                id: row.get(0)?,
                name: row.get(1)?,
                prompt: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        
        prompts.collect()
    }
    
    pub fn delete_system_prompt(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM system_prompts WHERE id = ?1", [id])?;
        Ok(())
    }
}

use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create users table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(255) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
    "#)
    .execute(pool)
    .await?;

    // Create workspaces table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS workspaces (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            color VARCHAR(7) DEFAULT '#6366f1',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
    "#)
    .execute(pool)
    .await?;

    // Create tabs table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS tabs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
            title VARCHAR(1000) NOT NULL,
            url TEXT NOT NULL,
            favicon_url TEXT,
            is_active BOOLEAN DEFAULT FALSE,
            position INTEGER DEFAULT 0,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
    "#)
    .execute(pool)
    .await?;

    // Create bookmarks table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS bookmarks (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title VARCHAR(1000) NOT NULL,
            url TEXT NOT NULL,
            description TEXT,
            tags TEXT[],
            folder VARCHAR(255),
            ai_summary TEXT,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
    "#)
    .execute(pool)
    .await?;

    // Create browsing_sessions table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS browsing_sessions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
            session_data JSONB NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
    "#)
    .execute(pool)
    .await?;

    // Create ai_conversations table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS ai_conversations (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title VARCHAR(255),
            messages JSONB NOT NULL DEFAULT '[]',
            model VARCHAR(100),
            provider VARCHAR(100),
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
    "#)
    .execute(pool)
    .await?;

    // Create content_analysis table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS content_analysis (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            url TEXT NOT NULL,
            title VARCHAR(1000),
            content_hash VARCHAR(64),
            analysis_data JSONB NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
    "#)
    .execute(pool)
    .await?;

    // Create user_data table
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS user_data (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID REFERENCES users(id) ON DELETE CASCADE,
            key VARCHAR(255) NOT NULL,
            value JSONB NOT NULL,
            category VARCHAR(100),
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            UNIQUE(user_id, key, category)
        )
    "#)
    .execute(pool)
    .await?;

    // Create indexes for better performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_bookmarks_user_id ON bookmarks(user_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tabs_workspace_id ON tabs(workspace_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_workspaces_user_id ON workspaces(user_id)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_content_analysis_url ON content_analysis(url)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_user_data_key ON user_data(key)")
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn drop_all_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Drop tables in reverse order due to foreign key constraints
    let tables = vec![
        "user_data",
        "content_analysis", 
        "ai_conversations",
        "browsing_sessions",
        "bookmarks",
        "tabs",
        "workspaces",
        "users"
    ];

    for table in tables {
        sqlx::query(&format!("DROP TABLE IF EXISTS {} CASCADE", table))
            .execute(pool)
            .await?;
    }

    Ok(())
} 
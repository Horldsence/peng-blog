use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input, Password};
use console::style;
use domain::{ADMIN_PERMISSIONS, DEFAULT_USER_PERMISSIONS};
use infrastructure::{establish_connection, Migrator, MigratorTrait, UserRepositoryImpl};
use service::repository::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

/// CLI tool for managing peng-blog
#[derive(Parser)]
#[command(name = "peng-blog")]
#[command(about = "Command-line interface for peng-blog management", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// User management commands
    User {
        #[command(subcommand)]
        user_command: UserCommands,
    },
    /// Database management commands
    Db {
        #[command(subcommand)]
        db_command: DbCommands,
    },
}

#[derive(Subcommand)]
enum UserCommands {
    /// List all users
    List,
    /// Show user details
    Show {
        /// User ID
        id: String,
    },
    /// Create a new user
    Create {
        /// Username
        #[arg(short, long)]
        username: Option<String>,
        /// Password
        #[arg(short, long)]
        password: Option<String>,
        /// Make this user an admin
        #[arg(short, long)]
        admin: bool,
        /// Non-interactive mode
        #[arg(long, conflicts_with_all = ["username", "password"])]
        non_interactive: bool,
    },
    /// Delete a user
    Delete {
        /// User ID
        id: String,
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Reset user password
    ResetPassword {
        /// User ID
        id: String,
        /// New password
        #[arg(short, long)]
        password: Option<String>,
        /// Non-interactive mode
        #[arg(long)]
        non_interactive: bool,
    },
    /// Promote user to admin
    Promote {
        /// User ID
        id: String,
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Demote user from admin
    Demote {
        /// User ID
        id: String,
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Run database migrations
    Migrate,
    /// Reset database (WARNING: This will delete all data!)
    Reset {
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Show database status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // If no subcommand is provided, run the server
    match cli.command {
        None => {
            // Run the blog server (it will initialize logging itself)
            app::run_server().await.map_err(|e| anyhow::anyhow!("{}", e))
        }
        Some(command) => {
            // Load environment variables for CLI commands
            dotenvy::dotenv().ok();
            
            // Initialize logging only for CLI commands (server handles its own logging)
            tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "peng_blog_cli=info".into()),
                )
                .init();
            
            // Get database URL
            let database_url = std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://blog.db".to_string());

            match command {
                Commands::User { user_command } => {
                    handle_user_command(user_command, &database_url).await
                }
                Commands::Db { db_command } => {
                    handle_db_command(db_command, &database_url).await
                }
            }
        }
    }
}

async fn handle_user_command(command: UserCommands, database_url: &str) -> anyhow::Result<()> {
    let db = establish_connection(database_url).await?;
    let user_repo = Arc::new(UserRepositoryImpl::new(db));

    match command {
        UserCommands::List => {
            list_users(&user_repo).await
        }
        UserCommands::Show { id } => {
            show_user(&user_repo, &id).await
        }
        UserCommands::Create { username, password, admin, non_interactive } => {
            create_user(&user_repo, username, password, admin, non_interactive).await
        }
        UserCommands::Delete { id, force } => {
            delete_user(&user_repo, &id, force).await
        }
        UserCommands::ResetPassword { id, password, non_interactive } => {
            reset_password(&user_repo, &id, password, non_interactive).await
        }
        UserCommands::Promote { id, force } => {
            promote_user(&user_repo, &id, force).await
        }
        UserCommands::Demote { id, force } => {
            demote_user(&user_repo, &id, force).await
        }
    }
}

async fn handle_db_command(command: DbCommands, database_url: &str) -> anyhow::Result<()> {
    match command {
        DbCommands::Migrate => {
            let db = establish_connection(database_url).await?;
            Migrator::up(&*db, None).await?;
            println!("{}", style("✓ Database migrations completed successfully").green());
            Ok(())
        }
        DbCommands::Reset { force } => {
            if !force {
                let confirm = Confirm::new()
                    .with_prompt("Are you sure you want to reset database? This will delete ALL data.")
                    .default(false)
                    .interact()?;
                
                if !confirm {
                    println!("{}", style("Operation cancelled").yellow());
                    return Ok(());
                }
            }

            let db = establish_connection(database_url).await?;
            
            // Drop all tables
            Migrator::down(&*db, None).await?;
            
            // Recreate tables
            Migrator::up(&*db, None).await?;
            
            println!("{}", style("✓ Database reset successfully").green());
            Ok(())
        }
        DbCommands::Status => {
            let db = establish_connection(database_url).await?;
            
            println!("\n{}", style("Database Status").bold().cyan());
            println!("Database URL: {}", database_url);
            println!("Connection: {}", style("✓ Connected").green());
            
            // Count users
            let user_repo = Arc::new(UserRepositoryImpl::new(db));
            let users = user_repo.list_users(1000).await?;
            println!("Total users: {}", style(users.len()).cyan());
            
            // Count admins
            let admin_count = users.iter().filter(|u| u.permissions == ADMIN_PERMISSIONS).count();
            println!("Admin users: {}", style(admin_count).cyan());
            
            Ok(())
        }
    }
}

async fn list_users(user_repo: &Arc<UserRepositoryImpl>) -> anyhow::Result<()> {
    println!("\n{}", style("Users").bold().cyan());
    println!("{}", "─".repeat(80));
    
    let users = user_repo.list_users(1000).await?;
    
    if users.is_empty() {
        println!("{}", style("No users found").yellow());
        return Ok(());
    }

    for user in users {
        let role = if user.permissions == ADMIN_PERMISSIONS {
            style("ADMIN").red().bold()
        } else {
            style("USER").green()
        };
        
        println!(
            "ID: {}\nUsername: {}\nRole: {}\nCreated: {}\n{}",
            user.id,
            style(&user.username).bold(),
            role,
            user.created_at.format("%Y-%m-%d %H:%M:%S"),
            "─".repeat(40)
        );
    }

    Ok(())
}

async fn show_user(user_repo: &Arc<UserRepositoryImpl>, id: &str) -> anyhow::Result<()> {
    let user_id = Uuid::parse_str(id)
        .map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;
    
    println!("\n{}", style("User Details").bold().cyan());
    println!("{}", "─".repeat(40));
    println!("ID: {}", user.id);
    println!("Username: {}", style(&user.username).bold());
    println!("Role: {}", if user.permissions == ADMIN_PERMISSIONS {
        style("ADMIN").red().bold()
    } else {
        style("USER").green()
    });
    println!("Permissions: {} ({})", user.permissions, format_permissions(user.permissions));
    println!("Created: {}", user.created_at.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}

async fn create_user(
    user_repo: &Arc<UserRepositoryImpl>,
    username: Option<String>,
    password: Option<String>,
    admin: bool,
    non_interactive: bool,
) -> anyhow::Result<()> {
    let (username, password) = if non_interactive {
        let username = username.ok_or_else(|| anyhow::anyhow!("Username is required in non-interactive mode"))?;
        let password = password.ok_or_else(|| anyhow::anyhow!("Password is required in non-interactive mode"))?;
        (username, password)
    } else {
        let username = match username {
            Some(u) => u,
            None => Input::<String>::new()
                .with_prompt("Username")
                .interact()?,
        };
        
        let password = match password {
            Some(p) => p,
            None => Password::new()
                .with_prompt("Password")
                .with_confirmation("Confirm password", "Passwords do not match")
                .interact()?,
        };
        
        (username, password)
    };

    let existing_user = user_repo.find_by_username(&username).await?;
    
    if existing_user.is_some() {
        return Err(anyhow::anyhow!("Username '{}' already exists", username));
    }

    let permissions = if admin { ADMIN_PERMISSIONS } else { DEFAULT_USER_PERMISSIONS };
    let user = user_repo.create_user(username, password, permissions).await?;

    println!("\n{}", style("✓ User created successfully").green());
    println!("ID: {}", user.id);
    println!("Username: {}", user.username);
    println!("Role: {}", if admin { "ADMIN" } else { "USER" });

    Ok(())
}

async fn delete_user(
    user_repo: &Arc<UserRepositoryImpl>,
    id: &str,
    force: bool,
) -> anyhow::Result<()> {
    let user_id = Uuid::parse_str(id)
        .map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;
    
    if !force {
        let confirm = Confirm::new()
            .with_prompt(&format!("Are you sure you want to delete user '{}'?", user.username))
            .default(false)
            .interact()?;
        
        if !confirm {
            println!("{}", style("Operation cancelled").yellow());
            return Ok(());
        }
    }

    user_repo.delete_user(user_id).await?;

    println!("\n{}", style("✓ User deleted successfully").green());

    Ok(())
}

async fn reset_password(
    user_repo: &Arc<UserRepositoryImpl>,
    id: &str,
    password: Option<String>,
    non_interactive: bool,
) -> anyhow::Result<()> {
    let user_id = Uuid::parse_str(id)
        .map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;
    println!("\nResetting password for user: {}", style(&user.username).bold());

    let password = match password {
        Some(p) => p,
        None if non_interactive => {
            return Err(anyhow::anyhow!("Password is required in non-interactive mode"));
        }
        None => Password::new()
            .with_prompt("New password")
            .with_confirmation("Confirm password", "Passwords do not match")
            .interact()?,
    };

    // Update password using repository method
    user_repo.update_password(user_id, password).await?;

    println!("\n{}", style("✓ Password reset successfully").green());
    println!("User ID: {}", user_id);

    Ok(())
}

async fn promote_user(
    user_repo: &Arc<UserRepositoryImpl>,
    id: &str,
    force: bool,
) -> anyhow::Result<()> {
    let user_id = Uuid::parse_str(id)
        .map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;
    
    if user.permissions == ADMIN_PERMISSIONS {
        println!("{}", style("User is already an admin").yellow());
        return Ok(());
    }

    if !force {
        let confirm = Confirm::new()
            .with_prompt(&format!("Are you sure you want to promote '{}' to admin?", user.username))
            .default(true)
            .interact()?;
        
        if !confirm {
            println!("{}", style("Operation cancelled").yellow());
            return Ok(());
        }
    }


    user_repo.update_permissions(user_id, ADMIN_PERMISSIONS).await?;

    println!("\n{}", style("✓ User promoted to admin").green());

    Ok(())
}

async fn demote_user(
    user_repo: &Arc<UserRepositoryImpl>,
    id: &str,
    force: bool,
) -> anyhow::Result<()> {
    let user_id = Uuid::parse_str(id)
        .map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;
    
    if user.permissions != ADMIN_PERMISSIONS {
        println!("{}", style("User is not an admin").yellow());
        return Ok(());
    }

    if !force {
        let confirm = Confirm::new()
            .with_prompt(&format!("Are you sure you want to demote '{}' from admin?", user.username))
            .default(false)
            .interact()?;
        
        if !confirm {
            println!("{}", style("Operation cancelled").yellow());
            return Ok(());
        }
    }


    user_repo.update_permissions(user_id, DEFAULT_USER_PERMISSIONS).await?;

    println!("\n{}", style("✓ User demoted from admin").green());

    Ok(())
}

fn format_permissions(permissions: u64) -> String {
    let mut flags = vec![];
    
    if permissions & (1 << 0) != 0 { flags.push("POST_CREATE"); }
    if permissions & (1 << 1) != 0 { flags.push("POST_UPDATE"); }
    if permissions & (1 << 2) != 0 { flags.push("POST_DELETE"); }
    if permissions & (1 << 3) != 0 { flags.push("POST_PUBLISH"); }
    if permissions & (1 << 4) != 0 { flags.push("USER_MANAGE"); }
    
    if flags.is_empty() {
        "NONE".to_string()
    } else {
        flags.join(" | ")
    }
}
use sqlx::Row;

/// Represents a Student, taken from the Students table
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, sqlx::FromRow)]
pub struct Student{
    /// The Student primary key.
    pub id: i32,

    /// Name of the student
    pub first_name: String,

    /// last name of the Student
    pub last_name: String,
}

/// Initialize the connection pool.
pub async fn init_db() -> anyhow::Result<sqlx::SqlitePool> {
    // Read env variable for database url
    let database = std::env::var("DATABASE_URL")?;
    // connect to the connection pool of the database
    let conn_pool = sqlx::SqlitePool::connect(&database).await?;
    // run the migration script
    sqlx::migrate!().run(&conn_pool).await?;
    Ok(conn_pool)
}

/// Retreives all Students from the database.
pub async fn all_students_db (conn_pool: &sqlx::SqlitePool) -> anyhow::Result<Vec<Student>>{
    Ok(
        sqlx::query_as::<_, Student>("SELECT * FROM students order by first_name, last_name")
            .fetch_all(conn_pool)
            .await?,
    )
}

/// Retreive Student by id from the database
pub async fn student_db (conn_pool: &sqlx::SqlitePool, id: i32) -> anyhow::Result<Student>{
    Ok(
        sqlx::query_as::<_, Student>("SELECT * FROM students WHERE id=$1")
            .bind(id)
            .fetch_one(conn_pool)
            .await?,
    )
}

/// Add Student to the database.
pub async fn add_student_db<T: ToString> (conn_pool: &sqlx::SqlitePool, first_name: T, last_name: T) -> anyhow::Result<i32> {
    Ok(
        sqlx::query("INSERT INTO students (first_name, last_name) VALUES ($1, $2) RETURNING id")
            .bind(first_name.to_string())
            .bind(last_name.to_string())
            .fetch_one(conn_pool)
            .await?
            .get(0)
    )
}

/// Update a Student in the database.
pub async fn update_student_db(conn_pool: &sqlx::SqlitePool, student: &Student) -> anyhow::Result<()> {
    sqlx::query("UPDATE students SET first_name=$1, last_name=$2 WHERE id=$3")
        .bind(&student.first_name)
        .bind(&student.last_name)
        .bind(&student.id)
        .execute(conn_pool)
        .await?;
    Ok(())
}

/// Delete a Student from the database.
pub async fn delete_student_db(conn_pool: &sqlx::SqlitePool, id: i32) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM students WHERE id=$1")
        .bind(id)
        .execute(conn_pool)
        .await?;
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn get_one_student(){
        dotenv::dotenv().ok();
        let db_conn = init_db().await.unwrap();
        let student = student_db(&db_conn, 1).await.unwrap();
        //println!("{:?}", student);
        assert_eq!(1, student.id);
    }

    #[sqlx::test]
    async fn get_all_students(){
        dotenv::dotenv().ok();
        let db_conn = init_db().await.unwrap();
        let all_rows = all_students_db(&db_conn).await.unwrap();
        assert!(!all_rows.is_empty());
    }
    
}
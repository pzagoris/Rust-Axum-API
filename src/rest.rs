use axum::{Router, Json, http::StatusCode, Extension, extract::{Path, self}};
use axum::routing::{delete, get, post, put};
use sqlx::SqlitePool;
use crate::db::{Student, all_students_db, student_db, add_student_db, delete_student_db, update_student_db};

/// THe boooks Rest service.
pub fn students_service() -> Router{
    Router::new()
        .route("/", get(get_all_students))
        .route("/:id", get(get_student))
        .route("/add", post(add_student))
        .route("/edit", put(update_student))
        .route("/delete/:id", delete(delete_student))       
}
// Extencion(cnn) is a dependency injeected by Axum from the database layer

/// GET all students.
async fn get_all_students(Extension(cnn): Extension<SqlitePool>) -> Result<Json<Vec<Student>>, StatusCode>{
    if let Ok(students) = all_students_db(&cnn).await {
        Ok(Json(students))
    }else{
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

//  id number that is parsed by Axum from the path.
/// GET a student.
async fn get_student(Extension(cnn): Extension<SqlitePool>, Path(id): Path<i32>) -> Result<Json<Student>, StatusCode> {
    if let Ok(student) = student_db(&cnn, id).await {
        Ok(Json(student))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

//  JSON encoded student to update from the patch body.
/// ADD a student.
async fn add_student(Extension(cnn): Extension<SqlitePool>, extract::Json(student): extract::Json<Student>) -> Result<Json<i32>, StatusCode> {
    if let Ok(new_id) = add_student_db(&cnn, &student.first_name, &student.last_name).await {
        Ok(Json(new_id))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
   }
}

/// UPDATE a student.
async fn update_student(Extension(cnn): Extension<SqlitePool>, extract::Json(student): extract::Json<Student>) -> StatusCode {
    if update_student_db(&cnn, &student).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

/// DELETE a student.
async fn delete_student(Extension(cnn): Extension<SqlitePool>, Path(id): Path<i32>) -> StatusCode {
    if delete_student_db(&cnn, id).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}


#[cfg(test)]
mod test{
    use super::*;
    // It creates a local-version and returns the client ready for testing
    use axum_test_helper::TestClient;

    async fn setup_tests() -> TestClient {
        dotenv::dotenv().ok();
        let connection_pool = crate::init_db().await.unwrap();
        let app = crate::router(connection_pool);
        TestClient::new(app)
    }

    #[tokio::test]
    async fn get_all_students() {
        let client = setup_tests().await;
        let res = client.get("/students").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        let students: Vec<Student> = res.json().await;
        assert!(!students.is_empty());
    }
}
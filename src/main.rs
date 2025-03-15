use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use payroll_manager::{config::{self}, entities::{company::{self, company_repository::CompanyRepository, company_service::CompanyService}, payroll::{self, payroll_repository::PayrollRepository, payroll_service::PayrollService}, permission::{permission_repository::PermissionRepository, permission_service::PermissionService}}, initialize_config, service::{self, ServiceHub}, user::{self, auth_service::AuthService, user_repository::UserRepository, user_service::UserService}, util::{db::{get_db_pool, run_migrations}, minio::MinioService}};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    initialize_config();

    let config = config::get();

    let db_pool = get_db_pool(&config.database.url).await;
    let minio_service = Arc::new(MinioService::new(&config.bucket.host, &config.bucket.access_key, &config.bucket.secret_key));

    match run_migrations(&db_pool).await {
        Ok(_) => (),
        Err(err) => panic!("Failed to run migrations: {}", err.message())
    }

    minio_service.create_bucket_if_not_exists(&config.bucket.payroll_base_bucket_name).await.expect("Failed to create bucket");

    let permission_repository = PermissionRepository::new();
    let permission_service = PermissionService::new(db_pool.clone(), permission_repository);

    let user_repository = UserRepository::new();
    let user_service = UserService::new(db_pool.clone(), user_repository);

    let auth_service = AuthService::new();

    let company_repository = CompanyRepository::new();
    let company_service = CompanyService::new(db_pool.clone(), company_repository);

    let payroll_repository = PayrollRepository::new();
    let payroll_service = PayrollService::new(db_pool.clone(), payroll_repository, Arc::clone(&minio_service));

    service::init(ServiceHub {
        permission_service,
        auth_service,
        user_service,
        company_service,
        payroll_service
    });

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api/v1")
                    .configure(user::auth_controller::config)
                    .configure(user::user_controller::config)
                    .configure(company::company_controller::config)
                    .configure(payroll::payroll_controller::config)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

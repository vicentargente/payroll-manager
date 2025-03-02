use std::sync::OnceLock;

use crate::{entities::{company::company_service::CompanyService, payroll::payroll_service::PayrollService, permission::permission_service::PermissionService}, user::{auth_service::AuthService, user_service::UserService}};

pub struct ServiceHub {
    pub permission_service: PermissionService,
    pub auth_service: AuthService,
    pub user_service: UserService,
    pub company_service: CompanyService,
    pub payroll_service: PayrollService
}

impl ServiceHub {
    pub fn permission(&self) -> &PermissionService {
        &self.permission_service
    }

    pub fn auth(&self) -> &AuthService {
        &self.auth_service
    }

    pub fn user(&self) -> &UserService {
        &self.user_service
    }

    pub fn company(&self) -> &CompanyService {
        &self.company_service
    }

    pub fn payroll(&self) -> &PayrollService {
        &self.payroll_service
    }
}

static INSTANCE: OnceLock<ServiceHub> = OnceLock::new();

pub fn init(service_hub: ServiceHub) {
    match INSTANCE.set(service_hub) {
        Ok(_) => (),
        Err(_) => panic!("ServiceHub already initialized"),
    }
}

pub fn get() -> &'static ServiceHub {
    INSTANCE.get().expect("ServiceHub not initialized")
}

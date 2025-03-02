use serde::Deserialize;

pub struct Permission {
    pub user_id: i64,
    pub user: i16,
    pub payroll: i16,
    pub company: i16
}

impl Permission {
    pub fn default(user_id: i64) -> Permission {
        Permission {
            user_id,
            user: 0,
            payroll: 0,
            company: 0
        }
    }

    pub fn new(
        user_id: i64,
        user: i16,
        payroll: i16,
        company: i16
    ) -> Permission {
        Permission {
            user_id,
            user,
            payroll,
            company
        }
    }

    pub fn from_role(user_id: i64, role: Role) -> Permission {
        let mut permission = Permission::default(user_id);

        match role {
            Role::SuperAdmin => {
                permission.set_user(Scope::Any(Operation::Create));
                permission.set_user(Scope::Any(Operation::Read));
                permission.set_user(Scope::Any(Operation::Update));
                permission.set_user(Scope::Any(Operation::Delete));

                permission.set_payroll(Scope::Any(Operation::Create));
                permission.set_payroll(Scope::Any(Operation::Read));
                permission.set_payroll(Scope::Any(Operation::Update));
                permission.set_payroll(Scope::Any(Operation::Delete));

                permission.set_company(Scope::Any(Operation::Create));
                permission.set_company(Scope::Any(Operation::Read));
                permission.set_company(Scope::Any(Operation::Update));
                permission.set_company(Scope::Any(Operation::Delete));
            },
            Role::Admin => {
                permission.set_user(Scope::SelfCompany(Operation::Create));
                permission.set_user(Scope::SelfCompany(Operation::Read));
                permission.set_user(Scope::SelfCompany(Operation::Update));
                permission.set_user(Scope::SelfCompany(Operation::Delete));

                permission.set_payroll(Scope::SelfCompany(Operation::Create));
                permission.set_payroll(Scope::SelfCompany(Operation::Read));
                permission.set_payroll(Scope::SelfCompany(Operation::Update));
                permission.set_payroll(Scope::SelfCompany(Operation::Delete));

                permission.set_company(Scope::SelfCompany(Operation::Read));
                permission.set_company(Scope::SelfCompany(Operation::Update));
                permission.set_company(Scope::SelfCompany(Operation::Delete));
            },
            Role::User => {
                permission.set_user(Scope::Owned(Operation::Read));

                permission.set_payroll(Scope::Owned(Operation::Read));

                permission.set_company(Scope::Owned(Operation::Read));
            },
        };

        permission
    }

    pub fn user(&self, scope: Scope) -> bool {
        self.user & scope.mask() != 0
    }

    pub fn set_user(&mut self, scope: Scope) {
        self.user |= scope.mask();
    }

    pub fn payroll(&self, scope: Scope) -> bool {
        self.payroll & scope.mask() != 0
    }

    pub fn set_payroll(&mut self, scope: Scope) {
        self.payroll |= scope.mask();
    }

    pub fn company(&self, scope: Scope) -> bool {
        self.company & scope.mask() != 0
    }

    pub fn set_company(&mut self, scope: Scope) {
        self.company |= scope.mask();
    }
}

#[derive(Clone, Copy)]
pub enum Operation {
    Create,
    Read,
    Update,
    Delete
}

impl Operation {
    fn value(&self) -> i16 {
        match self {
            Operation::Create => 0,
            Operation::Read => 1,
            Operation::Update => 2,
            Operation::Delete => 3
        }
    }
}

pub enum Scope {
    Any(Operation),
    SelfCompany(Operation),
    Owned(Operation)
}

impl Scope {
    fn mask(&self) -> i16 {
        let offset = match self {
            Scope::Any(operation) => operation.value(),
            Scope::SelfCompany(operation) => operation.value() + 4,
            Scope::Owned(operation) => operation.value() + 8,
        };

        1 << offset
    }
}

#[derive(Clone, Copy, Deserialize)]
pub enum Role {
    SuperAdmin,
    Admin,
    User
}

pub struct CreateLanguageRequest {
    pub name: String,
}

pub struct Language {
    pub id: i32,
    pub name: String,
}

pub struct CreateTypeRequest {
    pub name: String,
    pub lang_id: i32,
}

pub struct VariableType {
    pub id: i32,
    pub name: String,
}

pub struct VariableCore {
    pub id: i32,
    pub name: String,
}

pub struct CreateCoreRequest {
    pub name: String,
    pub lang_id: i32,
}

pub struct VariableRole {
    pub id: i32,
    pub name: String,
}

pub struct CreateRoleRequest {
    pub name: String,
    pub lang_id: i32,
}

pub struct CreateSdtmVersionRequest {
    pub name: String,
    pub lang_id: i32,
}

pub struct SdtmVersion {
    pub id: i32,
    pub name: String,
}

pub struct CreateSdtmDomainRequest {
    pub name: String,
    pub description: String,
    pub version_id: i32,
}

pub struct SdtmDomain {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct CreateSdtmVariableRequest {
    pub name: String,
    pub label: String,
    pub domain_id: i32,
    pub variable_type_id: i32,
    pub codelist: String,
    pub variable_core_id: i32,
    pub variable_role_id: i32,
    pub variable_order: i32,
}

pub struct SdtmVariable {
    pub id: i32,
    pub name: String,
    pub label: String,
    pub variable_type: String,
    pub codelist: String,
    pub variable_core: String,
    pub variable_role: String,
    pub variable_order: i32,
}

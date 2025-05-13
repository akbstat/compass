use apis::sdtm::metadata::{
    Language, SdtmDomain, SdtmVariable, SdtmVersion, VariableCore, VariableRole, VariableType,
};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct LanguageRow {
    pub id: i32,
    pub name: String,
}

impl Into<Language> for LanguageRow {
    fn into(self) -> Language {
        Language {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct VariableTypeRow {
    id: i32,
    name: String,
}

impl Into<VariableType> for VariableTypeRow {
    fn into(self) -> VariableType {
        VariableType {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct VariableCoreRow {
    id: i32,
    name: String,
}

impl Into<VariableCore> for VariableCoreRow {
    fn into(self) -> VariableCore {
        VariableCore {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct VariableRoleRow {
    id: i32,
    name: String,
}

impl Into<VariableRole> for VariableRoleRow {
    fn into(self) -> VariableRole {
        VariableRole {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct SdtmVersionRow {
    id: i32,
    name: String,
}

impl Into<SdtmVersion> for SdtmVersionRow {
    fn into(self) -> SdtmVersion {
        SdtmVersion {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct SdtmDomainRow {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Into<SdtmDomain> for SdtmDomainRow {
    fn into(self) -> SdtmDomain {
        SdtmDomain {
            id: self.id,
            name: self.name,
            description: self.description,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct SdtmVariableViewRow {
    pub variable_id: i32,
    pub variable_name: String,
    pub variable_label: String,
    pub domain_id: i32,
    pub variable_type: String,
    pub codelist: String,
    pub variable_core: String,
    pub variable_role: String,
    pub variable_order: i32,
}

impl Into<SdtmVariable> for SdtmVariableViewRow {
    fn into(self) -> SdtmVariable {
        SdtmVariable {
            id: self.variable_id,
            name: self.variable_name,
            label: self.variable_label,
            variable_type: self.variable_type,
            codelist: self.codelist,
            variable_core: self.variable_core,
            variable_role: self.variable_role,
            variable_order: self.variable_order,
        }
    }
}

use apis::sdtm::annotation::{
    Annotation, AnnotationKind, AnnotationVersion, FormDomain, FormVariable,
};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct AnnotationVersionRow {
    pub id: i32,
    pub project_version_id: i32,
    pub name: String,
    pub description: String,
}

impl Into<AnnotationVersion> for AnnotationVersionRow {
    fn into(self) -> AnnotationVersion {
        AnnotationVersion {
            id: self.id,
            project_version_id: self.project_version_id,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct FormDomainRow {
    pub id: i32,
    pub annotation_version_id: i32,
    pub form_id: i32,
    pub name: String,
    pub description: String,
}

impl Into<FormDomain> for FormDomainRow {
    fn into(self) -> FormDomain {
        FormDomain {
            id: self.id,
            annotation_version_id: self.annotation_version_id,
            form_id: self.form_id,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct FormVariableRow {
    pub id: i32,
    pub domain_id: i32,
    pub name: String,
    pub supp: bool,
}

impl Into<FormVariable> for FormVariableRow {
    fn into(self) -> FormVariable {
        FormVariable {
            id: self.id,
            domain_id: self.domain_id,
            name: self.name,
            supp: self.supp,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct AnnotationRow {
    pub id: i32,
    pub annotation_version_id: i32,
    pub form_id: i32,
    pub variable_id: i32,
    pub source_id: i32,
    pub kind: i32,
    pub annotation_display: String,
    pub assign: bool,
}

impl Into<Annotation> for AnnotationRow {
    fn into(self) -> Annotation {
        Annotation {
            id: self.id,
            annotation_version_id: self.annotation_version_id,
            form_id: self.form_id,
            variable_id: self.variable_id,
            source_id: self.source_id,
            kind: match self.kind {
                0 => AnnotationKind::Form,
                1 => AnnotationKind::Item,
                2 => AnnotationKind::Value,
                3 => AnnotationKind::Unit,
                4 => AnnotationKind::Option,
                _ => AnnotationKind::Unknown,
            },
            annotation_display: self.annotation_display,
            assign: self.assign,
        }
    }
}

pub struct NewAnnotation {
    pub annotation_version_id: i32,
    pub form_id: i32,
    pub variable_id: i32,
    pub source_id: i32,
    pub kind: i32,
    pub annotation_display: String,
    pub assign: bool,
}

pub struct UpdateAnnotation {
    pub variable_id: Option<i32>,
    pub annotation_display: String,
    pub assign: bool,
}

// pub fn bool_to_i16(source: &bool) -> i16 {
//     match source {
//         true => 1,
//         false => 0,
//     }
// }

// pub fn i16_to_bool(source: &i16) -> bool {
//     match source {
//         0 => false,
//         _ => true,
//     }
// }

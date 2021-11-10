use crate::value::*;
use std::collections::HashMap;

pub enum FieldPath {
    Field(String),
    Path(Vec<String>),
}

pub enum ResultOrder {
    Asc(FieldPath),
    Desc(FieldPath),
}

pub type ConstraintDocument = HashMap<FieldPath, Constraint>;

// Note that Constraints applied to an array
// value will map the constraint over the array.
pub enum Constraint {
    // constraints on subdocuments
    MatchesDocument(ConstraintDocument),

    // constraints on values
    Equals(Value),
    LessThan(Value),
    GreaterThan(Value),
    In(Vec<Value>),
    And(Box<Constraint>, Box<Constraint>),
}

pub type ProjectionDocument = HashMap<FieldPath, Projection>;

pub enum Projection {
    // projections on subdocuments
    ProjectDocument(ProjectionDocument),

    // projections on values
    Exclude,
    Include,
}

pub struct Query {
    constraints: ConstraintDocument,
    projection: ProjectionDocument,
    order: Option<Vec<ResultOrder>>,
}

// TODO
pub struct UpdateDocument {}

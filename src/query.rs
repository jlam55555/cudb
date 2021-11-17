//! User-facing query representation.

use crate::value::*;
use std::collections::HashMap;

/// Representations of a (possibly nested) document field.
pub enum FieldPath {
    /// The full path as a string. May include dots representing nesting.
    Field(String),
    /// The full path as a vector of path components.
    Path(Vec<String>),
}

/// The order that the results should be sorted in.
pub enum ResultOrder {
    /// Sorted ascending according to the given field.
    Asc(FieldPath),
    /// Sorted descending according to the given field.
    Desc(FieldPath),
}

/// Document type modeling query constraints.
pub type ConstraintDocument = HashMap<FieldPath, Constraint>;

/// A single query constraint on a field.
///
/// Note that Constraints applied to an array
/// value will map the constraint over the array.
pub enum Constraint {
    /// Constraints on subdocuments (hashtables).
    MatchesDocument(ConstraintDocument),

    /// Equality constraint on a value.
    Equals(Value),

    /// Less-than constraint on a value.
    LessThan(Value),

    /// Greater-than constraint on a value.
    GreaterThan(Value),

    /// Constraint if value is in specified list of values.
    In(Vec<Value>),

    /// Conjunction of constraints on a single field.
    And(Box<Constraint>, Box<Constraint>),
}

/// Projection of fields during a query (analogous to SQL `SELECT`).
pub type ProjectionDocument = HashMap<FieldPath, Projection>;

/// Projection of a single field of the projection document.
pub enum Projection {
    // Recursive projections on subdocuments.
    ProjectDocument(ProjectionDocument),

    // Exclude projecting a value.
    Exclude,

    // Project a value.
    Include,
}

/// Complete query operation.
pub struct Query {
    // Constraint document (`WHERE`)
    constraints: ConstraintDocument,

    // Projection document (`SELECT`)
    projection: ProjectionDocument,

    // Ordering document (`ORDER BY`)
    order: Option<Vec<ResultOrder>>,
}

/// Document used in update operations.
pub struct UpdateDocument {
    // TODO
}

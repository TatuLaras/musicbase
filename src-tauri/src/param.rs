// A mini domain specific language thing to abstractly query the database

pub trait AsQuery {
    fn as_query(&self, default: Self) -> String;
}

//  TODO: Come up with something better for identifying database fields than a string, the current
//  implementation requires the user of the API to have knowledge of the underlying SQL query to
//  know which fields are valid or not.
//  Works for now as a convenience thing but should be improved with something like an enum.

pub enum Order {
    Asc(String),
    Desc(String),
    Default,
    None,
}

pub fn asc(field: &str) -> Order {
    Order::Asc(field.to_string())
}

pub fn desc(field: &str) -> Order {
    Order::Desc(field.to_string())
}

impl AsQuery for Order {
    fn as_query(&self, default: Order) -> String {
        match self {
            Order::Asc(field) => format!("{} ASC", field),
            Order::Desc(field) => format!("{} DESC", field),
            Order::None => "".to_string(),
            Order::Default => default.as_query(Order::None),
        }
    }
}

pub enum Condition {
    Eq(String, String),
    Lte(String, String),
    Gte(String, String),
    Lt(String, String),
    Gt(String, String),
    Like(String, String),
    Search(String, String),
    None,
}

impl AsQuery for Condition {
    fn as_query(&self, _default: Condition) -> String {
        match self {
            Condition::Eq(field, value) => format!("{} = '{}'", field, value),
            Condition::Lte(field, value) => format!("{} <= '{}'", field, value),
            Condition::Gte(field, value) => format!("{} >= '{}'", field, value),
            Condition::Lt(field, value) => format!("{} < '{}'", field, value),
            Condition::Gt(field, value) => format!("{} > '{}'", field, value),
            Condition::Like(field, value) => format!("{} LIKE '%{}%'", field, value),
            // Note that at the moment search is just an alias for like. I'll still keep them
            // separate if I want to use a more sophisticated search method in the future.
            Condition::Search(field, value) => format!("{} LIKE '%{}%'", field, value),
            Condition::None => "1 = 1".into(),
        }
    }
}

pub fn eq(field: &str, value: &str) -> Condition {
    Condition::Eq(field.into(), value.into())
}

pub fn lte(field: &str, value: &str) -> Condition {
    Condition::Lte(field.into(), value.into())
}

pub fn gte(field: &str, value: &str) -> Condition {
    Condition::Gte(field.into(), value.into())
}

pub fn lt(field: &str, value: &str) -> Condition {
    Condition::Lt(field.into(), value.into())
}

pub fn gt(field: &str, value: &str) -> Condition {
    Condition::Gt(field.into(), value.into())
}

pub fn like(field: &str, value: &str) -> Condition {
    Condition::Like(field.into(), value.into())
}

pub fn search(field: &str, value: &str) -> Condition {
    Condition::Search(field.into(), value.into())
}

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::request::Request;
use crate::response::Response;

type RouteFunction = fn(&Request) -> Response;

#[derive(Debug)]
pub struct Router {
    routes: HashMap<String, HashMap<HTTPMethod, Route>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn route(mut self, route: Route) -> Router {
        self.routes
            .entry(route.path.clone())
            .or_insert_with(HashMap::new)
            .insert(route.method.clone(), route);

        self
    }

    pub fn find(&self, request: &Request) -> Option<RouteFunction> {
        Some(
            self.routes
                .get(&request.endpoint)?
                .get(&request.method)?
                .function,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HTTPMethod {
    GET,
    POST,
}

impl Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPMethod::GET => write!(f, "GET"),
            HTTPMethod::POST => write!(f, "POST"),
        }
    }
}
impl FromStr for HTTPMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            _ => Err(format!("Unsupported HTTP Method: {}", s)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Route {
    pub path: String,
    pub method: HTTPMethod,
    pub function: RouteFunction,
}

impl Route {
    pub fn new(path: &str, method: HTTPMethod, function: RouteFunction) -> Self {
        Self {
            path: path.to_owned(),
            method,
            function,
        }
    }
}

pub fn get(path: &str, function: RouteFunction) -> Route {
    Route::new(path, HTTPMethod::GET, function)
}

pub fn post(path: &str, function: RouteFunction) -> Route {
    Route::new(path, HTTPMethod::POST, function)
}

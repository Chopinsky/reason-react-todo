#![allow(dead_code)]
#![allow(unused_mut)]

use std::collections::HashMap;
use regex::Regex;
use http::*;

pub enum REST {
    NONE,
    GET,
    POST,
    PUT,
    DELETE,
    OTHER(String),
}

impl Default for REST {
    fn default() -> REST { REST::NONE }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RequestPath {
    Exact(&'static str),
    Partial(&'static str),
    WildCard(&'static str),
}

/* Manual mayham...

impl PartialEq for RequestPath {
    fn eq(&self, other: &RequestPath) -> bool {
        match self {
            &RequestPath::Literal(lit_val) => {
                match other {
                    &RequestPath::Literal(other_val) => lit_val == other_val,
                    _ => false,
                }
            },
            &RequestPath::WildCard(wild_card_val) => {
                match other {
                    &RequestPath::WildCard(other_val) => wild_card_val == other_val,
                    _ => false,
                }
            }
        }
    }
}

impl Eq for RequestPath {}

impl Hash for RequestPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            &RequestPath::Literal(lit_val) => lit_val.hash(state),
            &RequestPath::WildCard(wild_card_val) => wild_card_val.hash(state)
        }
    }
}

 * End of manual mayham
 */

pub type Callback = fn(Request, &mut Response);

pub struct Route {
    get: HashMap<RequestPath, Callback>,
    post: HashMap<RequestPath, Callback>,
    put: HashMap<RequestPath, Callback>,
    delete: HashMap<RequestPath, Callback>,
    others: HashMap<RequestPath, Callback>,
}

pub trait Router {
    fn get(&mut self, uri: RequestPath, callback: Callback);
    fn post(&mut self, uri: RequestPath, callback: Callback);
    fn put(&mut self, uri: RequestPath, callback: Callback);
    fn delete(&mut self, uri: RequestPath, callback: Callback);
    fn other(&mut self, uri: RequestPath, callback: Callback);
}

pub trait RouteHandler {
    fn handle_get(&self, req: Request, resp: &mut Response);
    fn handle_put(&self, req: Request, resp: &mut Response);
    fn handle_post(&self, req: Request, resp: &mut Response);
    fn handle_delete(&self, req: Request, resp: &mut Response);
    fn handle_other(&self, req: Request, resp: &mut Response);
}

impl Route {
    pub fn new() -> Self {
        Route {
            get: HashMap::new(),
            post: HashMap::new(),
            put: HashMap::new(),
            delete: HashMap::new(),
            others: HashMap::new(),
        }
    }

    pub fn from(source: &Route) -> Self {
        Route {
            get: source.get.clone(),
            put: source.put.clone(),
            post: source.post.clone(),
            delete: source.delete.clone(),
            others: source.others.clone(),
        }
    }
}

impl Router for Route {
    //TODO: add special handling for "*"

    fn get(&mut self, uri: RequestPath, callback: Callback) {
        self.get.entry(uri).or_insert(callback);
    }

    fn put(&mut self, uri: RequestPath, callback: Callback) {
        self.put.entry(uri).or_insert( callback);
    }

    fn post(&mut self, uri: RequestPath, callback: Callback) {
        self.post.entry(uri).or_insert(callback);
    }

    fn delete(&mut self, uri: RequestPath, callback: Callback) {
        self.delete.entry(uri).or_insert( callback);
    }

    fn other(&mut self, uri: RequestPath, callback: Callback) {
        self.others.entry(uri).or_insert( callback);
    }
}

impl RouteHandler for Route {
    fn handle_get(&self, req: Request, resp: &mut Response) {
        handle_request_worker(&self.get, req, resp)
    }

    fn handle_put(&self, req: Request, resp: &mut Response) {
        handle_request_worker(&self.put, req, resp)
    }

    fn handle_post(&self, req: Request, resp: &mut Response) {
        handle_request_worker(&self.post, req, resp)
    }

    fn handle_delete(&self, req: Request, resp: &mut Response) {
        handle_request_worker(&self.delete, req, resp)
    }

    fn handle_other(&self, req: Request, resp: &mut Response) {
        handle_request_worker(&self.others, req, resp)
    }
}

fn handle_request_worker(routes: &HashMap<RequestPath, Callback>, req: Request, resp: &mut Response) {
    if let Some(callback) = seek_path(&routes, req.path.clone()) {
        //Callback function will decide what to be written into the response
        callback(req, resp);
    } else {
        resp.status(404);
    }
}

fn seek_path(routes: &HashMap<RequestPath, Callback>, uri: String) -> Option<&Callback> {
    for (req_path, callback) in routes.iter() {
        match req_path.to_owned() {
            RequestPath::Exact(val) => {
                if uri.eq(&val) {
                    return Some(callback);
                }
            },
            RequestPath::Partial(val) => {
                if uri.starts_with(&val) {
                    return Some(callback);
                }
            },
            RequestPath::WildCard(wild) => {
                if let Ok(re) = Regex::new(wild) {
                    if re.is_match(&uri) {
                        return Some(callback);
                    }
                }
            }
        }
    }

    None
}
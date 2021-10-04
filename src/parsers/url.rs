use regex::Regex;
use thiserror::Error;
use std::fmt;

use crate::error;

#[derive(Error, Debug, PartialEq)]
pub enum PathParamError {
    #[error("empty path param: // or ends with /")]
    EmptyParam,
    #[error("ill formed path param: {0}")]
    IllFormedParam(String),
}


#[derive(Error, PartialEq)]
pub enum PathsError {
    #[error("path does not start with /: {0}")]
    StartSlash(String),
    #[error("path is empty")]
    EmptyPath,
    #[error("unable to match paths: {0}")]
    BadMatch(String),
    #[error("invalid path from path parameter")]
    BadPath(#[from] PathParamError),
}

impl fmt::Debug for PathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        error::debug::error_chain_fmt(self, f)
    }
}
        
#[derive(Error, Debug, PartialEq)]
pub enum QueryParamError {
    #[error("empty path param: && or ends with &")]
    EmptyParam,
    #[error("ill formed path param: {0}")]
    IllFormedParam(String),
}

#[derive(Error, PartialEq)]
pub enum QueriesError {
    #[error("empty queries")]
    EmptyQueries,
    #[error("invalid query from query parameters")]
    BadQuery(#[from] QueryParamError),
}

impl fmt::Debug for QueriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error::debug::error_chain_fmt(self, f)
    }
}

#[derive(Debug, PartialEq)]
pub struct QueryParam<'a> {
    name:       &'a str,
    data_type:  &'a str,
}

#[derive(Debug, PartialEq)]
pub struct PathParam<'a> {
    name:       &'a str,
    data_type:  &'a str,
}

#[derive(Debug, PartialEq)]
pub struct ApiGet<'a> {
    url:        &'a str,
    ver:        &'a str,
    paths:      Vec<PathParam<'a>>, queries:    Option<Vec<QueryParam<'a>>>,
}

pub fn parse_api_url<'a>(url: &'a str) -> ApiGet<'a> {
    lazy_static! {
        static ref URL_RE: Regex = Regex::new(r"(?x)    # https://spotify.com|v1/{artist,id}/hello?joe=5&bloe=4
                (?P<base>https?://(?:\S+?))             # base = https://spotify.com
                \|                                      # |
                (?P<ver>\S+?)                           # ver = v1
                (?P<paths>/[^?\s]+)           # paths = /{artist,id}/hello
                (?:\?(                                  # ?
                      ?P<queries>(\S+)                  # queries = joe=5&bloe=4
                ))?$
            ")
            .unwrap();
    }

    let trim_url = url.trim();
    let capture = URL_RE.captures(trim_url).unwrap();

    let q;
    q = match capture.name("queries") {
        None    =>  None,
        Some(i) =>  Some(parse_api_queries(i.as_str()).unwrap()),
    };

    ApiGet {
        url:        capture.name("base").unwrap().as_str(),
        ver:        capture.name("ver").unwrap().as_str(),
        paths:      parse_api_path(&capture.name("paths").unwrap().as_str()).unwrap(),
        queries:    q,
    }
}

fn parse_path_param (param: &str) -> Result<PathParam, PathParamError> {
    if param.is_empty() {
        return Err(PathParamError::EmptyParam);
    }

    lazy_static! {
        static ref PARAM_RE: Regex = Regex::new(r"(?x)   # {hello,world}
                \{                                       # {
                (?P<name>\S[^,]+?)                       # name = hello
                ,                                        # ,
                (?P<data_type>\S[^,]+?)                  # data_type = world
                \}                                       # }
            ")
            .unwrap();
        static ref INVALID_PARAM_RE: Regex = Regex::new(r"[\{\},]").unwrap();
    }

    match PARAM_RE.captures(param) {
        None => {
            match INVALID_PARAM_RE.find(param) {
                None => Ok(PathParam {
                    name:       param,
                    data_type:  "const",
                }),
                Some(_) => Err(PathParamError::IllFormedParam(param.to_string())),
            }
        },
        Some(m) => Ok(PathParam {
            name:       m.name("name").unwrap().as_str(),
            data_type:  m.name("data_type").unwrap().as_str(),
        })
    }
}

fn parse_api_path(path: &str) -> Result<Vec<PathParam>, PathsError> {
    if path.is_empty() {
        return Err(PathsError::EmptyPath);
    }

    if path.chars().nth(0).unwrap() != '/' {
        return Err(PathsError::StartSlash(path.to_string()));
    }

    path[1..].split("/")
        .map(|p| Ok(parse_path_param(p)?) )
        .collect()
}

fn parse_query_param(query: &str) -> Result<QueryParam, QueryParamError> {
    if query.is_empty() {
        return Err(QueryParamError::EmptyParam);
    }

    let args: Vec<&str> = query.split("=").collect();

    match args.len() {
        2 => Ok(QueryParam { name: args[0], data_type: args[1] }),
        _ => Err(QueryParamError::IllFormedParam(query.to_string()))
    }
}

fn parse_api_queries(queries: &str) -> Result<Vec<QueryParam>, QueriesError> {
    if queries.is_empty() {
        return Err(QueriesError::EmptyQueries);
    };

    queries.split("&")
        .map(|q| Ok(parse_query_param(q)?))
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_correct_api() {
        let mut query;
        let mut result;

        query = "http://spotify.com|v1/helloworld/myman/mwhahahah";
        result = parse_api_url(query);
        assert!(result == ApiGet {
            url: "http://spotify.com",
            ver: "v1",
            paths: Vec::from([
                 PathParam {
                     name: "helloworld",
                     data_type: "const",
                 },
                 PathParam {
                     name: "myman",
                     data_type: "const",
                 },
                 PathParam {
                     name: "mwhahahah",
                     data_type: "const",
                 }
            ]),
            queries: None,
        });

        query = "https://spotify.com|v4/hello-world/{artist,world}?bonvoyage=3&john=3";
        result = parse_api_url(query);
        assert!(result == ApiGet {
            url: "https://spotify.com",
            ver: "v4",
            paths: Vec::from([
                 PathParam {
                     name: "hello-world",
                     data_type: "const",
                 },
                 PathParam {
                     name: "artist",
                     data_type: "world",
                 },
            ]),
            queries: Some(Vec::from([
                QueryParam {
                    name: "bonvoyage",
                    data_type: "3",
                },
                QueryParam {
                    name: "john",
                    data_type: "3",
                },
            ])),
        });

        query = "https://www.googleapis.com/youtube|v3/channels";
        result = parse_api_url(query);
        assert!(result == ApiGet {
            url: "https://www.googleapis.com/youtube",
            ver: "v3",
            paths: Vec::from([
                 PathParam {
                     name: "channels",
                     data_type: "const",
                 }]),
            queries: None,
        });

        query = "https://graph.microsoft.com|v1.0/me/messages?filter=emailAddress";
        result = parse_api_url(query);
        assert!(result == ApiGet {
            url: "https://graph.microsoft.com",
            ver: "v1.0",
            paths: Vec::from([
                 PathParam {
                     name: "me",
                     data_type: "const",
                 },
                 PathParam {
                     name: "messages",
                     data_type: "const",
                 },
            ]),
            queries: Some(Vec::from([
                QueryParam {
                    name: "filter",
                    data_type: "emailAddress",
                },
            ])),
        });

        query = "https://api.ticktick.com/open|v1/project/{projectId,string}/task/{taskId,string}";
        result = parse_api_url(&query);
        assert!(result == ApiGet {
            url: "https://api.ticktick.com/open",
            ver: "v1",
            paths: Vec::from([
                 PathParam {
                     name: "project",
                     data_type: "const",
                 },
                 PathParam {
                     name: "projectId",
                     data_type: "string",
                 },
                 PathParam {
                     name: "task",
                     data_type: "const",
                 },
                 PathParam {
                     name: "taskId",
                     data_type: "string",
                 },
            ]),
            queries: None,
        });
            
    }

    #[test]
    fn parse_correct_paths() {
        let mut path;
        let mut parse_path;
        path = "/hello/world/how";
        parse_path = parse_api_path(&path).unwrap();
        assert!(parse_path.len() == 3);
        assert!(parse_path[0] == PathParam { name: "hello", data_type: "const"});
        assert!(parse_path[1] == PathParam { name: &"world", data_type: &"const"});
        assert!(parse_path[2] == PathParam { name: &"how", data_type: &"const"});

        path = "/hello/{artist,spotify_artist_id}/tbd";
        parse_path = parse_api_path(&path).unwrap();
        assert!(parse_path.len() == 3);
        assert!(parse_path[0] == PathParam { name: &"hello", data_type: &"const"});
        assert!(parse_path[1] == PathParam { name: &"artist", data_type: &"spotify_artist_id"});
        assert!(parse_path[2] == PathParam { name: &"tbd", data_type: &"const"});
    }

    #[test]
    fn parse_incorrect_paths() {
        let mut path;
        let mut parse_path;

        path = "/hello/world/";
        parse_path = parse_api_path(&path).unwrap_err();
        assert!(parse_path == PathsError::from(PathParamError::EmptyParam));

        path = "/hello//world";
        parse_path = parse_api_path(&path).unwrap_err();
        assert!(parse_path == PathsError::from(PathParamError::EmptyParam));

        path = "/hello/world,hi";
        parse_path = parse_api_path(&path).unwrap_err();
        assert!(parse_path == PathsError::from(PathParamError::IllFormedParam("world,hi".to_string())));

        path = "/hello/{world";
        parse_path = parse_api_path(&path).unwrap_err();
        assert!(parse_path == PathsError::from(PathParamError::IllFormedParam("{world".to_string())));
        
        path = "";
        parse_path = parse_api_path(&path).unwrap_err();
        assert!(parse_path == PathsError::EmptyPath);

        path = "hello/world";
        parse_path = parse_api_path(&path).unwrap_err();
        assert!(parse_path == PathsError::StartSlash("hello/world".to_string()));
    }

    #[test]
    fn parse_correct_query() {
        let mut query;
        let mut queries;

        query = "john=int&offset=str";
        queries = parse_api_queries(&query).unwrap();
        assert!(queries.len() == 2);
        assert!(queries[0] == QueryParam { name: "john", data_type: "int"} );
        assert!(queries[1] == QueryParam { name: "offset", data_type: "str" } );

        query = "artist_id=int";
        queries = parse_api_queries(&query).unwrap();
        assert!(queries.len() == 1);
        assert!(queries[0] == QueryParam { name: "artist_id", data_type: "int"} );
    }

    #[test]
    fn parse_incorrect_queries() {
        let mut query;
        let mut queries;

        query = "";
        queries = parse_api_queries(&query).unwrap_err();
        assert!(queries == QueriesError::EmptyQueries);

        query = "&";
        queries = parse_api_queries(&query).unwrap_err();
        assert!(queries == QueriesError::from(QueryParamError::EmptyParam));


        query = "&test=hello";
        queries = parse_api_queries(&query).unwrap_err();
        assert!(queries == QueriesError::from(QueryParamError::EmptyParam));

        query = "test=hello&";
        queries = parse_api_queries(&query).unwrap_err();
        assert!(queries == QueriesError::from(QueryParamError::EmptyParam));

        query = "lo&hello=hi";
        queries = parse_api_queries(&query).unwrap_err();
        assert!(queries == QueriesError::from(QueryParamError::IllFormedParam("lo".to_string())));
    }

    #[test]
    fn verify_query() {

    }
}

use uuid::Uuid;

pub struct PostDataType {
    description: String,
    data_name: String,
    data_type: String,
}

/// Response schema
///
pub struct PostResponseSchema {
    name: String,
    schemas: Vec<PostResponseDataSchema>,
    data: Vec<PostDataType>,
}

pub struct PostResponseDataSchema {
    is_array: bool,
    schema: PostResponseSchema,
}

pub struct PostResponseDataType {
    is_array: bool,
    response_type: PostDataType,
}

pub struct PostApiQuery {
    api_url: String,
    version: String,
    paths: Vec<PostDataType>,
    queries: Option<Vec<PostDataType>>,
    headers: Option<Vec<PostDataType>>,
    response: Vec<PostResponseSchema>,
    description: String,
    reference_url: String,
}

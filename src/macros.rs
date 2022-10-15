macro_rules! resp {
    ($response:expr, $data:expr, $message:expr) => {{
        $response.json(serde_json::json!({"data": $data, "message": $message, "error": false}))
    }};

    ($response:expr, $data:expr) => {{
        $response.json(serde_json::json!({"data": $data, "message": None::<String>, "error": false}))
    }};

    ($response:expr; $message:expr) => {{
        $response.json(serde_json::json!({"data": None::<String>, "message": $message, "error": false}))
    }};

    ($response:expr) => {{
        $response.json(serde_json::json!({"data": None::<String>, "message":  None::<String>, "error": false}))
    }};
}

macro_rules! eresp {
    ($response:expr, $data:expr, $message:expr) => {{
        $response.json(serde_json::json!({"data": $data, "message": $message, "error": true}))
    }};

    ($response:expr, $data:expr) => {{
        $response.json(serde_json::json!({"data": $data, "message": None::<String>, "error": true}))
    }};

    ($response:expr; $message:expr) => {{
        $response.json(serde_json::json!({"data": None::<String>, "message": $message, "error": true}))
    }};

    ($response:expr) => {{
        $response.json(serde_json::json!({"data": None::<String>, "message":  None::<String>, "error": true}))
    }};
}

macro_rules! auth {
    ($req:expr) => {{
        match crate::auth::auth($req).await {
            Ok(x) => x,
            Err(e) => return crate::macros::eresp!(HttpResponse::Unauthorized(); e),
        }
    }};
}

pub(crate) use auth;
pub(crate) use eresp;
pub(crate) use resp;

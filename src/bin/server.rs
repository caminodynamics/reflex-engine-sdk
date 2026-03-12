use anyhow::{anyhow, Context, Result};
use reflex_engine::{summarize_reasons, validate_event_and_persist, Event, Policy};
use serde::Serialize;
use tiny_http::{Header, Method, Response, Server, StatusCode};

#[derive(Serialize)]
struct ValidateEventResponse {
    decision: String,
    reason: String,
    policy_id: String,
    artifact_version: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn json_response<T: Serialize>(status: u16, payload: &T) -> Response<std::io::Cursor<Vec<u8>>> {
    let body = serde_json::to_vec(payload).expect("json serialization should succeed");
    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
        .expect("content-type header should be valid");

    Response::from_data(body)
        .with_status_code(StatusCode(status))
        .with_header(header)
}

fn main() -> Result<()> {
    let policy = Policy::load("demo-policy.json").context("failed to load demo-policy.json")?;
    let addr = std::env::var("REFLEX_ADDR").unwrap_or_else(|_| "127.0.0.1:18080".to_string());
    let server =
        Server::http(&addr).map_err(|err| anyhow!("failed to bind validator server: {err}"))?;

    println!("Reflex validator server listening on http://{addr}");
    println!("POST /validate");

    for mut request in server.incoming_requests() {
        let response = if request.method() == &Method::Post && request.url() == "/validate" {
            let mut body = String::new();

            match request.as_reader().read_to_string(&mut body) {
                Ok(_) => match serde_json::from_str::<Event>(&body) {
                    Ok(event) => match validate_event_and_persist(&event, &policy) {
                        Ok(validation) => json_response(
                            200,
                            &ValidateEventResponse {
                                decision: validation.decision.outcome.to_lowercase(),
                                reason: summarize_reasons(&validation.decision.reasons),
                                policy_id: validation.artifact.policy_id,
                                artifact_version: validation.artifact.artifact_version,
                            },
                        ),
                        Err(err) => json_response(
                            500,
                            &ErrorResponse {
                                error: err.to_string(),
                            },
                        ),
                    },
                    Err(err) => json_response(
                        400,
                        &ErrorResponse {
                            error: format!("invalid request body: {err}"),
                        },
                    ),
                },
                Err(err) => json_response(
                    400,
                    &ErrorResponse {
                        error: format!("failed to read request body: {err}"),
                    },
                ),
            }
        } else {
            json_response(
                404,
                &ErrorResponse {
                    error: "not found".to_string(),
                },
            )
        };

        let _ = request.respond(response);
    }

    Ok(())
}

use std::error::Error;
use reqwest::Response;
use serde_json::json;
use supervisor::NodeJsProc;

const SSR_ADDR: &'static str = "localhost:1000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let nodejs = match NodeJsProc::start_with_server_url(
        "nodejs-server/dist/ssr/ssr.js".into(),
        SSR_ADDR.into(),
    ) {
        Err(err) => return Err(err.into()),
        Ok(process) => process,
    };
    
    let resp = match make_ssr_req().await {
        Err(err) => {
            println!("req err: {:#?}", err);
            return Ok(());
        },
        Ok(resp) => resp,
    };

    let json_parsed = resp.json::<inertia::InertiaAppResponse>().await;
    
    if json_parsed.is_err() {
        println!("json err: {:#?}", json_parsed.unwrap_err());
        return Ok(());
    }

    let mut json_parsed = json_parsed.unwrap();

    json_parsed.body = json_parsed.body.replace("&quot;", "\"").replace("<!-- -->", "");

    assert_eq!(json_parsed, inertia::InertiaAppResponse {
        body: "<div id=\"app\" data-page=\"{\"component\":\"Index\",\"props\":{\"auth\":{\"user\":\"Inertia-Rust\"}},\"url\":\"/\",\"version\":null}\"><h1>Hello Inertia-Rust</h1></div>".into(),
        head: vec![
            "<meta name=\"description\" content=\"Just a mocked head... Ha!\" inertia>".into(),
            "<title inertia>Hello inertia!</title>".into(),
        ]
    });

    println!("{:#?}", json_parsed);

    // if we dont kill it, it will evaluate from child process to adult!!!
    let _ = nodejs.kill().await;
    Ok(())
}

async fn make_ssr_req() -> Result<Response, reqwest::Error> {
    let mut props: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    props.insert("auth".into(), json!({
        "user": "Inertia-Rust"
    }));

    let page = inertia::InertiaPage {
        component: "Index".into(),
        url: "/".into(),
        props,
        version: None,
    };

    let render_endpoint = format!("http://{}/render", SSR_ADDR);
    
    reqwest::Client::new()
        .get(render_endpoint)
        .json(&page)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await
}

async fn _reqwest() -> Result<(), Box<dyn Error>> {
    let resp = make_ssr_req().await;

    if resp.is_err() {
        println!("req err: {:#?}", resp.unwrap_err());
        return Ok(());
    }

    let resp = resp.unwrap();

    let json_parsed = resp.json::<inertia::InertiaAppResponse>().await;
    
    if json_parsed.is_err() {
        println!("json err: {:#?}", json_parsed.unwrap_err());
        return Ok(());
    }

    let mut json_parsed = json_parsed.unwrap();

    json_parsed.body = json_parsed.body.replace("&quot;", "\"").replace("<!-- -->", "");

    assert_eq!(json_parsed, inertia::InertiaAppResponse {
        body: "<div id=\"app\" data-page=\"{\"component\":\"Index\",\"props\":{\"auth\":{\"user\":\"Inertia-Rust\"}},\"url\":\"/\",\"version\":null}\"><h1>Hello Inertia-Rust</h1></div>".into(),
        head: vec![
            "<meta name=\"description\" content=\"Just a mocked head... Ha!\" inertia>".into(),
            "<title inertia>Hello inertia!</title>".into(),
        ]
    });

    println!("{:#?}", json_parsed);

    Ok(())
}

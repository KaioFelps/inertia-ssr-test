use reqwest::{Response, Url};
use serde_json::json;
use supervisor::NodeJsProc;

static SSR_ADDR: (&str, u16) = ("localhost", 1000);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let address = format_url(SSR_ADDR.0, &SSR_ADDR.1)?;

    let nodejs = match NodeJsProc::start_with_server_url(
        "nodejs-server/dist/ssr/ssr.js".into(),
        address.to_string(),
    ) {
        Err(err) => return Err(err.into()),
        Ok(process) => process,
    };
    
    let resp = match make_ssr_req(&address).await {
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

fn format_url(host: &str, port: &u16) -> Result<Url, String> {
    let host = if host.contains("://") { format!("{}:{}", host, port) } else { format!("http://{}:{}", host, port) };

    match Url::parse(&host) {
        Err(err) => return Err(format!("Some better error explaining that url is invalid: {}", err.to_string())),
        Ok(url) => Ok(url),
    }
}


async fn make_ssr_req(addr: &Url) -> Result<Response, reqwest::Error> {
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

    let mut render_endpoint = addr.clone();
    render_endpoint.set_path("render");
    
    reqwest::Client::new()
        .get(render_endpoint)
        .json(&page)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await
}

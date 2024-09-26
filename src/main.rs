use serde_json::json;

const SSR_ADDR: &'static str = "localhost:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut props = serde_json::Map::new();
    props.insert("auth".into(), json!({
        "user": "Kaio"
    }));

    let page = inertia::InertiaPage {
        component: "Index".into(),
        url: "/".into(),
        props,
        version: None,
    };

    let render_endpoint = format!("http://{}/render", SSR_ADDR);

    let resp = reqwest::Client::new()
        .get(render_endpoint)
        .json(&page)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await;

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

    let json_parsed = json_parsed.unwrap();

    assert_eq!(json_parsed, inertia::InertiaAppResponse {
        body: "<div id='app' data-page='{\"auth\":{\"user\":\"Kaio\"}}'".into(),
        head: vec![
            "<title inertia>Hello inertia!</title>".into(),
            "<description>Just a mocked head... Ha!</description>".into()
        ]
    });

    println!("{:#?}", json_parsed);

    Ok(())
}

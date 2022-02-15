
#[cfg(test)]
mod tests {
    use crate::say_hello;
    use actix_web::{test, App, web};
    use actix_web_starter::JsonResponse;

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().route("/{name}", web::get().to(say_hello))).await;

        let req = test::TestRequest::get().uri("/gabriel").to_request();
        let resp: JsonResponse = test::read_response_json(&mut app, req).await;

        println!("resp: {:?}", resp);
        assert!(resp.ok);
    }
}

#[cfg(test)]
mod tests {
    use crate::routes::say_hello::say_hello;
    use actix_web::{
        test::{read_response_json, TestRequest},
        {test::init_service, web, App},
    };
    use actix_web_starter::JsonResponse;

    #[actix_rt::test]
    async fn test_index_get() {
        let req = TestRequest::get().uri("/gabriel").to_request();

        let mut srv = init_service(App::new().configure(|cfg| {
            cfg.service(web::resource("/").route(web::get().to(say_hello)))
                .route("/{name}", web::get().to(say_hello));
        }))
        .await;

        let resp: JsonResponse = read_response_json(&mut srv, req).await;

        println!("{:?}", resp);
    }
}

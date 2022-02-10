use actix_web::{ get,web, HttpResponse, Responder};





// #[post("/add")]
// async fn add_user(app_data: web::Data<crate::AppState>, data: web::Json<Data>) -> impl Responder {
//     let action = app_data.service_manager.api.create(&data);
//     let result = web::block(move || action).await;
//     match result {
//         Ok(result) => HttpResponse::Ok().json(result.inserted_id),
//         Err(e) => {
//             println!("Error while getting, {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

// #[post("/update/{param}")]
// async fn update_user(app_data: web::Data<crate::AppState>, data: web::Json<Data>, param: web::Path<String>) -> impl Responder {
//     let action = app_data.service_manager.api.update(&data, &param);
//     let result = web::block(move || action).await;
//     match result {
//         Ok(result) => HttpResponse::Ok().json(result.modified_count),
//         Err(e) => {
//             println!("Error while getting, {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

#[get("/print_pos")]
async fn print_pos(app_data: web::Data<crate::AppState>) -> impl Responder {
    let action = app_data.service_manager.api.printer_pos();
    let result = action;
    match result {
        true => HttpResponse::Ok().json(result),
        false => {
            println!("Error while getting");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(print_pos);
}
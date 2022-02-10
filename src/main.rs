
use std::env;
use actix_cors::Cors;
use escpos_rs::{Printer, PrinterProfile,PrintData,Instruction,/*command::Font,Justification*/};
use actix_web::{http,middleware, web, App, HttpRequest, HttpResponse,HttpServer, Responder};

use rusb::{DeviceHandle, Language, UsbContext};
use std::time::Duration;
use serde::{ Serialize, Deserialize};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

struct UsbDevice<T: UsbContext> {
    handle: DeviceHandle<T>,
    language: Language,
    timeout: Duration,
}
#[derive(Serialize)]
struct Device {
    product_id: u16,
    vendor_id: u16,
    name: String,
}

#[derive(Deserialize)]
struct PrinterData{
    text:String,
    qr_content:String,
}


fn vec_to_json(vec:Vec<Device>)->String{
    match serde_json::to_string(&vec){
        Ok(dev)=>dev,
        Err(e) => panic!("Error: {}", e),
    }
    }

async fn list_devices() -> impl Responder {
    let timeout = Duration::from_secs(1);
    let mut devices_list: Vec<Device> =Vec::new();

    for device in rusb::devices().unwrap().iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        let mut usb_device = {
            match device.open() {
                Ok(h) => match h.read_languages(timeout) {
                    Ok(l) => {
                        if l.len() > 0 {
                            Some(UsbDevice {
                                handle: h,
                                language: l[0],
                                timeout,
                            })
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                },
                Err(_) => None,
            }
        };

        devices_list.push(Device {
            product_id: device_desc.product_id(),
            vendor_id:device_desc.vendor_id(),
            
            name: format!(
                "{} {}",
                device_desc.product_string_index().unwrap_or(0),
                usb_device.as_mut().map_or(String::new(), |h| h
                    .handle
                    .read_product_string(h.language, &device_desc, h.timeout)
                    .unwrap_or(String::new()))
            ),
        });
    }
    let json=vec_to_json(devices_list);
    format!("{}",json)
}

async fn printer_qr(req:HttpRequest)->impl Responder{


    let vendor_id=req.match_info().get("vendor_id").unwrap_or("000").to_string().parse::<u16>().unwrap_or(0);
    let product_id=req.match_info().get("product_id").unwrap_or("000").to_string().parse::<u16>().unwrap_or(0);

    let printer_details = PrinterProfile::usb_builder(vendor_id, product_id).build();
    // We pass it to the printer
    let printer = match Printer::new(printer_details) {
        Ok(maybe_printer) => match maybe_printer {
            Some(printer) => printer,
            None => panic!("No printer was found :("),
        },
        Err(e) => panic!("Error: {}", e),
    };
    let instruction =Instruction::dynamic_qr_code("%url%".to_string());
    // let instruction = match Instruction::qr_code("%url%".to_string()){
    //     Ok(qr)=>qr,
    //     Err(e)=> panic!("Error: {}", e),
    // };


    // let instruction2 = Instruction::text(
    //     "Hello, %name%!",
    //     Font::FontA,
    //     Justification::Center,
    //     // Words that will be replaced in this specific instruction
    //     Some(vec!["%name%".into()].into_iter().collect())
    // );
    // let print_data_1 = PrintData::builder().replacement("%name%", "Carlos")
    // .build();

    let print_data_2 = PrintData::builder().add_qr_code("%url%", "https://google.com")
    .build();

    // match printer.instruction(&instruction, Some(&print_data_1)) {
    //     Ok(_) => (), // "Hello, Carlos!" should've been printed.
    //     Err(e) => println!("Error: {}", e)
    // }
    match printer.instruction(&instruction, Some(&print_data_2)) {
        Ok(_) => (), // "Hello, Carlos!" should've been printed.
        Err(e) => println!("Error: {}", e)
    }
    format!("Si imprimio")

}

async fn printer_pos(req: HttpRequest,info:web::Json<PrinterData>) -> impl Responder {
    let text = info.text.to_string();
    let vendor_id=req.match_info().get("vendor_id").unwrap_or("000").to_string().parse::<u16>().unwrap_or(0);
    let product_id=req.match_info().get("product_id").unwrap_or("000").to_string().parse::<u16>().unwrap_or(0);
    // let printer_details = PrinterProfile::usb_builder(0x1fc9, 0x2016).build();
    let printer_details = PrinterProfile::usb_builder(vendor_id, product_id).build();
    // We pass it to the printer
    let printer = match Printer::new(printer_details) {
        Ok(maybe_printer) => match maybe_printer {
            Some(printer) => printer,
            None => panic!("No printer was found :("),
        },
        Err(e) => panic!("Error: {}", e),
    };
    // We print simple text
    match printer.println(text) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
    match printer.raw(vec![0x1d, 0x56, 0x41, 0x0]) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
    format!("Si imprimio")
}

// async fn decode_url(info:web::Json<PrinterData>) -> impl Responder {
//     let text = format!("{}{}",info.text,info.qr_content);
    
//     format!("{}",text)
// }
async fn decode_url(req: HttpRequest,info:web::Json<PrinterData>) -> impl Responder {
    let text = req.match_info().get("id").unwrap_or("World");

    HttpResponse::Ok().body(format!("{} {} {}",info.text.to_string(),info.qr_content.to_string(),text))
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
   
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {

        let cors_middleware = Cors::new()
        .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
        .finish();

        App::new()
            .wrap(cors_middleware)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(greet))
            .route("/printer/{product_id}/{vendor_id}", web::post().to(printer_pos))
            .route("/printerqr/{product_id}/{vendor_id}", web::get().to(printer_qr))
            .route("/decodeurl/{id}", web::post().to(decode_url))
            .route("/devices", web::get().to(list_devices))
    })
    .bind(("127.0.0.1", 5050))?
    .run()
    .await
}

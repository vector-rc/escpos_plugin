use escpos_rs::{Printer, PrinterProfile};

pub struct ApiService {
}

// Functions with quieries to Mongo
impl ApiService {
    pub fn printer_pos(&self) ->bool{
        let printer_details = PrinterProfile::usb_builder(0x1fc9, 0x2016 ).build();
        // We pass it to the printer
        let printer = match Printer::new(printer_details) {
            Ok(maybe_printer) => match maybe_printer {
                Some(printer) => printer,
                None => panic!("No printer was found :("),
            },
            Err(e) => panic!("Error: {}", e),
        };
        // We print simple text
        match printer.println("Hello, world!") {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }
        true

    }
}
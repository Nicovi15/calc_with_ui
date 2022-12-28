#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
//use std::string;
use eframe::egui;
use egui::{FontFamily, FontId, TextStyle};


fn main() {
    //let native_options = eframe::NativeOptions::default();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(280.0, 480.0)),
        icon_data: Some(load_icon(".\\icons8-calculator-50.png")), 
        ..Default::default()
    };
    eframe::run_native("CalculatoR", options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
    value : String,
    previous_value : f64,
    operator : String,
    decimal_part : bool,
}

impl Default for  MyEguiApp {
    fn default() -> Self {
        Self {
            value: String::from("0"),
            previous_value: 0.0,
            operator : String::from(""),
            decimal_part : false,
        }
    }
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Proportional};
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (TextStyle::Button, FontId::new(20.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        Self::default()
    }

    fn digit_button(&mut self, i : &i32){
        if self.value.len() == 1 && self.value.chars().nth(0).unwrap() == '0' {
            self.value = i.to_string();
        }
        else if self.value.len() == 2 && self.value == "-0"{
            self.value = String::from("-") + i.to_string().as_str();
        }
        else{
            self.value += i.to_string().as_str();
        }
    }

    fn save_previous(&mut self){
        self.previous_value = self.value.parse::<f64>().unwrap();
        self.value =  String::from("0");
        self.decimal_part = false;
    }

    fn proceed_calcul(&mut self){
        match self.operator.as_str() {
            "+" => self.value = (self.previous_value + self.value.parse::<f64>().unwrap()).to_string(),
            "-" => self.value = (self.previous_value - self.value.parse::<f64>().unwrap()).to_string(),
            "*" => self.value = (self.previous_value * self.value.parse::<f64>().unwrap()).to_string(),
            "/" => self.value = (self.previous_value / self.value.parse::<f64>().unwrap()).to_string(),
            "%" => self.value = (self.previous_value % self.value.parse::<f64>().unwrap()).to_string(),
            _ => {}
        }
        self.previous_value = 0.0;
        self.operator = String::from("");
        self.decimal_part = self.value.contains('.');
    }

    fn operator_button(&mut self, o : String){
       match o.as_str() {
        "CE" => {
            self.value = String::from("0");
        },
        "C" => {
            self.previous_value = 0.0;
            self.value = String::from("0");
        },
        "<-" => {
            if self.value.len() == 1 {
                self.value = String::from("0");
            }
            else{
                if self.value.remove(self.value.len()-1) == '.'{
                    self.decimal_part = false;
                }
            }
        },
        "1/x" => self.value = (1.0 / self.value.parse::<f64>().unwrap()).to_string(),
        "x^2" => self.value = self.value.parse::<f64>().unwrap().powi(2).to_string(),
        "sqrt" => self.value = self.value.parse::<f64>().unwrap().sqrt().to_string(),
        "+/-" => self.value = (self.value.parse::<f64>().unwrap() * -1.0).to_string(),
        "+" | "-" | "*" | "/" | "%"  => {
            self.proceed_calcul();
            self.operator = o;
            self.save_previous();
        },
        "," => {
            if !self.decimal_part {
                self.decimal_part = true;
                self.value += ".";
            }
        },
        "=" => self.proceed_calcul(),
        _ => println!("operator not found")
       }
       
    }
}

impl eframe::App for MyEguiApp {

   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
        
            if self.previous_value != 0.0 {
                ui.heading(String::from(self.previous_value.to_string() + " ") + self.operator.as_str());
            }
            else{
                ui.heading("");
            }

           ui.heading(self.value.to_string());
           ui.separator();

           ui.horizontal(|ui| {
            if ui.add_sized([60., 60.], egui::Button::new("%")).clicked(){
                self.operator_button( String::from("%"));
            }

            if ui.add_sized([60., 60.], egui::Button::new("CE")).clicked(){
                self.operator_button( String::from("CE"));
            }

            if ui.add_sized([60., 60.], egui::Button::new("C")).clicked(){
                self.operator_button( String::from("C"));
            }

            if ui.add_sized([60., 60.], egui::Button::new("<-")).clicked(){
                self.operator_button( String::from("<-"));
            }
        });

        ui.horizontal(|ui| {
            if ui.add_sized([60., 60.], egui::Button::new("1/x")).clicked(){
                self.operator_button( String::from("1/x"));
            }

            if ui.add_sized([60., 60.], egui::Button::new("x^2")).clicked(){
                self.operator_button( String::from("x^2"));
            }

            if ui.add_sized([60., 60.], egui::Button::new("sqrt")).clicked(){
                self.operator_button( String::from("sqrt"));
            }

            if ui.add_sized([60., 60.], egui::Button::new("/")).clicked(){
                self.operator_button( String::from("/"));
            }
        });

        ui.horizontal(|ui| {

            for i in 7..10{
                if ui.add_sized([60., 60.], egui::Button::new(i.to_string())).clicked(){
                    self.digit_button(&i);
                }
            }

            if ui.add_sized([60., 60.], egui::Button::new("*")).clicked(){
                self.operator_button( String::from("*"));
            }
        });

        ui.horizontal(|ui| {

            for i in 4..7{
                if ui.add_sized([60., 60.], egui::Button::new(i.to_string())).clicked(){
                    self.digit_button(&i);
                }
            }

            if ui.add_sized([60., 60.], egui::Button::new("-")).clicked(){
                self.operator_button( String::from("-"));
            }
        });

        ui.horizontal(|ui| {

            for i in 1..4{
                if ui.add_sized([60., 60.], egui::Button::new(i.to_string())).clicked(){
                    self.digit_button(&i);
                }
            }

            if ui.add_sized([60., 60.], egui::Button::new("+")).clicked(){
                self.operator_button( String::from("+"));
            }
        });

        ui.horizontal(|ui| {
            if ui.add_sized([60., 60.], egui::Button::new("+/-")).clicked(){
                self.operator_button( String::from("+/-"));
            }

            if ui.add_sized([60., 60.], egui::Button::new("0")).clicked(){
                self.digit_button(&0);
            }

            if ui.add_sized([60., 60.], egui::Button::new(",")).clicked(){
                self.operator_button( String::from(","));
            }

            if ui.add_sized([60., 60.], egui::Button::new("=")).clicked(){
                self.operator_button( String::from("="));
            }
        });

       });
   }
}
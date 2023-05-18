use fltk::frame::Frame;
use fltk::menu::Choice;
use fltk::{app::*, prelude::*};
use fltk::{
    app::{self, App},
    browser::HoldBrowser,
    button::Button,
    enums,
    input::Input,
    prelude::GroupExt,
    window::{DoubleWindow, Window},
};

use crate::models::{Home, NewHome};
use crate::service::HomeService;

const WIDGET_WITDTH: i32 = 500;
const WIDGET_HEIGHT: i32 = 600;

fn generete_input_with_label(
    title: &'static str,
    below_of: &Input,
    padding: i32,
    label_width: i32,
) -> Input {
    let label = Frame::new(0, 0, label_width, 30, title).below_of(&below_of.clone(), padding);
    let input = Input::default().with_size(200, 30).below_of(&label, -7);

    input
}

// const TYPE_OPTIONS: Vec<&str> = vec!("Apartamento", "Casa");

const TYPE_OPTIONS: [&'static str; 2] = ["Apartamento", "Casa"];

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
    Clean,
    // Save,
}

pub struct GUI {
    app: App,
    wind: DoubleWindow,
    sender: Sender<Message>,
    receiver: Receiver<Message>,

    street_input: Input,
    number_input: Input,
    floor_input: Input,
    zipcode_input: Input,
    squaremeters_input: Input,
    number_of_bathrooms_input: Input,
    number_of_rooms_input: Input,
    home_type: Choice,
    homes: Vec<Home>,

    filter_input: Input,
    list: HoldBrowser,
    create_button: Button,
    update_button: Button,
    delete_button: Button,
    clear_button: Button,

    home_service: HomeService,
    selected_id: i32,
}

impl GUI {
    pub fn new() -> GUI {
        let app = app::App::default().with_scheme(app::Scheme::Oxy);
        let wind = Window::default().with_label("HomeHub");
        let (sender, receiver) = channel::<Message>();

        let filter_input = Input::new(230, 20, 250, 30, "").with_label("Buscar:");

        // inputs
        let label = Input::new(20, 40, 0, 0, "");

        let street_input: Input = generete_input_with_label("Calle", &label, 10, 30);
        let number_input: Input = generete_input_with_label("Número", &street_input, 4, 50);
        let floor_input: Input = generete_input_with_label("Piso", &number_input, 4, 30);
        let zipcode_input: Input = generete_input_with_label("Código Postal", &floor_input, 4, 85);
        let squaremeters_input: Input =
            generete_input_with_label("Metros cuadrado", &zipcode_input, 4, 105);
        let number_of_bathrooms_input: Input =
            generete_input_with_label("Número de baños", &squaremeters_input, 4, 110);
        let number_of_rooms_input: Input =
            generete_input_with_label("Número de habitaciones", &number_of_bathrooms_input, 4, 155);

        let home_label = Frame::new(0, 0, 30, 30, "Tipo").below_of(&number_of_rooms_input, 4);
        let mut home_type = Choice::new(100, 20, 100, 30, "").below_of(&home_label, -7);

        for option in TYPE_OPTIONS {
            home_type.add_choice(option);
        }

        home_type.set_value(0);

        let create_button = Button::new(100, 20, 100, 30, "Create")
            .with_label("Crear")
            .below_of(&home_type, 30);

        let update_button = Button::new(100, 20, 100, 30, "Create")
            .with_label("Modificar")
            .right_of(&create_button, 10);

        let delete_button = Button::new(100, 20, 100, 30, "Create")
            .with_label("Borrar")
            .right_of(&update_button, 10);

        let clear_button = Button::new(100, 20, 100, 30, "Create")
            .with_label("Limpiar")
            .right_of(&delete_button, 10);

        let list = HoldBrowser::new(100, 20, 250, 400, "").right_of(&street_input, 10);

        let mut home_service = HomeService::new();

        let homes = home_service.get_all().unwrap();

        GUI {
            app,
            wind,
            sender,
            receiver,
            street_input,
            number_input,
            floor_input,
            zipcode_input,
            squaremeters_input,
            number_of_bathrooms_input,
            number_of_rooms_input,
            home_type,
            filter_input,
            list,
            homes,
            create_button,
            update_button,
            delete_button,
            clear_button,
            home_service,
            selected_id: -1,
        }
    }

    pub fn build(&mut self) {
        self.filter_input
            .set_trigger(enums::CallbackTrigger::Changed);
        self.filter_input.emit(self.sender, Message::Filter);

        self.list.emit(self.sender, Message::Select);

        self.create_button.emit(self.sender, Message::Create);
        self.update_button.emit(self.sender, Message::Update);
        self.delete_button.emit(self.sender, Message::Delete);
        self.clear_button.emit(self.sender, Message::Clean);

        self.wind.set_size(WIDGET_WITDTH, WIDGET_HEIGHT);

        self.sender.send(Message::Filter);
    }

    pub fn show(&mut self) {
        self.wind.end();
        self.wind.show();

        while self.app.wait() {
            match self.receiver.recv() {
                Some(Message::Create) => {
                    let street = self.street_input.value();
                    let number = self.number_input.value();
                    let floor = self.floor_input.value();
                    let zipcode = self.zipcode_input.value();
                    let squaremeters = self.squaremeters_input.value();
                    let number_of_bathrooms = self.number_of_bathrooms_input.value();
                    let number_of_rooms = self.number_of_rooms_input.value();
                    let home_type = self.home_type.choice().unwrap();
                    let home = NewHome {
                        street,
                        number,
                        floor,
                        zipcode,
                        squaremeters,
                        number_of_bathrooms,
                        number_of_rooms,
                        home_type,
                    };
                    self.home_service.create(home).expect("error creating");

                    self.homes = self.home_service.get_all().unwrap();

                    self.clear_inputs();
                    self.sender.send(Message::Filter);
                }
                Some(Message::Update) => {
                    if self.selected_id > -1 {
                        let street = self.street_input.value();
                        let number = self.number_input.value();
                        let floor = self.floor_input.value();
                        let zipcode = self.zipcode_input.value();
                        let squaremeters = self.squaremeters_input.value();
                        let number_of_bathrooms = self.number_of_bathrooms_input.value();
                        let number_of_rooms = self.number_of_rooms_input.value();
                        let home_type = self.home_type.choice().unwrap();
                        let home = NewHome {
                            street,
                            number,
                            floor,
                            zipcode,
                            squaremeters,
                            number_of_bathrooms,
                            number_of_rooms,
                            home_type,
                        };
                        self.home_service
                            .update_one(self.selected_id, home)
                            .expect("error updating");
                        self.homes = self.home_service.get_all().unwrap();
                        self.clear_inputs();
                        self.create_button.activate();
                        self.sender.send(Message::Filter);
                        self.sender.send(Message::Select);
                    } else {
                        println!("No hay elementos para actualizar");
                    }
                }
                Some(Message::Delete) => {
                    if self.selected_id > -1 {
                        self.home_service
                            .delete_one(self.selected_id)
                            .expect("error deleting");
                        self.homes = self.home_service.get_all().unwrap();
                        self.clear_inputs();
                        self.create_button.activate();
                        self.sender.send(Message::Filter);
                        self.sender.send(Message::Select);
                    } else {
                        println!("No hay elementos para eliminar");
                    }
                }
                Some(Message::Clean) => {
                    self.clear_inputs();
                    self.create_button.activate();
                    self.update_button.deactivate();
                    self.delete_button.deactivate();
                    self.sender.send(Message::Filter);
                    self.sender.send(Message::Select);
                    self.clear_button.deactivate();
                }
                Some(Message::Select) => {
                    if self.list.value() == 0 {
                        self.update_button.deactivate();
                        self.delete_button.deactivate();
                        self.clear_button.deactivate();
                    } else {
                        let text_selection = self.list.text(self.list.value()).unwrap();
                        let search_result = self
                            .homes
                            .iter()
                            .filter(|e| return e.to_table().eq_ignore_ascii_case(&text_selection))
                            .next();

                        match search_result {
                            Some(home) => {
                                self.selected_id = home.id;
                                self.street_input.set_value(&home.street);
                                self.number_input.set_value(&home.number.to_string());
                                self.floor_input.set_value(&home.floor);
                                self.zipcode_input.set_value(&home.zipcode);
                                self.squaremeters_input
                                    .set_value(&home.squaremeters.to_string());
                                self.number_of_bathrooms_input
                                    .set_value(&home.number_of_bathrooms.to_string());
                                self.number_of_rooms_input
                                    .set_value(&home.number_of_rooms.to_string());

                                let index = TYPE_OPTIONS
                                    .iter()
                                    .position(|&r| r == home.home_type)
                                    .unwrap();
                                self.home_type.set_value(index as i32);

                                self.create_button.deactivate();
                                self.update_button.activate();
                                self.delete_button.activate();
                                self.clear_button.activate();
                            }
                            _ => {
                                println!("No se ha encontrado el elemento");
                            }
                        }
                    }
                }
                Some(Message::Filter) => {
                    let prefix = self.filter_input.value().to_lowercase();
                    let filter_empty = prefix.trim().eq_ignore_ascii_case("");
                    self.list.clear();
                    for home in &self.homes {
                        let home_string = home.to_table();
                        if filter_empty || home_string.to_lowercase().contains(&prefix) {
                            self.list.add(&home_string);
                        }
                    }
                    self.sender.send(Message::Select);
                }
                None => {
                    // println!("None");
                }
            }
        }
    }

    fn clear_inputs(&mut self) {
        self.selected_id = -1;
        self.street_input.set_value("");
        self.number_input.set_value("");
        self.floor_input.set_value("");
        self.zipcode_input.set_value("");
        self.squaremeters_input.set_value("");
        self.number_of_bathrooms_input.set_value("");
        self.number_of_rooms_input.set_value("");
        self.home_type.set_value(0);
    }
}

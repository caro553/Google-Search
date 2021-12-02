/*
 * Binome1 : Carolina Vinkovic
 * binome2 : Tacherifte Aymen Chafik 
*/

/* This project is Creating a graphic interface application with Rust and the GTK-rs library and we had also choosen tu use the Glade tool */

/*In our project we could have made our application more efficient by using the model of the MVC architecture to store the content of the application (explanations of tips) and to control it with the controller to display the different contents and that from the Gkeyfile from the glib library but we didn't have enough time (other projects to do) to ensure the proper functioning of everything so we chose to do only the view and controller parts and make something that works well enough*/


extern crate gtk;
use gtk::prelude::*;


/*####################View#################### */

/* This part of code "view" is only for gerating different views of the application*/

/* This function is for gerating the view of the about part using a glade file */
fn get_about(){
    let file_src = include_str!("about.glade");
    let builder_about = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_about, file_src).expect("Glade file was not found");
    let about: gtk::AboutDialog = builder_about.get_object("about_dialog").unwrap();
    about.show_all();
}

/* This function is for generating the view of the explanation part which is a dialog box that appears when the first button of the application is clicked using a glade file and it's the same case for the next 14 functions*/
fn get_dialog1(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog1").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button1").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view1").unwrap();    
    /* We had choosen to give examples when the example button is clicked by printing it in the console*/
    _button.connect_clicked(move |_| {
        println!("type : Oran santa cruz | monica");
    });
    
    _dialog.show_all(); 
}

fn get_dialog2(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog2").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button2").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view2").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : Carrefour ~ drink");
    });
    
    _dialog.show_all(); 
}

fn get_dialog3(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog3").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button3").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view3").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : www.caf.fr logement");
    });
    
    _dialog.show_all(); 
}

fn get_dialog4(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog4").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button4").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view4").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : France world cup 201*");
    });
    
    _dialog.show_all(); 
}

fn get_dialog5(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog5").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button5").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view5").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : I wandered AROUND(4) cloud");
    });
    
    _dialog.show_all(); 
}

fn get_dialog6(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog6").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button6").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view6").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : Scientific discoveries 1900...2000 (be sure that you have put three dots)");
    });
    
    _dialog.show_all(); 
}

fn get_dialog7(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog7").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button7").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view7").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : intitle:MVC design pattern\n 
                    or type : inurl:MVC design pattern");
    });
    
    _dialog.show_all(); 
}

fn get_dialog8(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog8").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button8").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view8").unwrap(); 
    _button.connect_clicked(move |_| {
        println!("type : related:www.youtube.com");
    });
    
    _dialog.show_all(); 
}

fn get_dialog9(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog9").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button9").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view9").unwrap(); 
    _button.connect_clicked(move |_| {
        println!("type : \"fifa world cup\" ");
    });
    
    _dialog.show_all(); 
}

fn get_dialog10(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog10").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button10").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view10").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : Iphone - buy");
    });
    
    _dialog.show_all(); 
}

fn get_dialog11(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog11").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button11").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view11").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : https://www.google.fr/imghp?hl=fr then upload your image");
    });
    
    _dialog.show_all(); 
}

fn get_dialog12(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog12").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button12").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view12").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : define:fifo\n 
                    or type : etymology:fifo");
    });
    
    _dialog.show_all(); 
}

fn get_dialog13(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog13").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button13").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view13").unwrap();    
    _button.connect_clicked(move |_| {
        println!("type : Gkeyfile filetype:rs");
    });
    
    _dialog.show_all(); 
}

fn get_dialog14(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog14").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button14").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view14").unwrap(); 
    _button.connect_clicked(move |_| {
        println!("type : I had gone yesterday\n
                    you will see in the proposals the correct form of this sentence");
    });
    
    _dialog.show_all(); 
}

fn get_dialog15(){
    let file_src = include_str!("Dialog.glade");
    let builder_dialog = gtk::Builder::new();
    BuilderExt::add_from_string(&builder_dialog, file_src).expect("Glade file was not found");
    let _dialog: gtk::Dialog = builder_dialog.get_object("dialog15").unwrap();
    let _button: gtk::Button = builder_dialog.get_object("button15").unwrap();
    let _text_view: gtk::TextView = builder_dialog.get_object("text_view15").unwrap(); 
    _button.connect_clicked(move |_| {
        println!("This is just an example of order number of a online purchase : PQ0345");
    });
    
    _dialog.show_all(); 
}

/* This function is for generating the view of the main page of the application which contains titled clickable buttons to get an explanations about each trick for the GoogleSearch */
fn get_main(){
    /* This is a test to manage the failure of the inistialisation of GTK */
    if gtk::init().is_err(){
        println!("inistialisation of GTK failed");
        return;
    }
    /* This is the name of the glade file */
    let glade_src = include_str!("GoogleSearch.glade");
    /* This is a builder to get the objects which are defined in the glade (XML) file */
    let builder = gtk::Builder::new();
    /* Next step is adding the glade content to the builder with managing the failure of not founding the glade file */
    BuilderExt::add_from_string(&builder, glade_src).expect("Glade file was not found");
    /* Then we get each object after defining variable for it */
    let main_window: gtk::Window = builder.get_object("main_window").unwrap();
    let _about_button: gtk::Button = builder.get_object("about_button").unwrap();
    let _box: gtk::Button = builder.get_object("button1").unwrap();
    let _button1: gtk::Button = builder.get_object("button1").unwrap();
    let _button2: gtk::Button = builder.get_object("button2").unwrap();
    let _button3: gtk::Button = builder.get_object("button3").unwrap();
    let _button4: gtk::Button = builder.get_object("button4").unwrap();
    let _button5: gtk::Button = builder.get_object("button5").unwrap();
    let _button6: gtk::Button = builder.get_object("button6").unwrap();
    let _button7: gtk::Button = builder.get_object("button7").unwrap();
    let _button8: gtk::Button = builder.get_object("button8").unwrap();
    let _button9: gtk::Button = builder.get_object("button9").unwrap();
    let _button10: gtk::Button = builder.get_object("button10").unwrap();
    let _button11: gtk::Button = builder.get_object("button11").unwrap();
    let _button12: gtk::Button = builder.get_object("button12").unwrap();
    let _button13: gtk::Button = builder.get_object("button13").unwrap();
    let _button14: gtk::Button = builder.get_object("button14").unwrap();
    let _button15: gtk::Button = builder.get_object("button15").unwrap();
/* This is for the call-back of the delete event to quit the application */
    main_window.connect_delete_event(|_, _| {
        gtk::main_quit(); 
        Inhibit(false)
    });

/* Next part of this function are calls of the previous functions when collect the click event */
    _about_button.connect_clicked(move |_| {
        get_about();
    });

    _button1.connect_clicked(move |_| {
        get_dialog1();
    });
    _button2.connect_clicked(move |_| {
        get_dialog2();
    });
    _button3.connect_clicked(move |_| {
        get_dialog3();
    });
    _button4.connect_clicked(move |_| {
        get_dialog4();
    });
    _button5.connect_clicked(move |_| {
        get_dialog5();
    });
    _button6.connect_clicked(move |_| {
        get_dialog6();
    });
    _button7.connect_clicked(move |_| {
        get_dialog7();
    });
    _button8.connect_clicked(move |_| {
        get_dialog8();
    });
    _button9.connect_clicked(move |_| {
        get_dialog9();
    });
    _button10.connect_clicked(move |_| {
        get_dialog10();
    });
    _button11.connect_clicked(move |_| {
        get_dialog11();
    });
    _button12.connect_clicked(move |_| {
        get_dialog12();
    });
    _button13.connect_clicked(move |_| {
        get_dialog13();
    });
    _button14.connect_clicked(move |_| {
        get_dialog14();
    });
    _button15.connect_clicked(move |_| {
        get_dialog15();
    });

    main_window.show_all();
    gtk::main();
}


/*####################controller#################### */

/* This part of code is to control different parts of code (modele and view) and also the user interaction */

/* This function is a controller function which contain a call function for the view part to start the application and display the main page */
fn start_application(){
    get_main();
}

/*####################Main#################### */

/* The main function calls the controller to start the application */

fn main() {
    start_application();
}

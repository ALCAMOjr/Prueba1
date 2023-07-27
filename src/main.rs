use gtk::prelude::*;
use gtk::{Label, Window, WindowType, Button, Entry};
use gtk::CssProviderExt;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Calculadora");
    window.set_default_size(350, 150);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    window.add(&vbox);

    // Create a label to display the initial message
    let initial_message = "Ingresa 2 números y un operador separados.";
    let label = Label::new(Some(initial_message));
    vbox.pack_start(&label, false, false, 0);

    let entry = Entry::new(); // Create an Entry widget for user input
    vbox.pack_start(&entry, false, false, 0);

    let button = Button::with_label("Calcular");
    vbox.pack_start(&button, false, false, 0);

    let css_provider = gtk::CssProvider::new();
    let style_data = "
    label {
        font-size: 18px;
        color: #fff; /* Cambiamos el color a blanco */
    }
    entry {
        font-size: 16px;
        padding: 8px;
        border: 1px solid #ccc;
        color: #fff; /* Cambiamos el color a blanco */
    }
    button {
        font-size: 18px;
        background-color: #007bff;
        color: #fff; /* Cambiamos el color a blanco */
        padding: 10px 20px;
        border: none;
        border-radius: 4px;
    }
    button:hover {
        background-color: #0056b3;
    }
";
    css_provider.load_from_data(style_data.as_bytes()).expect("Failed to load CSS data");

    let screen = window.get_screen().expect("Error getting window screen");
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    button.connect_clicked(move |_| {
        let input = entry.get_text().to_string();
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        if tokens.len() < 3 || tokens.len() % 2 == 0 {
            label.set_text("Entrada inválida. Debe ingresar al menos dos números y un operador.");
            return;
        }

        let mut result: f32 = tokens[0].parse().expect("Error al parsear el número");

        for chunk in tokens[1..].chunks(2) {
            let operator = chunk[0].chars().next().expect("Operador inválido");
            let number: f32 = chunk[1].parse().expect("Error al parsear el número");

            result = operate(operator, result, number);
        }

        label.set_text(&format!("Resultado: {:.2}", result));
    });

    window.show_all();
    gtk::main();
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        'X' | 'x' | '*' => first_number * second_number,
        _ => panic!("Operador inválido."),
    }
}



#[cfg(test)]
mod tests {

    use crate::operate;

    #[test]
    fn operate_accepts_floating_point_numbers() {
        let op = operate('+', -10.5, 10.0);
        assert_eq!(op, -0.5);
    }
    #[test]
    fn operate_handles_addition() {
        let op = operate('+', -5.0, 200.0);
        assert_eq!(op, 195.0);
    }

    #[test]
    fn operate_handles_subtraction() {
        let op = operate('-', -12.0, -12.0);
        assert_eq!(op, 0.0);
    }

    #[test]
    fn operate_handles_division() {
        let op = operate('/', -12.0, -1.0);
        assert_eq!(op, 12.0);
    }

    #[test]
    fn operate_handles_multiplication() {
        let op = operate('*', -12.0, -2.0);
        assert_eq!(op, 24.0);
    }

    #[test]
    fn operate_handles_multiplication_x() {
        let op = operate('x', -12.0, 2.0);
        assert_eq!(op, -24.0);
    }

    #[test]
    fn operate_handles_multiplication_cap_x() {
        let op = operate('X', -12.0, 2.0);
        assert_eq!(op, -24.0);
    }

    #[test]
    #[should_panic]
    fn operate_panics_on_invalid_op() {
        operate('a', 1.0, 1.0);
    }
}
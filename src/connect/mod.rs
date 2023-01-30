fn main() {
    let connections: Vec<&'static str> = vec!["Connection 1", "Connection 2"];

    let mut connection_menu_items: Vec<terminal_menu::TerminalMenuItem> =
        vec![terminal_menu::label("Connections")];

    for i in 0..connections.len() {
        connection_menu_items.push(terminal_menu::button(connections[i]));
    }

    let connection_menu = terminal_menu::menu(connection_menu_items);

    terminal_menu::run(&connection_menu);

    println!(
        "Final: {}",
        terminal_menu::mut_menu(&connection_menu).selected_item_name()
    );

    println!(
        "Final: {}",
        terminal_menu::mut_menu(&connection_menu).selected_item_index()
    );
}

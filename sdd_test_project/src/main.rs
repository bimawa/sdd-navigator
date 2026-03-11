mod utils;

// @req: USR-001 - User must be able to register
fn register_user(username: &str, email: &str) -> bool {
    println!("Registering user: {}", username);
    let _ = email;
    true
}

// @req: USR-002 - User must be able to log in
fn login_user(username: &str, password: &str) -> bool {
    println!("Logging in user: {}", username);
    let _ = password;
    true
}

// @req: PRD-001 - Product list must be displayed
fn get_products_list() -> Vec<String> {
    println!("Fetching product list");
    vec!["Product A".to_string(), "Product B".to_string()]
}

fn calculate_total(items: &[f64]) -> f64 {
    items.iter().sum()
}

// @req: ORD-001 - Order placement process must be supported
fn place_order(user_id: u32, product_ids: &[u32]) -> bool {
    println!("Placing order for user {}", user_id);
    let _ = product_ids;
    true
}

fn main() {
    register_user("testuser", "test@example.com");
    login_user("testuser", "password123");
    get_products_list();
    let _ = calculate_total(&[10.0, 20.0, 5.0]);
    place_order(1, &[101, 102]);

    utils::format_date(1678886400);
    utils::helper_function();
    let is_valid = utils::validate_email("test@example.com");
    println!("Email valid: {}", is_valid);
}

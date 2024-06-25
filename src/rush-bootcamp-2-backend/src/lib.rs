use std::cell::RefCell;

// Dane mają być przetwarzane lokalnie
thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default();
}

// Powitanie (query odczyt)
#[ic_cdk::query]
fn greet(name: String, last_name: String) -> String {
    format!("Hello, {} {}!", name, last_name)
}

// Zapis rekordu (update zapis)
#[ic_cdk::update]
fn dodaj_wpis(wpis: String) {
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        return wpisy.borrow_mut().push(wpis);
    });
}

// Listowanie rekordów
#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<String> {
    return WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        return wpisy.borrow().clone();
    });
}
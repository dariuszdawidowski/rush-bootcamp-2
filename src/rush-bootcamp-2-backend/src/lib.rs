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

// Kasowanie rekordów
#[ic_cdk::update]
fn usun_wpis(id_wpisu: usize) {
    WPISY.with(|wpisy: &RefCell<Vec<String>>| {
        wpisy.borrow_mut().remove(id_wpisu)
    });
}

// Edycja rekordów
#[ic_cdk::update]
fn edytuj_wpis(id_wpisu: usize, nowy_wpis: String) {
    WPISY.with(|wpisy| {
        let mut binding = wpisy.borrow_mut();
        let wpis = binding.get_mut(id_wpisu);
        let stary_wpis = wpis.unwrap();
        *stary_wpis = nowy_wpis;
    });
}

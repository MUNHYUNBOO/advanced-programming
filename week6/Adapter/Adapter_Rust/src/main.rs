trait UserProvider {
    fn fetch(&self) -> String;
}

struct Internal;

impl UserProvider for Internal {
    fn fetch(&self) -> String {
        "get user info (from Internal)".to_string()
    }
}

struct External;

impl External {
    fn search(&self) -> String {
        "get user info (from External)".to_string()
    }
}

// 어댑터 구조체
struct Adapter<'a> {
    external: &'a External,
}

impl<'a> UserProvider for Adapter<'a> {
    fn fetch(&self) -> String {
        self.external.search()
    }
}

fn main() {
    let internal = Internal;
    println!("Internal: {}", internal.fetch());

    let external = External;
    let adapter = Adapter { external: &external };
    println!("External through Adapter: {}", adapter.fetch());
}

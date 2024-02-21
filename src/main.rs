mod storage;

use cli_kv::colors::Color::{Cyan, Green, Red, Yellow};
use storage::Storage;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let cmd: &str = args.get(1).unwrap();
    if cmd == "help" || cmd == "-h" || cmd == "h" || cmd == "--help" {
        #[rustfmt::skip]
        println!("{}{}{}{}",
        format!("{}\t{}\n\t\t{}\n", Yellow.new("update key:"), Cyan.new("kv upd key newvalue"), Cyan.new("kv upd key + value from clipboard")),
        format!("{}\t{}\n\t\t{}\n", Yellow.new("remove key:"), Cyan.new("kv rm key"), Cyan.new("kv rm + value from clipboard")),
        format!("{}\t{}\n\t\t{}\n", Yellow.new("add key:"), Cyan.new("kv add key value"), Cyan.new("kv add key + value from clipboard")),
        format!("{}\t{}\n\t\t{}\n", Yellow.new("get key:"), Cyan.new("kv get key"), Cyan.new("kv get + value from clipboard, value will be stored to clipboard")));
        return;
    }

    if !(cmd == "add" || cmd == "upd" || cmd == "get" || cmd == "rm") {
        #[rustfmt::skip]
        println!("{}{}{}", Red.new("unsupported cmd `"), Cyan.new(cmd), Red.new("`"));
        return;
    }

    let mut storage = match Storage::new() {
        Ok(s) => s,
        Err(err) => {
            #[rustfmt::skip]
            println!("{} {}", Red.new("can't read storage from disk"), Cyan.new(&err.to_string()));
            return;
        }
    };

    if cmd == "add" {
        if args.len() == 4 {
            let key: &str = args.get(2).unwrap();
            let val: &str = args.get(3).unwrap();
            storage.add(key, val);
            #[rustfmt::skip]
            println!("{} {} {} {}", Green.new("value"), Cyan.new(val), Green.new("added with key"), Cyan.new(key));
            return;
        }

        if args.len() == 3 {
            let key = args.get(3).unwrap();

            match terminal_clipboard::get_string() {
                Ok(v) => {
                    if v.len() == 0 {
                        println!("{}", Red.new("clipboard is empty"));
                        return;
                    }
                    storage.add(key, v.as_str());
                    #[rustfmt::skip]
                    println!("{} {} {} {}", Green.new("value"), Cyan.new(&v), Green.new("added with key"), Cyan.new(key));
                    return;
                }
                Err(err) => {
                    #[rustfmt::skip]
                    println!("{} {}", Red.new("can't extract value from clipboard"), Cyan.new(&err.to_string()));
                    return;
                }
            }
        }
    }

    if cmd == "upd" {
        if args.len() == 4 {
            let key = args.get(2).unwrap();
            let val = args.get(3).unwrap();
            storage.update(key, val);
            #[rustfmt::skip]
            println!("{} {} {} {}", Green.new("value"), Cyan.new(val), Green.new("updated for key"), Cyan.new(key));
            return;
        }

        if args.len() == 3 {
            let key = args.get(2).unwrap();

            match terminal_clipboard::get_string() {
                Ok(v) => {
                    if v.len() == 0 {
                        println!("{}", Red.new("clipboard is empty"));
                        return;
                    }
                    storage.update(key, v.as_str());
                    #[rustfmt::skip]
                    println!("{} {} {} {}", Green.new("value"), Cyan.new(&v), Green.new("updated for key"), Cyan.new(key));
                    return;
                }
                Err(err) => {
                    #[rustfmt::skip]
                    println!("{} {}", Red.new("can't extract value from clipboard"), Cyan.new(&err.to_string()));
                    return;
                }
            }
        }
    }

    if cmd == "get" {
        if args.len() == 3 {
            let key = args.get(2).unwrap();
            match storage.get(key) {
                Some(val) => {
                    terminal_clipboard::set_string(val).unwrap();
                    #[rustfmt::skip]
                    println!("{} {} {}", Green.new("got"), Cyan.new(val), Green.new("and copied to clipboard"));
                    return;
                }
                None => {
                    println!("{}", Red.new("no data found"));
                    return;
                }
            }
        }

        if args.len() == 2 {
            match terminal_clipboard::get_string() {
                Ok(key) => {
                    if key.len() == 0 {
                        println!("{}", Red.new("clipboard is empty"));
                        return;
                    }

                    match storage.get(&key) {
                        Some(val) => {
                            terminal_clipboard::set_string(val).unwrap();
                            #[rustfmt::skip]
                            println!("{} {} {}", Green.new("got"), Cyan.new(val), Green.new("and copied to clipboard"));
                            return;
                        }
                        None => {
                            println!("{}", Red.new("no data found"));
                            return;
                        }
                    }
                }
                Err(err) => {
                    #[rustfmt::skip]
                    println!("{} {}", Red.new("can't extract value from clipboard"), Cyan.new(&err.to_string()));
                    return;
                }
            }
        }
    }

    if cmd == "rm" {
        if args.len() == 3 {
            let key: &str = args.get(2).unwrap();
            storage.remove(key);
            println!("{} {}", Green.new("removed value by key"), Cyan.new(key));
            return;
        }

        if args.len() == 2 {
            match terminal_clipboard::get_string() {
                Ok(key) => {
                    if key.len() == 0 {
                        println!("{}", Red.new("clipboard is empty"));
                        return;
                    }
                    storage.remove(&key);
                    println!("{} {}", Green.new("removed value by key"), Cyan.new(&key));
                    return;
                }
                Err(err) => {
                    #[rustfmt::skip]
                    println!("{} {}", Red.new("can't extract value from clipboard"), Cyan.new(&err.to_string()));
                    return;
                }
            }
        }
    }
}

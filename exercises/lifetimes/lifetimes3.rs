struct Book {
    author: String,
    title: String,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: name, title: title };

    println!("{} by {}", book.title, book.author);
}
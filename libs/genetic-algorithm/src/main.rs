// трейт = интерфейс в Rust
trait Animal {
    fn kind(&self) -> &'static str;
}

// --

struct Chinchilla;

impl Animal for Chinchilla {
    fn kind(&self) -> &'static str {
        "chinchilla"
    }
}
// Этот фрагмент кода на Rust определяет метод kind, который принимает один аргумент &self, что указывает на то, что метод вызывается на экземпляре структуры или перечисления. Метод возвращает строковый литерал &'static str.
// -> &'static str - сигнатура возвращаемого типа. В данном случае, метод возвращает ссылку на строковый литерал со статическим временем жизни ('static). Это означает, что строка сохраняется во время выполнения всей программы и может быть безопасно использована в любом месте программы.
// "chinchilla" - строковый литерал, возвращаемый методом.
// Таким образом, при вызове метода kind на экземпляре, он вернет строку "chinchilla", указывая, например, на вид или тип животного в контексте программы.

fn main() {
    let chin1 = Chinchilla {};
    chin1.kind()
}

// --

struct Viscacha;

impl Animal for Viscacha {
    fn kind(&self) -> &'static str {
        "viscacha"
    }
}

// С помощью статической диспетчеризации (статический полиморфизм):
fn print_kind_static<A>(animal: &A)
where
    A: Animal,
{
    println!("{}", animal.kind());
}

// С помощью динамической диспетчеризации (динамический полиморфизм, полиморфизм времени выполнения):
fn print_kind_dynamic(animal: &dyn Animal) {
    println!("{}", animal.kind());
}
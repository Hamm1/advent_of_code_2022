
#[derive(Debug)]
struct Monkey {
    id: String,
    items: String,
    operation: String,
    test: i32,
    on_true: i32,
    on_false: i32,
    inspections: i32,
}
impl Monkey {
    fn new(id: String, items: String, operation: String, test: i32, on_true: i32, on_false: i32, inspections: i32) -> Self{
        Self { id, items, operation, test, on_true, on_false, inspections }
    }
    fn update(&mut self, items: String, inspections: i32){
        self.items = items;
        self.inspections = inspections;
    }
}

pub fn day11(path: String){
    let contents = std::fs::read_to_string(path).unwrap();
    let mut congress: Vec<Monkey> = vec![];
    let mut new_line: String = "".to_string();
    let mut starting_items: String = "".to_string();
    let mut operation: String = "".to_string();
    let mut test: i32 = 0;
    let mut on_true: i32 = 0;
    let mut on_false: i32 = 0;
    for line in contents.lines(){
        if line.contains("Monkey"){
            new_line = line.replace("Monkey ", "");
        }
        if line.contains("Starting"){
            starting_items = line.replace("Starting items: ", "").trim().to_string();
        }
        if line.contains("Operation"){
            operation = line.replace("Operation: ", "").trim().to_string();
        }
        if line.contains("Test"){
            test = line.replace("Test: divisible by ", "").trim().to_string().parse::<i32>().unwrap_or(0);
        }
        if line.contains("If true:"){
            on_true = line.replace("If true: throw to monkey ", "").trim().to_string().parse::<i32>().unwrap_or(0);
        }
        if line.contains("If false:"){
            on_false = line.replace("If false: throw to monkey ", "").trim().to_string().parse::<i32>().unwrap_or(0);
        }
        if line.is_empty(){
            let new_monkey = Monkey::new(new_line.clone(), starting_items.clone(), operation.clone(), test, on_true, on_false, 0);
            congress.push(new_monkey);
        }
        
    }
    Monkey::update(&mut congress[0], "79, 98".to_string(), 50);
    println!("{:?}",congress);

}
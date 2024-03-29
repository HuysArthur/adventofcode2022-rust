#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operator: String,
    value: String,
    diviser: i64,
    if_divisible: usize,
    if_not_divisible: usize,
    inspections_amount: i64
}

fn init_monkeys_part1(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current_monkey: Monkey = Monkey { items: Vec::new(), operator: String::from(""), value: String::from(""), diviser: 0, if_divisible: 0, if_not_divisible: 0, inspections_amount: 0 };

    for line in input.split('\n') {
        if line.starts_with("  Starting items: ") {
            current_monkey.items = line[18..].split(", ").map(|s| s.parse().unwrap()).collect();
        } else if line.starts_with("  Operation: ") {
            let (_, operation) = line.split_once("  Operation: new = old ").unwrap();
            let (operator, value_str) = operation.split_once(' ').unwrap();

            current_monkey.operator = operator.to_string();
            current_monkey.value = value_str.to_string();
        } else if line.starts_with("  Test: divisible by ") {
            let (_, rest) = line.split_once("  Test: divisible by ").unwrap();
            current_monkey.diviser = rest.parse().unwrap()
        } else if line.starts_with("    If true: throw to monkey ") {
            let (_, rest) = line.split_once("    If true: throw to monkey ").unwrap();
            current_monkey.if_divisible = rest.parse().unwrap()
        } else if line.starts_with("    If false: throw to monkey ") {
            let (_, rest) = line.split_once("    If false: throw to monkey ").unwrap();
            current_monkey.if_not_divisible = rest.parse().unwrap()
        } else if line.is_empty() {
            monkeys.push(current_monkey.clone());
        }
    }
    monkeys.push(current_monkey.clone());
    monkeys
}

pub fn result_part1(input: &str) -> i64 {
    let mut monkeys: Vec<Monkey> = init_monkeys_part1(input);

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let monkey = monkeys[monkey_index].clone();

            for item in monkey.items.clone() {
                monkeys[monkey_index].inspections_amount += 1;

                let item_index = monkeys[monkey_index].items.iter().position(|&i| i == item). unwrap();
                monkeys[monkey_index].items.remove(item_index);

                let new_worry_level: i64;
                if monkey.operator == "+" {
                    new_worry_level = (item + monkey.value.parse::<i64>().unwrap_or(item)) / 3;
                } else {
                    new_worry_level = item * monkey.value.parse::<i64>().unwrap_or(item) / 3;
                }
                if new_worry_level % monkey.diviser == 0 {
                    monkeys[monkey.if_divisible].items.push(new_worry_level);
                } else {
                    monkeys[monkey.if_not_divisible].items.push(new_worry_level);
                }
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.inspections_amount.partial_cmp(&m1.inspections_amount).unwrap());
    monkeys[0].inspections_amount * monkeys[1].inspections_amount
}

pub fn result_part2(input: &str) -> i64 {
    let mut monkeys: Vec<Monkey> = init_monkeys_part1(input);

    let product_divisers: i64 = monkeys.iter().map(|m| m.diviser).product();

    for _ in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            let monkey = monkeys[monkey_index].clone();

            for item in monkey.items.clone() {
                monkeys[monkey_index].inspections_amount += 1;

                let item_index = monkeys[monkey_index].items.iter().position(|&i| i == item). unwrap();
                monkeys[monkey_index].items.remove(item_index);

                let new_worry_level: i64;
                if monkey.operator == "+" {
                    new_worry_level = item + monkey.value.parse::<i64>().unwrap_or(item);
                } else {
                    new_worry_level = item * monkey.value.parse::<i64>().unwrap_or(item);
                }
                if new_worry_level % monkey.diviser == 0 {
                    monkeys[monkey.if_divisible].items.push(new_worry_level);
                } else {
                    monkeys[monkey.if_not_divisible].items.push(new_worry_level);
                }
            }
        }
        
        // Divide every worry level of the items of each monkey by a general unifier of the dividers of all the monkeys 
        let new_items_monkeys: Vec<Vec<i64>> = monkeys.iter().map(|m| m.items.iter().map(|i| i % product_divisers).collect()).collect();
        for (m_index, monkey_items) in new_items_monkeys.iter().enumerate() {
            monkeys[m_index].items = monkey_items.to_owned();
        }
    }

    monkeys.sort_by(|m1, m2| m2.inspections_amount.partial_cmp(&m1.inspections_amount).unwrap());
    monkeys[0].inspections_amount * monkeys[1].inspections_amount
}
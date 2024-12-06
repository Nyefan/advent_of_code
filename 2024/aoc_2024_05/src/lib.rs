use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: String) -> Result<i32> {
    let (print_rules, print_orders) = parse(input)?;
    let valid_order_sum = print_orders
        .iter()
        .filter(|order| order.is_valid_order(&print_rules))
        .map(|order| order[order.len() / 2])
        .sum();
    Ok(valid_order_sum)
}

pub fn part_2(input: String) -> Result<i32> {
    let (print_rules, print_orders) = parse(input)?;
    let valid_order_sum = print_orders
        .iter()
        .filter(|order| !order.is_valid_order(&print_rules))
        .map(|order| fixup(order, &print_rules))
        .map(|order| order[order.len() / 2])
        .sum();
    Ok(valid_order_sum)
}

fn parse(input: String) -> Result<(PrintRuleset, Vec<PrintOrder>)> {
    let binding = input.split_once("\n\n").ok_or("Error parsing input")?;
    let print_rules: PrintRuleset = binding
        .0
        .lines()
        .map(|line| -> Result<PrintRule> {
            Ok(line.split_once('|').ok_or("Error parsing input")?.into())
        })
        .collect::<Result<Vec<PrintRule>>>()?
        .into();
    let print_orders = binding
        .1
        .lines()
        .map(|line| -> Result<PrintOrder> {
            line.split(",")
                .map(|s| -> Result<i32> { Ok(s.parse::<i32>()?) })
                .collect::<Result<PrintOrder>>()
        })
        .collect::<Result<Vec<PrintOrder>>>()?;
    Ok((print_rules, print_orders))
}

fn fixup(print_order: &PrintOrder, print_ruleset: &PrintRuleset) -> PrintOrder {
    let mut transform: Vec<PageNumberWithRuleset> = print_order
        .iter()
        .map(|page_number| PageNumberWithRuleset {
            page_number: *page_number,
            print_ruleset,
        })
        .collect();
    transform.sort_by(|a, b| a.partial_cmp(b).unwrap());
    transform
        .iter()
        .map(|page_number_with_ruleset| page_number_with_ruleset.page_number)
        .collect()
}

impl PartialEq<Self> for PageNumberWithRuleset<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.page_number == other.page_number
    }
}

impl PartialOrd for PageNumberWithRuleset<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self
            .print_ruleset
            .must_be_before
            .get(&self.page_number)
            .is_some_and(|v| v.contains(&other.page_number))
        {
            return Some(Ordering::Less);
        }
        if self
            .print_ruleset
            .must_be_after
            .get(&self.page_number)
            .is_some_and(|v| v.contains(&other.page_number))
        {
            return Some(Ordering::Greater);
        }
        self.page_number.partial_cmp(&other.page_number)
    }
}

struct PageNumberWithRuleset<'a> {
    page_number: i32,
    print_ruleset: &'a PrintRuleset,
}

struct PrintRule {
    first: i32,
    last: i32,
}

#[derive(Debug)]
struct PrintRuleset {
    must_be_before: HashMap<i32, HashSet<i32>>,
    must_be_after: HashMap<i32, HashSet<i32>>,
}

impl From<Vec<PrintRule>> for PrintRuleset {
    fn from(input: Vec<PrintRule>) -> Self {
        let mut must_be_before = HashMap::new();
        let mut must_be_after = HashMap::new();
        for rule in input {
            must_be_before
                .entry(rule.first)
                .or_insert_with(HashSet::new)
                .insert(rule.last);
            must_be_after
                .entry(rule.last)
                .or_insert_with(HashSet::new)
                .insert(rule.first);
        }
        Self {
            must_be_before,
            must_be_after,
        }
    }
}

impl From<(&str, &str)> for PrintRule {
    fn from(input: (&str, &str)) -> Self {
        Self {
            first: input.0.parse().unwrap(),
            last: input.1.parse().unwrap(),
        }
    }
}

type PrintOrder = Vec<i32>;

trait IsValidOrder {
    fn is_valid_order(&self, print_ruleset: &PrintRuleset) -> bool;
}

impl IsValidOrder for PrintOrder {
    fn is_valid_order(&self, print_rules: &PrintRuleset) -> bool {
        let mut forbidden_pages: HashSet<i32> = HashSet::new();
        for page in self {
            if forbidden_pages.contains(page) {
                return false;
            }
            print_rules
                .must_be_after
                .get(page)
                .iter()
                .flat_map(|o| o.iter())
                .for_each(|&p| {
                    forbidden_pages.insert(p);
                });
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        let result_sample = part_1(std::fs::read_to_string("data/sample")?)?;
        assert_eq!(result_sample, 143);
        let result_actual = part_1(std::fs::read_to_string("data/actual")?)?;
        assert_eq!(result_actual, 7024);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let result_sample = part_2(std::fs::read_to_string("data/sample")?)?;
        assert_eq!(result_sample, 123);
        let result_actual = part_2(std::fs::read_to_string("data/actual")?)?;
        assert_eq!(result_actual, 4151);
        Ok(())
    }
}

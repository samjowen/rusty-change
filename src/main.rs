use currencies::CurrencyKind;
use utils::calculate_change;

mod currencies;
mod tests;
mod utils;
use crate::currencies::Money;

fn main() {
    let one_pound = Money {
        value: 100,
        currency: CurrencyKind::Gbp,
    };

    println!(
        "{:?}",
        calculate_change(
            &[&one_pound, &one_pound, &one_pound, &one_pound, &one_pound, &one_pound,],
            532,
        )
    );
}

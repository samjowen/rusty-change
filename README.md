# Rusty Change 💰

Rusty Change is a Rust crate that provides a function to calculate the optimal change for different currencies, taking into account the different denominations of tender.

## Table of Contents
- [Usage](#usage)
- [Supported Currencies](#supported-currencies)
- [Supported Tender](#supported-tender)
- [Contributions](#contributions)

## Usage

```rust
use rusty_change::{calculate_change, CurrencyKind, Money};

let paid = vec![&Money { value: 1000, currency: CurrencyKind::Gbp }];
let price = 375;
let change = calculate_change(&paid, price);

assert_eq!(
    change,
    vec![
        &Money { value: 200, currency: CurrencyKind::Gbp },
        &Money { value: 200, currency: CurrencyKind::Gbp },
        &Money { value: 200, currency: CurrencyKind::Gbp },
        &Money { value: 50, currency: CurrencyKind::Gbp },
        &Money { value: 20, currency: CurrencyKind::Gbp },
        &Money { value: 5, currency: CurrencyKind::Gbp },
    ]
);

## Supported Currencies

Rusty Change currently supports the following currencies:

- British Pound (GBP)
- Euro (EUR)
- United States Dollar (USD)

## Supported Tender

For each supported currency, Rusty Change provides the tender denominations in the form of a static array of Money structs. These tenders can be accessed using the get_tender() method of the CurrencyKind enum.

Here's an example of how to access the British Pound Sterling tender:

```rust
use rusty_change::{CurrencyKind, BRITISH_STERLING_TENDER};

let tender = CurrencyKind::Gbp.get_tender();

assert_eq!(tender, &BRITISH_STERLING_TENDER);
```

## Contributions

Contributions are welcome! Please feel free to open an issue or a pull request if you have any suggestions or improvements.
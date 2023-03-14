#[derive(PartialEq, Debug)]
pub struct Money {
    // This the smallest unit of currency (e.g. pence, cents, eurocents). Using floating point numbers here would be a bad idea,
    // because of the possibility of rounding errors. If you want to use floating point numbers, you should use the Decimal crate.
    pub currency: CurrencyKind,
    pub value: i64,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CurrencyKind {
    Gbp,
    Usd,
    Eur,
}

impl CurrencyKind {
    pub fn get_tender(&self) -> &'static [Money] {
        match self {
            CurrencyKind::Gbp => &BRITISH_STERLING_TENDER,
            CurrencyKind::Usd => &US_DOLLAR_TENDER,
            CurrencyKind::Eur => &EURO_TENDER,
        }
    }
}

pub const BRITISH_STERLING_TENDER: [Money; 12] = [
    Money {
        value: 1,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 2,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 5,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 10,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 20,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 50,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 100,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 200,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 500,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 1000,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 2000,
        currency: CurrencyKind::Gbp,
    },
    Money {
        value: 5000,
        currency: CurrencyKind::Gbp,
    },
];

pub const US_DOLLAR_TENDER: [Money; 10] = [
    Money {
        value: 1,
        currency: CurrencyKind::Usd,
    },
    Money {
        value: 5,
        currency: CurrencyKind::Usd,
    },
    Money {
        value: 10,
        currency: CurrencyKind::Usd,
    },
    Money {
        value: 25,
        currency: CurrencyKind::Usd,
    },
    Money {
        value: 50,
        currency: CurrencyKind::Usd,
    },
    Money {
        value: 100,
        currency: CurrencyKind::Usd,
    },
    Money {
        value: 500,
        currency: CurrencyKind::Usd,
    },
    Money {
        value: 1000,
        currency: CurrencyKind::Usd,
    },
    Money {
        value: 5000,
        currency: CurrencyKind::Usd,
    },
    Money {
        value: 10000,
        currency: CurrencyKind::Usd,
    },
];

pub const EURO_TENDER: [Money; 15] = [
    Money {
        value: 1,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 2,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 5,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 10,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 20,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 50,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 100,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 200,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 500,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 1000,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 2000,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 5000,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 10000,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 20000,
        currency: CurrencyKind::Eur,
    },
    Money {
        value: 50000,
        currency: CurrencyKind::Eur,
    },
];

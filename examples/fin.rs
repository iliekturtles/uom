//! Applying units of measure to financial quantities is reasonable.
//! Financial applications are high risk applications, subject to formal verification and audits.
//! With many quantities (time, price, contract sizes, amount as expressed in quote, quantity per time)
//! In many cases, f64 cannot be used as base type, because of rounding errors.
//!
//! Here we do example of fixed scale financial quantities using u128 integer.
//!
//! Scale may be data driven, and `uom` encoded as `decimals` quantity per symbol(e.g. EUR amount, USD amount, EUR price, etc).
//! In this case we would  divide "mantissas" by "decimals" to get the final "user" value.
//!
//! Sometimes finances need several bases in single system to be used, which `uom` does not support.
#[cfg(feature = "u128")]
#[macro_use]
extern crate uom;

#[cfg(not(feature = "u128"))]
fn main() {}

#[cfg(feature = "u128")]
fn main() {
    use crate::fin::*;
    let milliu = Amount::format_args(milliunit, Abbreviation);

    let dq = Quote::format_args(quote::dollar, Abbreviation);

    let cp = Price::format_args(price::cent, Abbreviation);

    let per_hour = Velocity::format_args(velocity::unit_per_hour, Abbreviation);

    let two_euro = Amount::new::<unit>(2);
    println!("euro_amount = {:?}", milliu.with(two_euro));

    let euro_price = Price::new::<price::dollar>(2);
    println!("euro_price = {:?}", cp.with(euro_price));

    let euro_quote = euro_price * two_euro;
    println!("euro_quote = {:?}", dq.with(euro_quote));

    let contract_duration = Time::new::<day>(1);

    let euro_million = Amount::new::<megaunit>(30);
    let per_day_settlement = euro_million / contract_duration;
    println!("per_day_settlement = {:?}", per_hour.with(per_day_settlement));

    let dollar_price = Price::new::<price::cent>(99);
    println!("dollar_price ={:?}", cp.with(dollar_price));

    let eur_to_dollar = euro_price / dollar_price;
    println!("eur_to_dollar ={:?}", eur_to_dollar);
}

#[cfg(feature = "u128")]
mod fin {
    pub use crate::fin::amount::{megaunit, milliunit, unit};
    pub use crate::fin::time::day;
    pub use crate::fin::u128::*;
    pub use uom::fmt::DisplayStyle::Abbreviation;

    #[macro_use]
    pub mod price {
        quantity! {
            quantity: Price; "price";
            dimension: FIN<
                P1,
                Z0,
                Z0>;
            units {
                    @dust: 1.; "dust", "dust", "dust";
                    @cent: 10_000_000.; "cent", "cent", "cents";
                    @dollar: 1_000_000_000.; "USD", "dollar", "dollars";
                    @grand: 1_000_000_000_000.; "grand", "grand", "grands";
            }
        }
    }

    #[macro_use]
    pub mod time {
        quantity! {
            quantity: Time; "time";
            dimension: FIN<
                Z0,
                P1,
                Z0>;
            units {
                @millisecond: 1.; "millis", "millisecond", "milliseconds";
                @second: 1_000.; "s", "second", "seconds";
                @minute: 60_000.; "min", "minute", "minutes";
                @hour: 3_600_000.; "h", "hour", "hours";
                @day: 86_400_000.; "d", "day", "days";
                @week: 604_800_000.; "w", "week", "weeks";
                @month: 2_592_000_000.; "mo", "month", "months";
            }
        }
    }

    #[macro_use]
    pub mod amount {
        quantity! {
            quantity: Amount; "amount";
            dimension: FIN<
                Z0,
                Z0,
                P1>;
            units {
                @microunit: 1.; "µu", "microunit", "microunits";
                @milliunit: 1_000.; "milliu", "milliunit", "milliunit";
                @unit: 1_000_000.; "u", "unit", "unit";
                @kilounit: 1_000_000_000.; "K", "kilounit", "kilounits";
                @megaunit: 1_000_000_000_000.; "M", "megaunit", "megunits";
            }
        }
    }

    #[macro_use]
    pub mod velocity {
        quantity! {
            quantity: Velocity; "velocity";
            dimension: FIN<
                Z0,
                N1,
                P1>;
            units {
                @unit_per_millisecond: 1_000_000. / (1.); "unit/millis", "unit per millisecond", "units per millisecond";
                @unit_per_second: 1_000_000. / (1_000.0); "unit/s", "unit per second", "units per second";
                @unit_per_minute: 1_000_000. / (60_000.0); "unit/min", "unit per minute", "units per minute";
                @unit_per_hour: 1_000_000. / (60.0 * 60_000.0);  "unit/h", "unit per hour", "units per hour";
                @unit_per_day: 1_000_000. / (60.0 * 60_000.0 * 24.);  "unit/d", "unit per day", "units per day";

            }
        }
    }

    #[macro_use]
    pub mod quote {
        quantity! {
            quantity: Quote; "quote";
            dimension: FIN<
                P1,
                Z0,
                P1>;
            units {
                @dollar: 1_000_000_000.0 * 1_000_000.0 ; "$", "dollar", "dollars";
            }
        }
    }

    system! {
        quantities: FIN {
            price: dust, PM;
            time: millisecond, T;
            amount: microunit, A;
        }

        units: U {
            mod price::Price,
            mod time::Time,
            mod amount::Amount,
            mod velocity::Velocity,
            mod quote::Quote,
        }
    }

    mod u128 {

        mod fin {
            pub use super::super::*;
        }

        FIN!(self::fin, u128);
    }
}

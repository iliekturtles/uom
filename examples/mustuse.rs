
extern crate uom;

use uom::Conversion;
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si ::Unit;
use uom::si ::length      ::{meter , millimeter , micrometer };
use uom::si ::time        ::{second, millisecond, microsecond};
use uom::si ::velocity    ::{self, meter_per_second};
use uom::si ::acceleration::meter_per_second_squared;
use uom::si ::area        ::square_meter;
use uom::si ::frequency   ::hertz;

// use uom::si ::mass        ::gram;
// use uom::si ::momentum    ::gram_meter_per_second;
use uom::si ::angle                    ::radian;
use uom::si ::ratio                    ::ratio;
// use uom::si ::thermodynamic_temperature::degree_celsius;
// use uom::si ::temperature_interval     ::degree_fahrenheit;

use uom::si ::f64::{ Length
                   , Time
                   , Area
                   , Velocity
                   // , Mass
                   , Angle
                   // , Ratio
                   // , ThermodynamicTemperature
                   // , TemperatureInterval
                   };

use typenum::{P2, P3};

macro_rules! check_n_print {
    ($id:literal, $value:expr) => {
        $value;
        println!("{} => {:?}", $id, $value);
    };
}

macro_rules! check_n_print_unit {
    ($id:literal, $value:expr, $unit:expr) => {
        $value;
        println!("{} => {:>10.4?} => {:>10.4?}", $id, $value, $value.into_format_args($unit, Abbreviation));
    };
}

fn sep(tag: &str) {
    let line = "=".repeat(25);
    println!("");
    println!("{} {} {}", line, tag, line);
    println!("");
}

fn main() {
    sep("new values");

    check_n_print_unit!("new length", Length::new::<meter> (1.0), millimeter );
    check_n_print_unit!("new time  ", Time  ::new::<second>(2.0), microsecond);

    sep("Accessing/Transforming values");

    let l1  = Length::new::<micrometer  >(3.0);
    let l2  = Area  ::new::<square_meter>(4.0);
    let t1  = Time  ::new::<millisecond >(5.0);
    let t2  = t1.powi(P2::new());
    let t3  = t1.powi(P3::new());
    let v1  = l1 / t1;
    // let m1  = Mass  ::new::<gram       >(5.0);
    let fmt = Velocity::format_args(meter_per_second, Abbreviation);

    check_n_print!("get       ", l1.get::<millimeter>());
    check_n_print!("conversion", l1.get::<millimeter>().conversion());

    sep("Formatting");

    check_n_print!("with            ", fmt.with(v1));
    check_n_print!("into_format_args", v1.into_format_args(meter_per_second, Abbreviation));

    sep("Values checks");

    // Arithmetic operations already throw a warning
    // check_n_print_unit!("+", t1 + t1, microsecond);
    // check_n_print_unit!("+", t1 - t1, microsecond);
    // check_n_print_unit!("+", m1 * l1 / t1, gram_meter_per_second);
    // check_n_print_unit!("+", l1 / t2, meter_per_second_squared);

    check_n_print!("is_nan          ", t1.is_nan          ());
    check_n_print!("is_finite       ", l1.is_finite       ());
    check_n_print!("is_infinite     ", t1.is_infinite     ());
    check_n_print!("is_normal       ", l1.is_normal       ());
    check_n_print!("classify        ", t1.classify        ());
    check_n_print!("is_sign_negative", l1.is_sign_negative());
    check_n_print!("is_sign_positive", t1.is_sign_positive());

    sep("Operations on any value");

    check_n_print_unit!("signum ", l2.signum              (         ), square_meter);
    check_n_print_unit!("hypot  ", t1.hypot               (t1 + t1  ), microsecond );
    check_n_print_unit!("powi   ", l1.powi                (P2::new()), square_meter);
    check_n_print_unit!("mul_add", l1.mul_add             (l1, l2   ), square_meter);
    check_n_print_unit!("recip  ", t1.recip               (         ), hertz       );
    check_n_print_unit!("max    ", l1.max                 (l1       ), millimeter  );
    check_n_print_unit!("min    ", t1.min                 (t1       ), microsecond );
    check_n_print_unit!("floor  ", t1.floor::<microsecond>(         ), microsecond );
    check_n_print_unit!("ceil   ", l1.ceil ::<millimeter >(         ), millimeter  );
    check_n_print_unit!("round  ", t1.round::<microsecond>(         ), microsecond );
    check_n_print_unit!("trunc  ", l1.trunc::<millimeter >(         ), millimeter  );
    check_n_print_unit!("fract  ", t1.fract::<microsecond>(         ), microsecond );
    check_n_print_unit!("abs    ", l1.abs                 (         ), millimeter  );
    check_n_print_unit!("sqrt   ", t2.sqrt                (         ), microsecond );
    check_n_print_unit!("cbrt   ", t3.cbrt                (         ), microsecond );

    sep("String representations");

    check_n_print!("description ", velocity                ::description ());
    check_n_print!("abbreviation", meter_per_second_squared::abbreviation());
    check_n_print!("singular    ", second                  ::singular    ());
    check_n_print!("plural      ", meter                   ::plural      ());

    sep("Angle-specific functions");

    let a = Angle::new::<radian>(6.0);

    check_n_print_unit!("sin   ", a.sin    ( ), ratio );
    check_n_print_unit!("sinh  ", a.sinh   ( ), ratio );
    check_n_print_unit!("cos   ", a.cos    ( ), ratio );
    check_n_print_unit!("cosh  ", a.cosh   ( ), ratio );
    check_n_print_unit!("tan   ", a.tan    ( ), ratio );
    check_n_print_unit!("tanh  ", a.tanh   ( ), ratio );
    check_n_print_unit!("atan2 ", a.atan2  (a), radian);
    check_n_print!     ("sincos", a.sin_cos( ));

    sep("Ratio-specific functions");

    // Ratio has the must_use attribute already
    // let r = Ratio::new::<ratio>(0.5);
    //
    // check_n_print_unit!("sin   ", r.asin  (  ), radian );
    // check_n_print_unit!("sinh  ", r.asinh (  ), radian );
    // check_n_print_unit!("cos   ", r.acos  (  ), radian );
    // check_n_print_unit!("cosh  ", r.acosh (  ), radian );
    // check_n_print_unit!("tan   ", r.atan  (  ), radian );
    // check_n_print_unit!("tanh  ", r.atanh (  ), radian );
    // check_n_print_unit!("exp   ", r.exp   (  ), ratio  );
    // check_n_print_unit!("exp2  ", r.exp2  (  ), ratio  );
    // check_n_print_unit!("ln    ", r.ln    (  ), ratio  );
    // check_n_print_unit!("log   ", r.log   (2.), ratio  );
    // check_n_print_unit!("log2  ", r.log2  (  ), ratio  );
    // check_n_print_unit!("log10 ", r.log10 (  ), ratio  );
    // check_n_print_unit!("exp_m1", r.exp_m1(  ), ratio  );
    // check_n_print_unit!("ln_1p ", r.ln_1p (  ), ratio  );

}

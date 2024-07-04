#[allow(warnings, dead_code)]
pub mod length {
    pub fn meter_to_feet(meter: f32) -> f32 {
        meter * 3.28084
    }

    pub fn feet_to_meter(feet: f32) -> f32 {
        feet / 3.28084
    }
}

/*-----------------Funciones para peso------------------*/
#[allow(warnings)]
pub mod weight {
    pub fn kg_to_pound(kg: f32) -> f32 {
        kg * 2.20462
    }

    pub fn pound_to_kg(pound: f32) -> f32 {
        pound / 2.20462
    }
}

/*-----------------Funciones para temperatura------------------*/
#[allow(warnings)]
pub mod temp {
    pub fn c_to_f(c: f32) -> f32 {
        c * 1.8 + 32.0
    }

    pub fn f_to_c(f: f32) -> f32 {
        (f - 32.0) / 1.8
    }

    pub fn c_to_k(c: f32) -> f32 {
        c + 273.15
    }

    pub fn k_to_c(k: f32) -> f32 {
        k - 273.15
    }
}

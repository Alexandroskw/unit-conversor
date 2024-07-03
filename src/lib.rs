pub mod utils;
/*-----------------Funciones para hacer las conversiones------------------*/

/*-----------------Funciones para longitud------------------*/
pub mod length {
    pub fn meter_to_feet(meter: f32) -> f32 {
        meter * 3.28084
    }

    pub fn feet_to_meter(feet: f32) -> f32 {
        feet / 3.28084
    }
}

/*-----------------Funciones para peso------------------*/
pub mod weight {
    pub fn kg_to_pound(kg: f32) -> f32 {
        kg * 2.20462
    }

    pub fn pound_to_kg(pound: f32) -> f32 {
        pound / 2.20462
    }
}

/*-----------------Funciones para temperatura------------------*/
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

/*-----------------Funciones para calcular los submúltiplos------------------*/
pub mod tiempo {
    pub fn seg_to_min(s: f32) -> f32 {
        s / 60.0
    }

    pub fn seg_to_hr(s: f32) -> f32 {
        s / 3600.0
    }

    pub fn seg_to_ms(s: f32) -> f32 {
        s * 1000.0
    }

    pub fn seg_to_us(s: f32) -> f32 {
        s * 1_000_000.0
    }
}

pub mod longitud {
    const FACTOR: f32 = 1_000.0;

    pub fn m_to_km(m: f32) -> f32 {
        m / FACTOR
    }

    pub fn m_to_cm(m: f32) -> f32 {
        (m * FACTOR) / 10.0
    }

    pub fn m_to_mm(m: f32) -> f32 {
        m * FACTOR
    }

    pub fn m_to_dm(m: f32) -> f32 {
        (m * FACTOR) / 100.0
    }

    pub fn m_to_um(m: f32) -> f32 {
        m * 1_000_000.0
    }

    pub fn m_to_nm(m: f32) -> f32 {
        m * 1000_000_000.0
    }
}

pub mod mass {
    const FACTOR: f32 = 1_000.0;

    pub fn g_to_kg(g: f32) -> f32 {
        g / FACTOR
    }

    pub fn g_to_cg(g: f32) -> f32 {
        g * 100.0
    }

    pub fn g_to_mg(g: f32) -> f32 {
        g * 1_000.0
    }

    pub fn g_to_dg(g: f32) -> f32 {
        g * 10.0
    }

    pub fn g_to_ug(g: f32) -> f32 {
        g * 1_000_000.0
    }

    pub fn g_to_ng(g: f32) -> f32 {
        g * 1000_000_000.0
    }
}

pub mod ampere {
    pub fn a_to_ma(a: f32) -> f32{
        a * 1_000.0
    }

    pub fn a_to_ua(a: f32) -> f32 {
        a * 1_000_000.0
    }

    pub fn a_to_na(a: f32) -> f32 {
        a * 1_000_000_000.0
    }
} 

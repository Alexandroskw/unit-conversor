// #[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

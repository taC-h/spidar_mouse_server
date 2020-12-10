use std::os::raw::{c_int, c_float};

#[link(name="SpidarMouse")]
extern "C" {
    fn OpenSpidarMouse() -> c_int;
    fn SetForce(Force_XScale: c_float, Force_YScale: c_float, duration: c_int);
    fn CloseSpidarMouse() -> c_int;
    fn SetMinForceDuty(MinForceDuty: c_float);
    fn SetDutyOnCh(
        duty1: c_float,
        duty2: c_float,
        duty3: c_float,
        duty4: c_float,
        duration: c_int
    );
}

pub fn open_spidar_mouse() -> i32 {
    unsafe{ OpenSpidarMouse() }
}

pub fn set_force(force_x_scale: f32, froce_y_scale: f32, duration: i32) {
    unsafe{ SetForce(force_x_scale, froce_y_scale, duration); }
}

pub fn close_spidar_mouse() -> bool {
    unsafe{
        match CloseSpidarMouse(){
            0 => false,
            _ => true
        }
    }
}

pub fn set_min_force_duty(min_force_duty: f32) {
    unsafe{ SetMinForceDuty(min_force_duty); }
}

pub fn set_duty_on_ch(
    duty1: f32,
    duty2: f32,
    duty3: f32,
    duty4: f32,
    duration: i32
){
    unsafe{
        SetDutyOnCh(
            duty1,
            duty2,
            duty3,
            duty4,
            duration
        );
    }
}
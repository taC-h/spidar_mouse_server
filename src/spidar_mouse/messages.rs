use serde::{Serialize, Deserialize};
use super::wrapper::*;


#[derive(Serialize)]
pub struct ResponseBody {
    pub result: bool,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub enum RequestBody {
    OpenSpidarMouse,
    CloseSpidarMouse,
    SetForce{
        Force_X: f32,
        Force_Y: f32,
        duration: i32,
    },
    SetMinForceDuty{
        MinForceDuty: f32,
    },
    SetDutyOnCh{
        duty1: f32,
        duty2: f32,
        duty3: f32,
        duty4: f32,
        duration: i32,
    },
}

impl RequestBody{
    #[allow(non_snake_case)]
    pub fn run(self) -> (bool,String){
        match self {
            RequestBody::OpenSpidarMouse => {
                let ret = open_spidar_mouse();
                let result = if ret == 1 { true } else { false };
                let message = match ret {
                    1 => "SpidarMouseの接続に成功しました",
                    -1 => "SPIDAR-mouse を検出できませんでした",
                    _ => "メモリ領域の確保に失敗しました",
                }.to_string();
                (result, message)
            },
            RequestBody::CloseSpidarMouse => {
                let ret = close_spidar_mouse();
                if ret {
                    (true, format!("SpidarMouseを終了しました"))
                } else {
                    (false, format!("SpidarMouseの終了に失敗しました"))
                }
            },
            RequestBody::SetForce{Force_X, Force_Y, duration} => {
                set_force(Force_X, Force_Y , duration);
                (true, format!("func:SetForce\nrecived:{:.2},{:.2},{}", Force_X, Force_Y, duration))
            },
            RequestBody::SetMinForceDuty{MinForceDuty} => {
                set_min_force_duty(MinForceDuty);
                (true, format!("func:SetMinForceDuty\nrecived:{}", MinForceDuty))
            }
            RequestBody::SetDutyOnCh{ duty1, duty2, duty3, duty4, duration } => {
                set_duty_on_ch(duty1, duty2, duty3, duty4, duration);
                (true, format!("func:SetDutyOnCh\nrecived:{:.2},{:.2},{:.2},{:.2},{}", duty1, duty2, duty3, duty4, duration))
            }
        }
    }
}
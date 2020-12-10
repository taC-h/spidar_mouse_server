class Request  {
    constructor(){
        this.openSpidarMouse = "OpenSpidarMouse";
        this.closeSpidarMouse = "CloseSpidarMouse";
        this.setForce = {"SetForce":{
            Force_x: 0,
            Force_y: 0,
            duration: 0,

        }};
        this.setMinForceDuty = {"SetMinForceDuty":{
            MinForceDuty:0.2
        }};
        this.setDutyOnCh = {"SetDutyOnCh":{
            duty1: 0,
            duty2: 0,
            duty3: 0,
            duty4: 0,
            duration: 0,
        }};
    }
};

{
    let socket = new WebSocket(`ws://${ document.location.host }/ws/`);
    socket.loger = document.getElementById("logger");

    socket.addEventListener("open",(e)=>{
    socket.loger.textContent = "ws:conetced";
    });

    socket.addEventListener("message",(e)=>{
    socket.loger.textContent = e.data;
    });

    socket.addEventListener("close",(e)=>{
    socket.loger.textContent = "ws:closed";
    });

    socket.addEventListener("error",(e)=>{
    socket.loger.textContent = "ws:closed by error" + new String(e);
    });


    let root = document.getElementById("area");
    root.innerHTML ="";
    let op = document.createElement("button");
    op.textContent = "OpenSpidarMouse";
    op.onclick = ()=>{ socket.send(JSON.stringify("OpenSpidarMouse")) };
    root.appendChild(op)

    let paramsdata = {
        "duty1": 0,
        "duty2": 0,
        "duty3": 0,
        "duty4": 0,
        "duration": 0,
    };
    let params = document.createElement("div");
    params.id = "params";
    [1,2,3,4].map((value)=>{
        let elm = document.createElement("input");
        elm.id = "duty"+new String(value);
        elm.type = "range";
        elm.value = "0";
        elm.min = "0";
        elm.max = "1";
        elm.step = "0.01"
        elm.textContent = "0"
        elm.oninput=(e)=>{
            let num = parseFloat(elm.value);
            elm.textContent = num;
            paramsdata[elm.id] = num;
            let tmp = JSON.stringify({SetDutyOnCh:paramsdata});
            socket.send(tmp);
        };
        params.appendChild(elm);
    });
    root.appendChild(params);
    let cl = document.createElement("button");
    cl.textContent = "CloseSpidarMouse";
    cl.onclick = ()=>{ socket.send(JSON.stringify("CloseSpidarMouse")) };
    root.appendChild(cl);

}
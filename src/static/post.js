function req(){
    let xhr = new XMLHttpRequest();
    xhr.open("POST","/api/");
    xhr.send("OpenSpidarMouse");
    xhr.onreadystatechange = () => {
        if ((xhr.readyState == 4) && (xhr.status == 200)) {
            document.getElementById("loger").innerHTML = xhr.responseText;
        }
    }
}
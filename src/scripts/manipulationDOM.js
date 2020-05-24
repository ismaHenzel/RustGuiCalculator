document.addEventListener("keypress", event=>{
    external.invoke(event.key)
})
function getData(value){
    document.getElementById("display").innerHTML = value
}
function last_operation(value){
    document.getElementById("last_op").innerHTML = value
}
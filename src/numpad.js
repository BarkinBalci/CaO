function display(val) {
    document.getElementById("calculate-input").value += val
  }

function clr(){
    document.getElementById("calculate-input").value = ""
}

function del(){
  var a = document.getElementById("calculate-input"),
  b = a.value;
  a.value = b.substring(0, b.length - 1);
}

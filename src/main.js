const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
let values;

let span=15;
let a=5;
let b=10;
let pointloads=[];
let pointmoments=[];
let uniformloads=[];
let linearloads=[];


function makebeam(){
  let spanin = document.getElementById("spanin");
  let ain = document.getElementById("ain");
  let bin = document.getElementById("bin");
  span = parseFloat(spanin.value);
  a = parseFloat(ain.value);
  b = parseFloat(bin.value);
}

function addpointload(){
  let xin = parseFloat(document.getElementById("plxin").value);
  let fxin = parseFloat(document.getElementById("plfxin").value);
  let fyin = parseFloat(document.getElementById("plfyin").value);
  pointloads.push([xin,fxin,fyin])
}

function addpointmoment(){
  let xin = parseFloat(document.getElementById("pmxin").value);
  let magin = parseFloat(document.getElementById("pmmin").value);
  pointmoments.push([xin,magin])
}

function addpuniformload(){
  let x1in = parseFloat(document.getElementById("ulx1in").value);
  let x2in = parseFloat(document.getElementById("ulx2in").value);
  let fyin = parseFloat(document.getElementById("ulfyin").value);
  uniformloads.push([x1in,x2in,fyin])
}

function addlinearload(){
  let x1in = parseFloat(document.getElementById("llx1in").value);
  let x2in = parseFloat(document.getElementById("llx2in").value);
  let fy1in = parseFloat(document.getElementById("llfy1in").value);
  let fy2in = parseFloat(document.getElementById("llfy2in").value);
  linearloads.push([x1in,x2in,fy1in,fy2in])
}

function reset(){
  span=15;
  a=0;
  b=10;
  pointloads=[];
  pointmoments=[];
  uniformloads=[];
  linearloads=[];
  Plotly.purge("plot");
  const inputElements = document.querySelectorAll('input[type="text"], input[type="email"], input[type="number"], input[type="search"], input[type="tel"], input[type="url"]');
  inputElements.forEach(input => {input.value = '';});
}

async function execute() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  // greetMsgEl.textContent = await invoke("execute", {span: 17.0,
  //                                                   a: 3.0,
  //                                                   b: 13.0,
  //                                                   pointloads:[[6,0,-90]],
  //                                                   });
  if (pointloads.length==0){
    pointloads.push([])
  }
  if (pointmoments.length==0){
    pointmoments.push([])
  }
  if (uniformloads.length==0){
    uniformloads.push([])
  }
  if (linearloads.length==0){
    linearloads.push([])
  }
  values = await invoke("execute", {span: span,
          a: a,
          b: b,
          pointloads:[...pointloads], // [location, xvalue, yvalue] <- LOWKEY DONT USE X WHY IS IT HERE LOL! Test: [6,0,-90]
          pointmoments:[...pointmoments], // [location, value] Test: [17,50]
          uniformloads:[...uniformloads], // [start, end, value] Test: [8,17,-10]
          linearloads:[...linearloads], // [start, end, startval, endval] Test: [8,17,-10,0]
          });
  
  
  const trace = {
    x: values[0],
    y: values[1],
    type: 'scatter',
    mode: 'lines',
    marker: { color: 'green' },
    fill: "tonexty",
    fillcolor: "rgba(0,255,0,.1"
  };

  const trace2 = {
    x: values[0],
    y: values[2],
    type: 'scatter',
    mode: 'lines',
    marker: { color: 'red' },
    fill: "tonexty",
    fillcolor: "rgba(255,0,0,.1",
    xaxis: "x2",
    yaxis: "y2",
  };


  
  const layout = {
    title: {text:'Shear Force and Bending Moment Diagram', y: .85, x: .5, xanchor: "center",yanchor: "top"},
    xaxis2: { title: 'Distance (m)' },
    yaxis: { title: 'Shear Force (N)' },
    yaxis2: { title: 'Bending Moment (Nm)' },
    grid : {rows: 2, columns:1, pattern: "independent"},
    showlegend: false,
    height: 600,
  };
  
  Plotly.newPlot('plot', [trace,trace2], layout);


}

document.getElementById("exec").addEventListener("click",execute);
document.getElementById("makebeam").addEventListener("click",makebeam);
document.getElementById("rst").addEventListener("click",reset);

document.getElementById("addpl").addEventListener("click",addpointload);
document.getElementById("addpm").addEventListener("click",addpointmoment);
document.getElementById("addul").addEventListener("click",addpuniformload);
document.getElementById("addll").addEventListener("click",addlinearload);

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    execute();
  });
});


use ndarray::{array, Array1, Array2, ArrayView1, ArrayView, Axis};



// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn execute(span: f64, a: f64, b: f64, pointloads: Vec<Vec<f64>>, pointmoments: Vec<Vec<f64>>, uniformloads: Vec<Vec<f64>>, linearloads: Vec<Vec<f64>>) -> (Vec<f64>,Vec<f64>,Vec<f64>) {
    // format!("Hello, {:?}! You've been greeted from Rust!", pointloads)
    
    // Defining parameters
    let divs: f64 = 10000.0;
    let delta: f64 = span/divs;
    let x_axis: Array1<f64> = Array1::range(0.0, span+delta, delta);
    let check_point_loads: usize = pointloads[0].len();
    let check_point_moments: usize = pointmoments[0].len();
    let check_uniform_loads: usize = uniformloads[0].len();
    let check_linear_loads: usize = linearloads[0].len();

    let mut reactions: Array1<f64> = array![0.0, 0.0, 0.0];
    let mut shear_froce: Array2<f64> =  Array2::zeros((0, x_axis.len()));
    let mut bending_moment: Array2<f64> =  Array2::zeros((0, x_axis.len()));


    // Calculating Reactions from point loads
    let mut point_loads_records: Array2<f64> = Array2::zeros((0,3));
    if check_point_loads > 0 {
        for (_i, p) in pointloads.iter().enumerate(){
            let (va, vb, ha) = point_load_reactions(p, a, b);
            point_loads_records.push_row(ArrayView::from(&[va,ha,vb])).unwrap();
            reactions[0] = reactions[0] + va;
            reactions[1] = reactions[1] + ha;
            reactions[2] = reactions[2] + vb;
        }
    }

    // Calculating Reactions from point moments
    let mut point_moments_records: Array2<f64> = Array2::zeros((0,2));
    if check_point_moments > 0 {
        for (_i, m) in pointmoments.iter().enumerate(){
            let (va, vb) = point_moment_reactions(m, a, b);
            point_moments_records.push_row(ArrayView::from(&[va,vb])).unwrap();
            reactions[0] = reactions[0] + va;
            reactions[2] = reactions[2] + vb;
        }
    }

    // Calculating Reactions from uniform loads
    let mut uniform_loads_records: Array2<f64> = Array2::zeros((0,2));
    if check_uniform_loads > 0 {
        for (_i, u) in uniformloads.iter().enumerate(){
            let (va, vb) = uniform_load_reactions(u, a, b);
            uniform_loads_records.push_row(ArrayView::from(&[va,vb])).unwrap();
            reactions[0] = reactions[0] + va;
            reactions[2] = reactions[2] + vb;
        }
    }

    // Calculating Reactions from linear loads
    let mut linear_loads_records: Array2<f64> = Array2::zeros((0,2));
    if check_linear_loads > 0 {
        for (_i, l) in linearloads.iter().enumerate(){
            let (va, vb) = linear_load_reactions(l, a, b);
            linear_loads_records.push_row(ArrayView::from(&[va,vb])).unwrap();
            reactions[0] = reactions[0] + va;
            reactions[2] = reactions[2] + vb;
        }
    }

    // Shear and moment calculation from point loads
    if check_point_loads > 0 {
        for (i, p) in pointloads.iter().enumerate(){
            let (shear,moment) = point_load_shear_moment(&x_axis,p, point_loads_records.row(i),a,b);
            shear_froce.push_row(ArrayView::from(&shear)).unwrap();
            bending_moment.push_row(ArrayView::from(&moment)).unwrap();
        }
    }

    // Shear and moment calculation from point moments
    if check_point_moments > 0 {
        for (i, m) in pointmoments.iter().enumerate(){
            let (shear,moment) = point_moment_shear_moment(&x_axis,m, point_moments_records.row(i),a,b);
            shear_froce.push_row(ArrayView::from(&shear)).unwrap();
            bending_moment.push_row(ArrayView::from(&moment)).unwrap();
        }
    }

    // Shear and moment calculation from uniform loads
    if check_uniform_loads > 0 {
        for (i, u) in uniformloads.iter().enumerate(){
            let (shear,moment) = uniform_load_shear_moment(&x_axis,u, uniform_loads_records.row(i),a,b);
            shear_froce.push_row(ArrayView::from(&shear)).unwrap();
            bending_moment.push_row(ArrayView::from(&moment)).unwrap();
        }
    }

    // Shear and moment calculation from linear loads
    if check_linear_loads > 0 {
        for (i, l) in linearloads.iter().enumerate(){
            let (shear,moment) = linear_load_shear_moment(&x_axis,l, linear_loads_records.row(i),a,b);
            shear_froce.push_row(ArrayView::from(&shear)).unwrap();
            bending_moment.push_row(ArrayView::from(&moment)).unwrap();
        }
    }

    // format!("the vertical reaction at A is {} kN",reactions[0])
    (x_axis.to_vec(),shear_froce.sum_axis(Axis(0)).to_vec(),bending_moment.sum_axis(Axis(0)).to_vec())
}


// Get the reactions due to the point loads
fn point_load_reactions(point_load: &Vec<f64>, a: f64, b: f64)-> (f64,f64,f64){
    let xp: f64 = point_load[0];
    let fx: f64 = point_load[1];
    let fy: f64 = point_load[2];
    
    let la_p: f64 = a - xp;
    let mp: f64 = fy*la_p;
    let la_vb: f64 = b-a;

    let vb: f64 = mp/la_vb;
    let va: f64 = -fy-vb;
    let ha: f64 = -fx;

    (va, vb, ha)
}

// Get the shear and moment due to the point loads
fn point_load_shear_moment(x: &Array1<f64>, point_load: &Vec<f64>, point_load_record:ArrayView1<f64>, a: f64, b: f64)-> (Array1<f64>,Array1<f64>){
    let xp: f64 = point_load[0];
    let fy: f64 = point_load[2];
    let va: f64 = point_load_record[0];
    let vb: f64 = point_load_record[2];

    let mut shear = Array1::zeros(x.len());
    let mut moment = Array1::zeros(x.len());

    for (i, xval) in x.iter().enumerate(){
        let mut shearval: f64 = 0.0;
        let mut momentval: f64=  0.0;

        if xval>&a {
            shearval = shearval + va;
            momentval = momentval - va * (xval-a);
        }
        if xval>&b {
            shearval = shearval + vb;
            momentval = momentval - vb * (xval-b);
        }
        if xval>&xp{
            shearval = shearval + fy;
            momentval = momentval - fy * (xval-xp);
        }
        shear[i] = shearval;
        moment[i] = momentval;

    }

    (shear,moment)
}

// Get the reactions due to the point moments
fn point_moment_reactions(point_moment: &Vec<f64>,a: f64, b: f64) -> (f64,f64){
    let m: f64 = point_moment[1];

    let la_vb: f64 = b-a;

    let vb: f64 = m/la_vb;
    let va: f64 = -vb;

    (va,vb)
}

// Get the shear and moment due to the point moments
fn point_moment_shear_moment(x: &Array1<f64>, point_moment: &Vec<f64>, point_moment_record:ArrayView1<f64>, a: f64, b: f64)-> (Array1<f64>,Array1<f64>){
    let xm: f64 = point_moment[0];
    let m: f64 = point_moment[1];
    let va: f64 = point_moment_record[0];
    let vb: f64 = point_moment_record[1];

    let mut shear = Array1::zeros(x.len());
    let mut moment = Array1::zeros(x.len());

    for (i, xval) in x.iter().enumerate(){
        let mut shearval: f64 = 0.0;
        let mut momentval: f64=  0.0;

        if xval>&a {
            shearval = shearval + va;
            momentval = momentval - va * (xval-a);
        }
        if xval>&b {
            shearval = shearval + vb;
            momentval = momentval - vb * (xval-b);
        }
        if xval>&xm{
            momentval = momentval - m;
        }
        shear[i] = shearval;
        moment[i] = momentval;

    }

    (shear,moment)


}

// Get the reactions due to the uniform loads
fn uniform_load_reactions(uniform_load: &Vec<f64>, a: f64, b: f64)-> (f64,f64){
    let xstart: f64 = uniform_load[0];
    let xend: f64 = uniform_load[1];
    let fy: f64 = uniform_load[2];

    let resultant_fy: f64 = fy*(xend-xstart);
    let resultant_x: f64 = xstart + 0.5 * (xend-xstart);

    let la_p: f64 = a-resultant_x;
    let mp: f64 = resultant_fy * la_p;
    let la_vb: f64 = b-a;

    let vb: f64 = mp/la_vb;
    let va: f64 = -resultant_fy-vb;

    (va,vb)
}

// Get the shear and moment due to the uniform loads
fn uniform_load_shear_moment(x: &Array1<f64>, uniform_load: &Vec<f64>, uniform_load_record:ArrayView1<f64>, a: f64, b: f64)-> (Array1<f64>,Array1<f64>){
    let xstart: f64 = uniform_load[0];
    let xend: f64 = uniform_load[1];
    let fy: f64 = uniform_load[2];
    let va: f64 = uniform_load_record[0];
    let vb: f64 = uniform_load_record[1];

    let mut shear = Array1::zeros(x.len());
    let mut moment = Array1::zeros(x.len());

    for (i, xval) in x.iter().enumerate(){
        let mut shearval: f64 = 0.0;
        let mut momentval: f64=  0.0;

        if xval>&a {
            shearval = shearval + va;
            momentval = momentval - va * (xval-a);
        }
        if xval>&b {
            shearval = shearval + vb;
            momentval = momentval - vb * (xval-b);
        }
        if xval>&xstart && xval<= &xend{
            shearval = shearval + fy*(xval-xstart); //this is the integral of the straight line aka x
            momentval = momentval - fy * 0.5 * (xval-xstart) * (xval-xstart); //this is basically 1/2x^2 aka the integral of above :)
        } else if xval > &xend {
            shearval = shearval + fy * (xend-xstart);
            momentval = momentval - fy * 0.5 * (xval-xstart) * (xval-xstart);
        }
        shear[i] = shearval;
        moment[i] = momentval;

    }

    (shear,moment)
}

// Get the reactions due to the linear loads
fn linear_load_reactions(linear_load: &Vec<f64>, a: f64, b: f64)-> (f64,f64){
    let xstart: f64 = linear_load[0];
    let xend: f64 = linear_load[1];
    let fystart: f64 = linear_load[2];
    let fyend: f64 = linear_load[3];

    let resultant_fy: f64;
    let resultant_x: f64;

    // is it increasing triangle or decreasing
    if fystart.abs()>0.0{
        resultant_fy = 0.5 * fystart * (xend-xstart);
        resultant_x = xstart + 1.0/3.0 * (xend - xstart);
    } else {
        resultant_fy = 0.5 * fyend * (xend-xstart);
        resultant_x = xstart + 2.0/3.0 * (xend - xstart);
    }

    let la_p: f64 = a-resultant_x;
    let mp: f64 = resultant_fy * la_p;
    let la_vb: f64 = b-a;

    let vb: f64 = mp/la_vb;
    let va: f64 = -resultant_fy-vb;

    (va,vb)
}

// Get the shear and moment due to the linear loads
fn linear_load_shear_moment(x: &Array1<f64>, linear_load: &Vec<f64>, linear_load_record:ArrayView1<f64>, a: f64, b: f64)-> (Array1<f64>,Array1<f64>){
    let xstart: f64 = linear_load[0];
    let xend: f64 = linear_load[1];
    let fystart: f64 = linear_load[2];
    let fyend: f64 = linear_load[3];
    let va: f64 = linear_load_record[0];
    let vb: f64 = linear_load_record[1];

    let mut shear = Array1::zeros(x.len());
    let mut moment = Array1::zeros(x.len());

    for (i, xval) in x.iter().enumerate(){
        let mut shearval: f64 = 0.0;
        let mut momentval: f64=  0.0;

        if xval>&a {
            shearval = shearval + va;
            momentval = momentval - va * (xval-a);
        }
        if xval>&b {
            shearval = shearval + vb;
            momentval = momentval - vb * (xval-b);
        }
        if xval>&xstart && xval<= &xend{
            if fystart.abs()>0.0{
                let xbase: f64 = xval - xstart;
                let fcut: f64 = fystart - xbase * (fystart/(xend-xstart));
                let r1: f64 = 0.5 * xbase * (fystart - fcut);
                let r2: f64 = xbase * fcut;
                shearval = shearval + r1 + r2;
                momentval = momentval - r1 * 2.0/3.0 * xbase - r2 * (xbase/2.0);
            } else {
                let xbase: f64 = xval - xstart;
                let fcut: f64 = fyend * (xbase/(xend-xstart));
                let r: f64 = 0.5 * xbase * (fcut);
                shearval = shearval + r;
                momentval = momentval - r * xbase/3.0;
            }
        } else if xval > &xend {
            if fystart.abs()>0.0{
                let r: f64 = 0.5*fystart*(xend-xstart);
                let xr = xstart + 1.0/3.0*(xend-xstart);
                shearval = shearval + r;
                momentval = momentval - r*(xval-xr);
            } else {
                let r: f64 = 0.5*fyend*(xend-xstart);
                let xr = xstart + 2.0/3.0*(xend-xstart);
                shearval = shearval + r;
                momentval = momentval - r*(xval-xr);
            }
        }
        shear[i] = shearval;
        moment[i] = momentval;

    }

    (shear,moment)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![execute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

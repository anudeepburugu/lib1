
//   #[macro_use(bitflags)]
// extern crate bitflags;

pub mod quad {
use super::two_dim::*;
/*pub fn update(side1:f32,side2:f32,side3:f32,side4:f32,t:f32)->Result<Vec<(f32,f32)>,MYError>{

    let mut res=Results{tfv:0.0,qfv:0.0};
    let x2=side1*t.to_radians().cos();
    let y2=side1*t.to_radians().sin();
let (x4,y4)= (side4,0.0);
let area2;

    let _area1=Results::area_from_sas(SAS{side1:side4,side2:side1,angle:t}).expect("failure");
    let diagonal=Results::distance_from_points((x2,y2), (x4,y4));
    if let Ok(area)=res.area_from_sides(Sides{side1:diagonal,side2:side2,side3:side3}){
        area2=area;
    }else{
        area2=0.0;
    }
let alpha=Results::angle_from_sides(side4, diagonal, area2) as f32;
    let (x1,y1)=(-side1*(alpha).cos(),-side1*(alpha).sin());
   if let Ok(vertices)=Results::vertices_from_sides(Sides{side1:side2,side2:side3,side3:diagonal}){
Ok(vec![
    (x1,y1),
    vertices.point1,
    vertices.point2,
    vertices.point3,
    (x1,y1),

// (area1+area2,0.0),
    ])
   }else{
      Err(MYError::CustomError)
   }

    

}*/


pub fn round_n(a:i32,val:f32)->f32{
    let r=10.0f32.powi(a);
    let value=(val*r) as i32;
    value as f32/r

}

pub fn update(side1:f32,side2:f32,side3:f32,side4:f32,t:f32)->Result<(Vec<(f32,f32)>,Vec<f32>),MYError>{

    let mut res=Results{tfv:0.0,qfv:0.0};
    let x4=round_n(3,side4*t.to_radians().cos());
    let y4=round_n(3,side4*t.to_radians().sin());
let (x2,y2)= (side1,0.0);
let area2;

    let area1=Results::area_from_sas(SAS{side1:side4,side2:side1,angle:t}).expect("failure");
    let diagonal=Results::distance_from_points((x2,y2), (x4,y4));
    if let Ok(area)=res.area_from_sides(Sides{side1:diagonal,side2:side2,side3:side3}){
        area2=area;
    }else{
        area2=0.0;
    }
let alpha=Results::angle_from_sides(side1, diagonal, area1);
    let (x1,y1)=(round_n(3,side1*alpha.cos()),round_n(3,-side1*alpha.sin()));
   if let Ok(vertices)=Results::vertices_from_sides(Sides{side1:side2,side2:side3,side3:diagonal}){
// let diag2=Results::side_from_angle(side1: f32, side2: f32, ang_rad: f32)

let diag2=Results::distance_from_points(vertices.point2, (x1,y1));
if let Ok(area_te)=res.area_from_sides(Sides{side1:side1,side2:side2,side3:diag2}){
let alpha2=Results::angle_from_sides(diag2,side1,area_te);
let (x3,y3)=(round_n(3,diag2*alpha2.cos()),round_n(3,diag2*alpha2.sin()));
// println!("{:?}",vec![(x1,y1),vertices.point1,vertices.point2,vertices.point3,(x1,y1),]);
// println!("{}", res.area_from_vertices(Vertices{point1:(0.0,0.0),point2:(round_n(3,x2),round_n(3,y2)),point3:(round_n(3,x3),round_n(3,y3))})+res.area_from_vertices(Vertices{point3:(round_n(3,x4),round_n(3,y4)),point2:(round_n(3,x2),round_n(3,y2)),point1:(round_n(3,x3),round_n(3,y3))}),);
// println!("{:?} {:?}",diag2,diagonal,area1+area2);

Ok((vec![(0.0,0.0),(round_n(3,x2),round_n(3,y2)),(round_n(3,x3),round_n(3,y3)),(round_n(3,x4),round_n(3,y4)),(0.0,0.0),],vec![area2+area1,diagonal,diag2]))

}else{
    Err(MYError::CustomError)
}}else{
    Err(MYError::CustomError)

}
}

pub fn get_results(side1:f32,side2:f32,side3:f32,side4:f32,area:f32)->(Vec<Vec<(f32,f32)>>,Vec<Vec<f32>>){
let mut vec_values:Vec<Vec<(f32,f32)>>=vec![];
let mut vec_areas:Vec<Vec<f32>>=vec![];

for i in 700..1100{
    if let Ok(value)= update(side1,side2,side3,side4,i as f32/10.0){
if round_n(2,value.1[0]) == round_n(2, area){
 vec_values.clear();
 vec_values.push(value.0.to_owned());
            vec_areas.clear();
            vec_areas.push(value.1);
            break;
}else if value.1[0] < area.ceil() && value.1[0] > area.floor(){
            vec_values.push(value.0.to_owned());
            vec_areas.push(value.1);
}

    }
}
(vec_values,vec_areas)
}
}

// use std::string::String;
pub mod two_dim {
use std::{fmt};
use std::result::Result;
// use libm;

#[allow(unused,dead_code,unused_variables,unused_must_use)]
use std::sync::{Arc,Mutex, mpsc};
// #[derive(Copy,Clone)]
pub struct Sides{
pub side1:f32,
pub side2:f32,
pub side3:f32,
}

// #[derive(Copy,Clone)]
pub struct Vertices{
 pub point1:(f32,f32),
 pub        point2:(f32,f32),
 pub        point3:(f32,f32),
}

// #[derive(Copy,Clone)]
pub struct SAS{
pub angle:f32,
pub side1:f32,
pub side2:f32
}

#[derive(Copy,Clone)]
pub struct Qsides{
pub side1:f32,
pub side2:f32,
pub side3:f32,
pub side4:f32,
}



pub fn max_min(vec:Vec<f32>)->(f32,f32,Vec<f32>){
    let mut max:f32=0.0;
    let mut min:f32=vec[0].to_owned();
for value in vec.iter(){
    if *value > max{
        max=*value;
    }else if *value<=0.0{
min=*value;
    }
}
    return (max,min,vec);
}


// #[derive(Err)]
// enum ErrorKind{
    // InvalidTriangleSides,
// }
#[derive(Copy,Clone)]
pub struct Results{
    // tfs:f32,
   pub tfv:f32,
    // tfa:f32,
    // qfs:f32,
   pub qfv:f32,
    // ar_c:f32,
    // vol_cube:f32,

}
#[derive(Copy,Clone)]
pub struct Qvertices{
   pub point1:(f32,f32),
   pub point2:(f32,f32),
   pub point3:(f32,f32),
    pub point4:(f32,f32),

}
/*
fn index_of<T>( list:&Vec<T>,value:T,idx:&mut Option<i32>)where T:PartialOrd {
    let mut indx=0i32;
    for val in list.iter(){
        if *val== value{
            *idx=Some(indx);
        }else{
*idx=  None;
        }
        indx+=1;
    }
}
*/

pub enum MYError{
 CustomError,
}
impl std::error::Error for MYError{

}
impl std::fmt::Debug for MYError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    
    write!(f,"seems to be an Error!! ")
    }
}
impl std::fmt::Display for MYError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f," Error Occured")
    }
}

 pub trait Geom{
     fn angle_from_points(points:Vec<(f32,f32)>)->f32;
fn angle_from_sides(side1:f32,side2:f32,area:f32)->f32;
fn side_from_angle(side1:f32,side2:f32,ang_rad:f32)->f32;
 fn side_from_area( sides:Sides,tfs:f32)->Result<(f32,f32),MYError>;
fn area_from_sides(&mut self, sides:Sides)->Result<f32,MYError>;
fn area_from_vertices(&mut self,sides:Vertices)->f32;
fn area_from_sas(sides:SAS)->Result<f32,MYError>;
fn quadarea_from_sides(  self,sides:Qsides)->Vec<f32>;
// fn quadarea_from_sides_approx(&mut self,sides:Vec<f32>)->f32;
fn distance_from_points(point1:(f32,f32),point2:(f32,f32))->f32;
fn quadarea_from_vertices(&mut self,sides:Qvertices);
fn get_values(&mut self, val1:f32,val2:f32,val3:f32,val4:f32)->Result<f32,MYError>;
fn point_from_area(point1:(f32,f32),point2:(f32,f32),area:f32)->f32;
fn vertices_from_sides(sides:Sides)->Result<Vertices,MYError>;
fn center_from_distance(points:Vec<(f32,f32)>,angle:f32)->((f32,f32),(f32,f32));
fn point_from_distance(points:Vec<(f32,f32)>,d:f32,dir:char,parallel:bool)->(f32,f32);
}

use super::quad::round_n;
impl Geom for Results{
    fn point_from_distance(points:Vec<(f32,f32)>,d:f32,dir:char,parallel:bool)->(f32,f32){
        
        let mut m= Self::angle_from_points(points[0..2].to_owned()).to_radians().tan();
        if !parallel{
            m=-(1.0/m);
        }
        let point=points[2];
let intercept= point.1-m*point.0;

let b=2.0*(-point.0-m*point.1+m*intercept);
let a= 1.0+m.powi(2);
let c= point.1.powi(2)+point.0.powi(2)-d.powi(2)+intercept.powi(2)-2.0*point.1*intercept;
let x1=(-b+((b.powi(2)-4.0*a*c).powf(0.5)))/(2.0*a);
let x2=(-b-((b.powi(2)-4.0*a*c).powf(0.5)))/(2.0*a);
let y1=m*x1+intercept;
let y2=m*x2+intercept;

// println!("{:?} {:?}",(x1,y1),(x2,y2));
if dir == '+'{
if y2>y1{
    (x2,y2)
}else{
    (x1,y1)
}
}
else {
    if y1<y2{
        (x1,y1)
    }else{
        (x2,y2)
    }    

}

    }


     fn angle_from_points(points:Vec<(f32,f32)>)->f32{
         (((points[0].1-points[1].1)/(points[0].0-points[1].0)).atan())*(180f32/std::f32::consts::PI)
     }
/// angle not in radians
fn center_from_distance( points:Vec<(f32,f32)>,angle:f32)->((f32,f32),(f32,f32)){
let dist= Self::distance_from_points(points[0].to_owned(),points[1].to_owned());
let (x1,y1)= points[0];
let (x2,y2)= points[1];
let  ox1;
let  ox2;
let  oy2;
let  oy1;
let (c1,c2)= ((x1+x2)/2.0,(y1+y2)/2.0);
let d= (dist*(angle/2.0).to_radians().cos())/(2.0*(angle/2.0).to_radians().sin());
if x2==x1 {
ox1=d+c1;
oy1=c2;
ox2=-d+c1;
oy2=c2;
}else if y2==y1{
ox2=c1;
oy2=d+c2;
ox1=c1;
oy1=-d+c2;
}else{
let m=(x1-x2)/(y2-y1);
let c=((y1+y2)/2.0) - m*(x1+x2)/2.0;
let aq= 1.0+m.powi(2);
let bq= 2.0*m*(c-c2)-(2.0*c1);
let cq= c1.powi(2) + (c-c2).powi(2) - d.powi(2);
ox1= (-bq + (bq.powi(2)-4.0*aq*cq).powf(0.5))/(2.0*aq);
ox2= (-bq - (bq.powi(2)-4.0*aq*cq).powf(0.5))/(2.0*aq);
oy1= m*ox1+c;
oy2= m*ox2+c;
}
((ox1,oy1),(ox2,oy2))
}
    ///input angle in radians
    fn side_from_angle(side1: f32, side2: f32,ang_rad:f32)->f32{
let x=round_n(3,side1*ang_rad.cos());
let y=round_n(3,side1*ang_rad.sin());
Self::distance_from_points((x,y), (side2,0.0))
    }
fn point_from_area(_point1:(f32,f32),_point2:(f32,f32),_area:f32)->f32{
0.0
}
    fn distance_from_points(point1:(f32,f32),point2:(f32,f32))->f32{
((point1.0-point2.0).powi(2)+(point1.1-point2.1).powi(2)).powf(0.5)
    }


    fn side_from_area( sides:Sides,tfs:f32)->Result<(f32,f32),MYError>{
let y1=tfs*2f32/sides.side1;
if sides.side2.powi(2)-y1.powi(2)>0.0{
let x1= (sides.side2.powi(2)-y1.powi(2)).powf(0.5);
Ok((((x1-sides.side1).powi(2)+ y1.powi(2)).powf(0.5),((x1+sides.side1).powi(2)+ y1.powi(2)).powf(0.5)))

}else{
    Err(MYError::CustomError)
}

    }

fn quadarea_from_vertices(&mut self,sides:Qvertices){
    self.area_from_vertices( Vertices{point1:sides.point1,point2:sides.point2,point3:sides.point3});
let value1= self.tfv; 
    self.area_from_vertices(Vertices{point1:sides.point2,point2:sides.point3,point3:sides.point4});
    self.qfv=value1+self.tfv;
}


 fn vertices_from_sides(sides:Sides)->Result<Vertices,MYError>{
    if sides.side1+sides.side2 >sides.side3 && sides.side1+sides.side3 >sides.side2 && sides.side3+sides.side2 >sides.side1{

    Ok(Vertices {
        point1:(0.0,0.0),
        // point2:(sides.side1,0.0),
        point3:(sides.side3,0.0),
        point2:{
            let x=(sides.side1.powi(2)-sides.side2.powi(2)+ sides.side3.powi(2)).abs()/(2.0*sides.side3);
let y= (sides.side1.powi(2) - x.powi(2)).powf(0.5);

            // let x=(-sides.side2.powi(2)+sides.side3.powi(2)- sides.side1.powi(2)).abs()/(2.0*sides.side1);
// let y= (sides.side2.powi(2) - x.powi(2)).powf(0.5);
(x,y)}
 })
}else{
    Err(MYError::CustomError)
}
}

fn area_from_sides(&mut self, sides:Sides)->Result<f32,MYError>{
    let side=sides.side3;
    let vers=Self::vertices_from_sides(sides)?;
        Ok(side*0.5*vers.point2.1)
    }

fn area_from_vertices(&mut self,sides:Vertices)->f32{
        // if sides.point1!=None && sides.point2!=None && sides.point3!=None{
            let imx=[(sides.point1.0-sides.point2.0).abs(),
            (sides.point1.0-sides.point3.0).abs(),
            (sides.point2.0-sides.point3.0).abs()].to_vec();
            let imy= [(sides.point1.1-sides.point2.1).abs(),
            (sides.point1.1-sides.point3.1).abs(),
            (sides.point2.1-sides.point3.1).abs()].to_vec();

let (max_x,_,imx)=max_min(imx);
let (max_y,_,imy)=max_min(imy);
max_x*max_y-(0.5*(imx[0]*imy[0]+imx[1]*imy[1]+imx[2]*imy[2]))
// }else{
        //    Err(io::Error::new(io::ErrorKind::NotFound, "value not found"))

// }
}
fn angle_from_sides(side1:f32,side2:f32,area:f32)->f32{
// std::f32::consts::
   round_n(3,(2.0*area/(side1*side2)).asin())
}
fn area_from_sas(sides:SAS)->Result<f32,MYError>{
        // if sides.side1!=None && sides.side2!=None && sides.angle!=None{

    let value= 0.5*sides.side1*sides.side2*round_n(3,sides.angle.to_radians().sin());
        // Ok(value)
    // }else{
        // Err(MYError::CustomError)
    // }
    if value>0.0{
Ok(value)

    }else{
        Err(MYError::CustomError)
    }
// }else{
        //    Err(io::Error::new(io::ErrorKind::NotFound, "value not found"))

// }
}


fn get_values(&mut self, val1:f32,val2:f32,val3:f32,val4:f32 )->Result<f32,MYError>{
 if let Ok(val)=self.area_from_sides( Sides{
side1:(val1.powi(2) + val2.powi(2)).powf(0.5),
side2:val3,
side3:val4
    }){
    
Ok(0.5*val1*val2+ val)
}else{
Err(MYError::CustomError)
}
}

fn quadarea_from_sides(self,values:Qsides)->Vec<f32>{
    // let mut vecs:Vec<f32>= vec![];

let  vecs_mutex= std::sync::Arc::new(Mutex::new(vec![]));
let  self_mutex= std::sync::Arc::new(Mutex::new(self));

let mut threads:Vec<std::thread::JoinHandle<()>>=vec![];

for j in 0..180{
    let vecs= vecs_mutex.clone();
    let self_ex= self_mutex.clone();
let thread= std::thread::spawn(move||{
for i in j*10..(j+1)*10{
if let Ok(tfs)= Self::area_from_sas(SAS{angle:(i as f32/10.0).to_radians(),side1:values.side1,side2:values.side2,}){
if let Ok((side3s,side3l))= Self::side_from_area(Sides{side1:values.side3,side2:values.side4,side3:0.0}, tfs){

if let Ok(tfs2)=  self_ex.lock().unwrap().area_from_sides(Sides{side1:values.side3,side2:values.side4,side3:side3s}){
vecs.lock().unwrap().push(tfs +tfs2);
}
if let Ok(tfs2)=  self_ex.lock().unwrap().area_from_sides(Sides{side1:values.side3,side2:values.side4,side3:side3l}){
vecs.lock().unwrap().push(tfs +tfs2);
}
}}}

});
threads.push(thread);
}
for thread in threads{
    thread.join().unwrap();
}
let val= vecs_mutex.lock().unwrap().clone();
val
}


}



}
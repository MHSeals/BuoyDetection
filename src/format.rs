use crate::buoy::{
    Buoy,
    AllBuoy,    
};

pub fn format(buoys: AllBuoy, heading: f32) -> String{
    let mut formatted_string: String;
    formatted_string = format!("HEADING {heading}");
    for buoy in buoys{
        formatted_string = format!("{}\nCOLOR {:?}\nSIZE {:?}\nCOORDINATE_X {}\nCOORDINATE_Y {}\nDISTANCE {}\n", formatted_string, buoy.get_color(), buoy.get_size(), buoy.get_coordinates().0, buoy.get_coordinates().0, buoy.get_distance());
    }
    return formatted_string;
}

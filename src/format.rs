use crate::buoy::{
    Buoy,
    AllBuoy,    
};

pub fn format(buoys: AllBuoy, heading: f32) -> String{
    let mut formatted_string: String;
    formatted_string = format!("HEADING {heading}");
    if buoys.get_len() >= 1{
        formatted_string = format!("{}\n\nFOUND", formatted_string);
    }
    else{
        formatted_string = format!("{}\n\n!FOUND", formatted_string);
    }
    for buoy in buoys{
        formatted_string = format!("{}\nCOLOR {:?}\nSIZE {:?}\nCOORDINATE_X {}\nCOORDINATE_Y {}\nDISTANCE {}\n", formatted_string, buoy.get_color(), buoy.get_size(), buoy.get_coordinates().0, buoy.get_coordinates().0, buoy.get_distance());
    }
    return formatted_string;
}

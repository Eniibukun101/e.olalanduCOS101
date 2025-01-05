
// use std::io::Read;

// fn main(){

//     let commisioners_list = get_text("commisioner.txt");
//     let commisioners: Vec<&str> = commisioners_list.trim().split('\n').collect();
//     let ministry_list = get_text("ministry.txt");
//     let ministry: Vec<&str> = ministry_list.trim().split('\n').collect();
//     let geopolitical_zone_list = get_text("geopolitical_zone.txt");
//     let geopolitical_zone: Vec<&str> = geopolitical_zone_list.trim().split('\n').collect();

//     println!("S/N, Name of Commisioners, Ministry, Geopolitical Zone");
//     for i in 0..commisioners.len(){

//         println!("{}) {}, {}, {}", i + 1, commisioners[i], ministry[i], geopolitical_zone[i]);

//     }
// }

// fn get_text(location: &str) -> String{

//     let mut file = std::fs::File::open(location).unwrap();
//     let mut content = String::new();
//     file.read_to_string(&mut content).unwrap();
//     content

// }


fn main() {
    // Define separate datasets
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Ministry of Internal Affairs",
        "Ministry of Justice",
        "Ministry of Defence",
        "Ministry of Power & Steel",
        "Ministry of petroleum",
    ];

    let zones = vec![
        "South-West",
        "North-East",
        "South-South",
        "South-West",
        "South-East",
    ];

   for (i, (name, (ministry, zone))) in names.iter().zip(ministries.iter().zip(zones.iter())).enumerate() {
        println!("Minister {}: {}, {}, {}", i + 1, name, ministry, zone);
    }
}
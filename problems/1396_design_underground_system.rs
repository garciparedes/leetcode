use std::collections::HashMap;

struct UndergroundSystem {
    currently_in: HashMap<i32, (String, i32)>,
    averages: HashMap<String, HashMap<String, (f64, f64)>>,   
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        UndergroundSystem {
            currently_in: HashMap::new(),
            averages: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.currently_in.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (origin, t0) = self.currently_in.remove(&id).unwrap();
        
        self.averages.entry(origin)
            .or_insert_with(HashMap::new)
            .entry(station_name)
            .and_modify(|e| {
                e.0 += (t - t0) as f64;
                e.1 += 1 as f64;
            })
            .or_insert(((t - t0) as f64, 1.0));
    }
    
    fn get_average_time(&mut self, start_station: String, end_station: String) -> f64 {
        let (cum, n) = self.averages
            .entry(start_station)
            .or_insert_with(HashMap::new)
            .get(&end_station)
            .unwrap();
        
        return *cum / *n;
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */

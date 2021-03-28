pub enum TrafficLights {
    RED(usize),
    YELLOW(usize),
    GREEN(usize)
}

pub trait PrintTrafficLights {
    fn time(&self) -> usize;

    fn as_str(&self) -> String;
}

impl PrintTrafficLights for TrafficLights {
    fn time(&self) -> usize {
        match *self {
            TrafficLights::RED(num) => num,
            TrafficLights::YELLOW(num) => num,
            TrafficLights::GREEN(num) => num,
        }
    }

    fn as_str(&self) -> String {
        match *self {
            TrafficLights::RED(num) => format!("traffic light red time is: {}", num),
            TrafficLights::YELLOW(num) => format!("traffic light yellow time is: {}", num),
            TrafficLights::GREEN(num) => format!("traffic light green time is: {}", num)
        }
    }
}

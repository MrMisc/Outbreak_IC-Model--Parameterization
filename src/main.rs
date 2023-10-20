extern crate plotly;

use plotly::common::Line;
use plotly::common::{Mode,Title,Marker};
use plotly::color::Rgba;
use plotly::layout::AxisType;
use plotly::{Plot, Scatter,Layout};
use plotly::layout::{Axis,Shape};
// use std::thread;
use rand::distributions::Uniform;
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use statrs::distribution::{Normal, Poisson, StudentsT, Triangular, Weibull,Gamma};
extern crate rayon;
use rayon::prelude::*;

extern crate serde;
// extern crate serde_json;
// use serde::Deserializer;
// use serde::{Deserialize, Serialize};
// use serde_json::json;

// use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
// use std::io::Read;
use std::io::Write;
// use std::time::{Duration, Instant};
use std::{fs, io, process};
// use std::error::Error;

use csv::Writer;


pub mod limits{
    pub fn min(a:f64,b:f64)->f64{
        if a<b{
            a
        }
        else{
            b
        }
    }
    pub fn max(a:f64,b:f64)->f64{
        if a<b{
            b
        }
        else{
            a
        }
    }
}

pub fn poisson(rate:f64)->u64{
    let mut rng = thread_rng();
    let v: &Vec<f64> = &Poisson::new(rate)
    .unwrap()
    .sample_iter(&mut rng)
    .take(1)
    .collect();     
    v[0] as u64
}

pub fn normal(mean: f64, std: f64, upper:f64) -> f64 {
    let mut thing: f64 = 0.0;
    loop {
        // println!("Mean value is {}, STD value is {}",mean,std);
        let mut rng = thread_rng();
        let v: &Vec<f64> = &Normal::new(limits::max(0.0001,mean), limits::max(0.0001,std))
            .unwrap()
            .sample_iter(&mut rng)
            .take(1)
            .collect();
        thing = v[0];
        if thing > 0.0 && thing<upper{
            break;
        }
    }
    thing
}

pub fn normal_(min:f64,mean: f64, std: f64, upper:f64) -> f64 {
    let mut thing: f64 = 0.0;
    loop {
        // println!("Mean value is {}, STD value is {}",mean,std);
        let mut rng = thread_rng();
        let v: &Vec<f64> = &Normal::new(limits::max(0.0001,mean), limits::max(0.0001,std))
            .unwrap()
            .sample_iter(&mut rng)
            .take(1)
            .collect();
        thing = v[0];
        if thing > min && thing<upper{
            break;
        }
    }
    thing
}

pub fn gamma(alpha:f64, beta:f64)->f64{
    let mut rng = thread_rng();
    let v: &Vec<f64> =
    &Gamma::new(alpha, beta)
        .unwrap()
        .sample_iter(&mut rng)
        .take(1)
        .collect();
    v[0]
}

pub fn roll(prob:f64)->bool{
    let mut rng = thread_rng();
    let roll = Uniform::new(0.0, 1.0);
    let rollnumber: f64 = rng.sample(roll);
    rollnumber<prob    
}

pub fn uniform(start:f64,end:f64)->f64{
    let mut rng = thread_rng();
    let roll = Uniform::new(start, end);
    let rollnumber: f64 = rng.sample(roll);
    rollnumber
}

#[derive(Clone)]
pub struct Zone_3D{
    segments:Vec<Segment_3D>, 
    zone:usize,
    capacity:u32,
    eviscerate:bool
}

#[derive(Clone)]
pub struct Segment_3D{
    zone:usize,
    origin_x:u64,
    origin_y:u64,
    origin_z:u64,
    range_x:u64,
    range_y:u64,
    range_z:u64,
    capacity:u32,
    eviscerated:bool
}

pub struct Eviscerator{
    zone:usize,
    infected: bool,
    number_of_times_infected:u8
}

impl Segment_3D{
    fn generate(&mut self,infected:bool,colonized:bool,mut n:u32,atc0:f64, atc1:f64,probability:f64,mut hosts:&mut Vec<host>){
        //fn new(zone:usize, std:f64,loc_x:f64, loc_y:f64,loc_z:f64,restriction:bool,range_x:u64,range_y:u64,range_z:u64, atc0:f64, atc1:f64,probability:f64)->host{
        if n<self.capacity{
            self.capacity-=n;
        }
        // if nig>self.capacity{
        //     nig = self.capacity;
        // }
        // let new_segment = Segment_3D {
        //     zone: self.zone,
        //     origin_x: self.origin_x,
        //     origin_y: self.origin_y,
        //     origin_z: self.origin_z,
        //     range_x: self.range_x,
        //     range_y: self.range_y,
        //     range_z: self.range_z,
        //     capacity: self.capacity-nig,
        //     eviscerated: self.eviscerated,
        // };        
        if !infected{hosts.push(host::new(self.zone,0.0,self.origin_x as f64,self.origin_y as f64,self.origin_z as f64,RESTRICTION,self.range_x,self.range_y,self.range_z,atc0,atc1,probability));}
        else{
            let mut host_to_add:host = host::new_inf(self.zone,0.0,self.origin_x as f64,self.origin_y as f64,self.origin_z as f64,RESTRICTION,self.range_x,self.range_y,self.range_z,atc0,atc1,probability);
            host_to_add.colonized = colonized;
            hosts.push(host_to_add);
        }
        // new_segment
    }
}

impl Zone_3D{
    fn add(&mut self)->[u64;7]{
        // println!("Adding to {}",self.zone);
        //Arbitrary numbers that I set beforehand. Simply 
        let mut origin_x:u64 = 200000;
        let mut origin_y:u64 = 200000;
        let mut origin_z:u64 = 200000;
        let mut range_x:u64 = 0;
        let mut range_y:u64 = 0;
        let mut range_z:u64 = 0;
        self.segments.sort_by(|segment_a,segment_b| segment_b.capacity.cmp(&segment_a.capacity)); //sort vector by capacity first -> Compare accordingly so that 
        if let Some(first_space) = self.segments.iter_mut().find(|segment| segment.capacity >= 1){ //0 occupants 
            //Segment capacity update
            first_space.capacity -= 1; //add 1 occupant
            origin_x = first_space.origin_x;
            origin_y = first_space.origin_y;
            origin_z = first_space.origin_z;
            range_x = first_space.range_x;
            range_y = first_space.range_y;
            range_z = first_space.range_z;
        }        
        let condition:bool = origin_x != 200000 && origin_y != 200000 && origin_z != 200000;
        if condition{
            self.capacity-=1;
        }
        [condition as u64,origin_x,origin_y,origin_z,range_x,range_y,range_z]
    }
    fn subtract(&mut self,x:u64,y:u64,z:u64){
        let mut counter:bool = false;
        // println!("Reached subtract function!");
        self.segments.iter_mut().for_each(|mut seg| {
            // println!("x vs segment x is {} vs {}",x,seg.origin_x);
            if x == seg.origin_x && y == seg.origin_y && z == seg.origin_z{
                // println!("Reaching inside of subtract function here!");
                seg.capacity += 1;
                self.capacity+=1;
                counter = true;
            }
        });
    }
    fn generate_empty(zone:usize,grid:[u64;3],step:[usize;3])->Zone_3D{
        let mut vector:Vec<Segment_3D> = Vec::new();
        let mut count:u32 = 0;
        for x in (0..grid[0]).step_by(step[0]){
            for y in (0..grid[1]).step_by(step[1]){
                for z in (0..grid[2]).step_by(step[2]){
                    vector.push(Segment_3D{zone:zone.clone(),origin_x:x,origin_y:y,origin_z:z,range_x:step.clone()[0] as u64,range_y:step.clone()[1] as u64,range_z:step.clone()[2] as u64,capacity:NO_OF_HOSTS_PER_SEGMENT[zone] as u32, eviscerated:false});
                    count+=NO_OF_HOSTS_PER_SEGMENT[zone] as u32;
                }
            }
        }
        Zone_3D{segments:vector,zone:zone, capacity:count,eviscerate:EVISCERATE_ZONES.contains(&zone)}
    }
    fn generate_full(zone:usize,grid:[u64;2],step:[usize;3])->Zone_3D{
        let mut vector:Vec<Segment_3D> = Vec::new();
        for x in (0..grid[0]).step_by(step[0]){
            for y in (0..grid[1]).step_by(step[1]){
                for z in (0..grid[2]).step_by(step[2]){
                    vector.push(Segment_3D{zone:zone.clone(),origin_x:x,origin_y:y,origin_z:z,range_x:step.clone()[0] as u64,range_y:step.clone()[1] as u64,range_z:step.clone()[2] as u64,capacity:0,eviscerated:false})
                }
            }
        }
        Zone_3D{segments:vector,zone:zone, capacity:0,eviscerate:EVISCERATE_ZONES.contains(&zone)}
    }    
    fn feed_setup(self, vector:Vec<host>, time:usize,feed_pd:f64)->Vec<host>{
        let mut vec:Vec<host> = vector.clone();
        for segment in self.clone().segments{
            host::feed(&mut vec,segment.origin_x.clone(),segment.origin_y.clone(),segment.origin_z.clone(),segment.zone.clone(),time,feed_pd);
        }
        vec
    }
    // fn eviscerate(&mut self,eviscerators:&mut Vec<Eviscerator>, vector:&mut Vec<host>,time:usize){
    //     //Filter out eviscerators that are for the zone in particular
    //     let mut evs: Vec<&mut Eviscerator> = eviscerators.iter_mut().filter(|ev| ev.zone == self.zone).collect();
    //     // Define the step size for comparison
    //     let step_size = evs.len();

    //     // Iterate over the smaller vector and compare with elements in the larger vector at regular intervals
    //     for (i, eviscerator) in evs.iter_mut().enumerate() {
    //         let start_index = i; // Start index in the larger vector for this eviscerator
    //         for (j, host) in vector.iter_mut().skip(start_index).step_by(step_size).enumerate() {
    //             // Compare and update the elements in the larger vector
    //             // if eviscerator.values_are_greater(larger_value) {
    //             //     *larger_value = eviscerator.values.clone(); // Assuming your struct has a clone method
    //             if host.infected && host.zone == eviscerator.zone{
    //                 eviscerator.infected = true;
    //                 // println!("EVISCERATOR HAS BEEN INFECTED AT TIME {} of this chicken stock entering zone!",host.time);
    //                 eviscerator.number_of_times_infected = 0;
    //                 println!("{} {} {} {} {} {}",host.x,host.y,host.z,12,time,host.zone);
    //             }else if eviscerator.infected && host.zone == eviscerator.zone{
    //                 // println!("Confirming that an eviscerator is infected in zone {}",eviscerator.zone);
    //                 host.infected = host.transfer(limits::max(0.0,1.0-(eviscerator.number_of_times_infected as f64)*EVISCERATOR_TO_HOST_PROBABILITY_DECAY));
    //                 eviscerator.number_of_times_infected += 1;
    //                 if host.infected{
    //                     println!("{} {} {} {} {} {}",host.x,host.y,host.z,11,time,host.zone);
    //                     // panic!("Evisceration has infected a host!!!");
    //                 }
    //             }
    //             //Decay of infection
    //             if eviscerator.number_of_times_infected>=EVISCERATE_DECAY{
    //                 eviscerator.infected = false;
    //             }
    //         }
    //     }
        
    // }
    fn modify(&mut self, start:[u64;3], end:[u64;3], range:[usize;3]){//range is synonymous with the stepsize in side STEP -> we are modifying the step here ---> hetero dims will not be reflected visually
        self.segments.retain(|segment| start.iter().zip([segment.origin_x,segment.origin_y,segment.origin_z].iter()).all(|(|&a,&b)| a<=b)==false && end.iter().zip([segment.origin_x,segment.origin_y,segment.origin_z].iter()).all(|(|&a,&b)| a>=b) == false);

        for x in (start[0]..end[0]).step_by(range[0]){
            for y in (start[1]..end[1]).step_by(range[1]){
                for z in (start[2]..end[2]).step_by(range[2]){
                    self.segments.push(Segment_3D{zone:self.zone,origin_x:x,origin_y:y,origin_z:z,range_x:range.clone()[0] as u64,range_y:range.clone()[1] as u64,range_z:range.clone()[2] as u64,capacity:self.segments[0].capacity,eviscerated:self.eviscerate})
                }
            }
        }
    }
    fn eviscerate(&mut self,eviscerators:&mut Vec<Eviscerator>, vector:&mut Vec<host>,time:usize){
        //DO THE MISHAP EXPLOSION BEFOREHAND
        // Decision:If the mishap explosion does not even exceed the spacing between the evisceration belt in side the eviscerationi zone, we do not need to bother doing mishap explosions
        if MISHAP && MISHAP_RADIUS as u64>self.segments[0].range_x{
            let index_spacing:usize = (MISHAP_RADIUS as u64/ self.segments[0].range_x) as usize;
            vector.sort_by(|a,b| a.origin_x.cmp(&b.origin_x));
            let mut ind:Vec<usize> = Vec::new();
            vector.iter_mut().enumerate().for_each(|(idx,mut host)| {
                if EVISCERATE_ZONES.contains(&host.zone) && host.infected && roll(MISHAP_PROBABILITY){
                    // panic!("Kaboom!");
                    ind.push(idx);
                }
            });
            for &idx in &ind{
                let start_index = if idx >= index_spacing {idx-index_spacing}else{0};
                let end_index = std::cmp::min(idx+index_spacing+1,vector.len());
                for host in &mut vector[start_index..end_index]{
                    host.infected = true;
                    println!("{} {} {} {} {} {}",host.x,host.y,host.z,13,time,host.zone);
                }
            }            
        }

        //Filter out eviscerators that are for the zone in particular
        let mut evs: Vec<&mut Eviscerator> = eviscerators.iter_mut().filter(|ev| ev.zone == self.zone).collect();
        // Define the step size for comparison
        let step_size = evs.len();
        //Organic iteration
        for (j, host) in vector.iter_mut().enumerate() {
            // Compare and update the elements in the larger vector
            // if eviscerator.values_are_greater(larger_value) {
            //     *larger_value = eviscerator.values.clone(); // Assuming your struct has a clone method
            let mut eviscerator:&mut Eviscerator = evs[j%step_size];
            if host.infected && host.zone == eviscerator.zone{
                eviscerator.infected = true;
                // println!("EVISCERATOR HAS BEEN INFECTED AT TIME {} of this chicken stock entering zone!",host.time);
                eviscerator.number_of_times_infected = 0;
                println!("{} {} {} {} {} {}",host.x,host.y,host.z,12,time,host.zone);
            }else if eviscerator.infected && host.zone == eviscerator.zone{
                // println!("Confirming that an eviscerator is infected in zone {}",eviscerator.zone);
                host.infected = host.transfer(limits::max(0.0,1.0-(eviscerator.number_of_times_infected as f64)*EVISCERATOR_TO_HOST_PROBABILITY_DECAY));
                if host.infected{host.number_of_times_infected+=1;}
                eviscerator.number_of_times_infected += 1;
                if host.infected{
                    println!("{} {} {} {} {} {}",host.x,host.y,host.z,11,time,host.zone);
                    // panic!("Evisceration has infected a host!!!");
                }
            }
            //Decay of infection
            if eviscerator.number_of_times_infected>=EVISCERATE_DECAY{
                eviscerator.infected = false;
            }
        }
    }    
}


#[derive(Clone)]
pub struct host{
    infected:bool,
    number_of_times_infected:u32,
    time_infected:f64,
    generation_time:f64,
    colonized:bool,
    motile:u8,
    zone:usize, //Possible zones denoted by ordinal number sequence
    prob1:f64,  //Probability of contracting disease - these are tied to zone if you create using .new() implementation within methods
    prob2:f64,  //standard deviation if required OR second probabiity value for transferring in case that is different from prob1
    x:f64,
    y:f64,
    z:f64, //can be 0 if there is no verticality
    perched:bool,
    eating:bool,
    eat_x:f64,
    eat_y:f64,
    eating_time:f64,
    age:f64,  //Age of host
    time:f64, //Time host has spent in facility - start from 0.0 from zone 0
    origin_x:u64,
    origin_y:u64,
    origin_z:u64,
    restrict:bool,  //Are the hosts free roaming or not?
    range_x:u64,  //"Internal" GRIDSIZE to simulate caged hosts in side the zone itself, not free roaming within facility ->Now to be taken from Segment
    range_y:u64,  //Same as above but for the y direction
    range_z:u64
}
//Note that if you want to adjust the number of zones, you have to, in addition to adjusting the individual values to your liking per zone, also need to change the slice types below!
//Parameterization options
const ACCELERATION:f64 = 0.4; //factor by which delta is decreased as we approach better fit value i.e. Bigger value -> More aggressive
const DECELERATION:f64 = 0.1; //factor that represents the percentage value of a delta change we take as rate of objective function decrease (trending towards ideal solution - decreases)
const PINPOINT:bool = false;
const PERCENTILE_MONITOR:f64 = 0.01; // 10th percentile values of all tests
const WEIGHT:f64 = 0.4; //percentage that this concert of best fit parameter estimates continue to influence the parameter estimate
// const WEIGHT_INITIATION_POINT:usize = 5; //When do we weight for the percentile parameters?How many 
//I.e. Smaller figure -> More punishing
//Resolution
const STEP:[[usize;3];1] = [[4,4,4]];  //Unit distance of segments ->Could be used to make homogeneous zoning (Might not be very flexible a modelling decision)
const HOUR_STEP: f64 = 4.0; //Number of times hosts move per hour
const LENGTH: usize =57*24; //How long do you want the simulation to be?
//Infection/Colonization module
// ------------Do only colonized hosts spread disease or do infected hosts spread
const HOST_0:f64 = 3.0;
const COLONIZATION_SPREAD_MODEL:bool = true;
const TIME_OR_CONTACT:bool = true; //true for time -> contact uses number of times infected to determine colonization
//If you want to use a truncated normal distribution for time to go from infected to colonized ...
const TIME_TO_COLONIZE:[f64;2] = [5.0*24.0, 11.6*24.0]; //95% CI for generation time
const COLONIZE_TIME_MAX_OVERRIDE:f64 = 26.0*24.0;
//If you want to use a gamma distribution
const ADJUSTED_TIME_TO_COLONIZE:[f64;2] = [4.5,1.0/2.0];  //In days, unlike hours. This is converted to hours within code
const PROBABILITY_OF_HORIZONTAL_TRANSMISSION:f64 = 0.05; //Chance of infected, but not yet colonized host, infecting their own deposits-> eggs
// const RATE_OF_COLONIZATION_DECAY: f64 = 0.17;
const FECAL_SHEDDING_PERIOD:f64 = 19.77*24.0;//Period in of which faeces from infected hosts will be infected, after which they will not be.
// const RECOVERY_RATE:[f64;2] = [0.00044264,0.00066390]; //Lower and upper range that increases with age
const RECOVERY_RATE:[f64;2] = [0.002,0.008]; //Lower and upper range that increases with age


const NO_TO_COLONIZE:u32 = 100;
//Infection rules
const HOSTTOHOST_CONTACT_SPREAD:bool = true; // Host -> Host, Host -> Faeces and Host -> Eggs via spatial proximity
const HOSTTOEGG_CONTACT_SPREAD:bool = false;
const HOSTTOFAECES_CONTACT_SPREAD:bool = false;
const EGGTOHOST_CONTACT_SPREAD:bool = false;
const FAECESTOHOST_CONTACT_SPREAD:bool = true;
const EGGTOFAECES_CONTACT_SPREAD:bool = true;
const FAECESTOEGG_CONTACT_SPREAD:bool = true;
// const INITIAL_COLONIZATION_RATE:f64 = 0.47; //Probability of infection, resulting in colonization -> DAILY RATE ie PER DAY
//Space
const LISTOFPROBABILITIES:[f64;1] = [0.15]; //Probability of transfer of samonella per zone - starting from zone 0 onwards
const GRIDSIZE:[[f64;3];1] = [[120.0,4.0,4.0]];
const MAX_MOVE:f64 = 1.0;
const MEAN_MOVE:f64 = 0.5;
const STD_MOVE:f64 = 1.0; // separate movements for Z config    
const MAX_MOVE_Z:f64 = 1.0;
const MEAN_MOVE_Z:f64 = 2.0;
const STD_MOVE_Z:f64 = 4.0;
const NO_OF_HOSTS_PER_SEGMENT:[u64;1] = [2];
//Anchor points
//Vertical perches
const PERCH:bool = false;
const PERCH_HEIGHT:f64 = 2.0; //Number to be smaller than segment range z -> Denotes frequency of heights at which hens can perch
const PERCH_FREQ:f64 = 0.15; //probability that hosts go to perch
const DEPERCH_FREQ:f64 = 0.4; //probability that a chicken when already on perch, decides to go down from perch
//Nesting areas
const NEST:bool = false;
const NESTING_AREA:f64 = 0.25; //ratio of the total area of segment in of which nesting area is designated - min x y z side
//Space --- Segment ID
const TRANSFERS_ONLY_WITHIN:[bool;1] = [false]; //Boolean that informs simulation to only allow transmissions to occur WITHIN segments, not between adjacent segments
//Fly option
const FLY:bool = false;
const FLY_FREQ:u8 = 3; //At which Hour step do the  
//Disease 
const TRANSFER_DISTANCE: f64 = 1.3;//maximum distance over which hosts can trasmit diseases to one another
const SIZE_FACTOR_FOR_EGGS:f64 = 0.15; //eggs are significantly smaller than their original hosts, so it stands to reason that their transfer distance for contact spread should be smaller
//Host parameters
const PROBABILITY_OF_INFECTION:f64 = 0.12; //probability of imported host being infected
const MEAN_AGE:f64 = 17.0*7.0*24.0; //Mean age of hosts imported (IN HOURS)
const STD_AGE:f64 = 3.0*24.0;//Standard deviation of host age (when using normal distribution)
const MAX_AGE:f64 = 20.0*7.0*24.0; //Maximum age of host accepted (Note: as of now, minimum age is 0.0)
const DEFECATION_RATE:f64 = 6.0; //Number times a day host is expected to defecate
const MIN_AGE:f64 = 1.0*24.0;

const DEPOSIT:bool = true;
const DEPOSIT_RATE_AFFECTED_BY_INFECTION:bool = true;
const DEPOSIT_RATE:f64 = 6.0/7.0; //Number of times a day host is expected to deposit a consumable deposit
const DEPOSIT_RATE_INFECTION_MULTIPLIER:f64 = 2.0/3.0;
//

//Feed parameters
const FEED_1:bool = false; //Do the hosts get fed - omnipotent method
const FEED_2:bool = false;//Do the hosts get fed - with standalone feeders ->crowding implication
const FEED_INFECTION_RATE:f64 = 0.003; //Probability of feed being infected
const FEED_ZONES:[usize;1] = [1]; //To set the zones that have feed provided to them.
const FEED_TIMES: [usize;2] = [11,14]; //24h format, when hosts get fed: Does not have to be only 2 - has no link to number of zones or anything like that
const FEEDER_SPACING:f64 = 2.5;
const FEED_DURATION:f64 = 0.5;
//Purge/Slaughter parameters
const SLAUGHTER_POINT:usize = 1; //Somewhere in zone {}, the hosts are slaughtered/killed and will cease to produce any eggs or faeces
//Evisceration parameters
const EVISCERATE:bool = false;
const EVISCERATE_ZONES:[usize;1] = [2]; //Zone in which evisceration takes place
const EVISCERATE_DECAY:u8 = 5;
const NO_OF_EVISCERATORS:[usize;1] = [6];
const EVISCERATOR_TO_HOST_PROBABILITY_DECAY:f64 = 0.25;   //Multiplicative decrease of  probability - starting from LISTOFPROBABILITIES value 100%->75% (if 0.25 is value)->50% ->25%->0%
//Evisceration -------------> Mishap/Explosion parameters
const MISHAP:bool = false;
const MISHAP_PROBABILITY:f64 = 0.01;
const MISHAP_RADIUS:f64 = 9.0; //Must be larger than the range_x of the eviscerate boxes for there to be any change in operation
//Transfer parameters
const ages:[f64;1] = [800.0*24.0]; //Time hosts are expected spend in each region minimally
//Collection
const AGE_OF_HOSTCOLLECTION: f64 = 20.0*24.0;  //For instance if you were collecting hosts every 15 days
const COLLECT_DEPOSITS: bool = true;
const AGE_OF_DEPOSITCOLLECTION:f64 = 1.0*24.0; //If you were collecting their eggs every 3 days
const FAECAL_CLEANUP_FREQUENCY:usize = 2; //How many times a day do you want faecal matter to be cleaned up?
//or do we do time collection instead?
const TIME_OF_COLLECTION :f64 = 200.0; //Time that the host has spent in the last zone from which you collect ONLY. NOT THE TOTAL TIME SPENT IN SIMULATION
//Influx? Do you want new hosts being fed into the scenario everytime the first zone exports some to the succeeding zones?
const INFLUX:bool = false;
const PERIOD_OF_INFLUX:u8 = 24; //How many hours before new batch of hosts are imported?
const PERIOD_OF_TRANSPORT:u8 = 1; //Prompt to transport hosts between zones every hour (checking that they fulfill ages requirement of course)
//Restriction?
const RESTRICTION:bool = true;
//Generation Parameters
const SPORADICITY:f64 = 4.0; //How many fractions of the dimension of the cage/segment do you want the hosts to start at? Bigger number makes the spread of hosts starting point more even per seg


//Additional 3D parameters
const FAECAL_DROP:bool = false; //Does faeces potentially drop in terms of depth?
const PROBABILITY_OF_FAECAL_DROP:f64 = 0.3;




impl host{
    fn recover(mut vector:&mut Vec<host>,rec0:f64,rec1:f64){
        vector.iter_mut().for_each(|mut x| {
            let grad:f64 = (rec1-rec0)/(MAX_AGE - MIN_AGE);
            let prob:f64 = rec0+(x.age-MIN_AGE) * grad;            
            if roll(prob){
                if x.infected && x.motile == 0 && !x.colonized{
                    x.infected = false;
                    x.colonized = false;
                    x.time_infected = 0.0;
                    x.number_of_times_infected = 0;
                    x.generation_time =gamma(ADJUSTED_TIME_TO_COLONIZE[0],ADJUSTED_TIME_TO_COLONIZE[1])*24.0;
                }                 
            }
        })
    }

    fn feed(mut vector:&mut Vec<host>, origin_x:u64,origin_y:u64,origin_z:u64, zone:usize,time:usize,feed_pd:f64){
        if FEED_1&&roll(feed_pd){
            // println!("Infected feed confirmed");
            vector.iter_mut().for_each(|mut h|{
                if h.motile == 0 && !h.infected && h.origin_x == origin_x && h.origin_y == origin_y && h.origin_z == origin_z && h.zone == zone{
                    h.infected = h.transfer(1.0);
                    println!("{} {} {} {} {} {}",h.x,h.y,h.z,10,time,h.zone); //10 is now an interaction type driven by the infected feed
                }
            })
        }else if FEED_2{
            //Identify locations of feeders based off of spacing provided
            // println!("HERE");
            let [range_x,range_y,_] = STEP[zone];
            let x_no = limits::max(range_x as f64/FEEDER_SPACING,2.0) as usize;
            let y_no = limits::max(2.0,range_y as f64/FEEDER_SPACING) as usize;
            let no:u64 = (x_no as u64 - 1)*(y_no as u64 - 1);
            vector.iter_mut().for_each(|mut h|{
                if h.motile == 0 && h.origin_x == origin_x && h.origin_y == origin_y && h.origin_z == origin_z && h.zone == zone{
                    h.eating = true;
                }
            });
            let total_to_feed:Vec<host> = vector.iter().filter(|&h| h.motile == 0 && h.origin_x == origin_x && h.origin_y == origin_y && h.origin_z == origin_z && h.zone == zone).cloned().collect();
            let total_to_feed:usize = total_to_feed.len();
            let per:usize = total_to_feed/(no as usize);
            // let counter:usize = 0;
            for x in 1..x_no{
                for y in 1..y_no{
                    //relative origin_location to segment frame of reference
                    let x_location = FEEDER_SPACING*(x as f64);
                    let y_location = FEEDER_SPACING*(y as f64);
                    // Determine how many elements to modify for the current iteration
                    let start_index:usize = (x - 1) * y_no + (y - 1);
                    // let end_index = start_index + per;                    
                    vector.iter_mut().filter(|h| h.motile == 0 && h.origin_x == origin_x && h.origin_y == origin_y && h.origin_z == origin_z && h.zone == zone).skip(start_index).take(per).for_each(|h| {
                        h.eat_x = x_location;
                        h.eat_y = y_location;
                    })
                }
            }
            if roll(feed_pd){
                vector.iter_mut().for_each(|mut h|{
                    if h.motile == 0 && !h.infected && h.origin_x == origin_x && h.origin_y == origin_y && h.origin_z == origin_z && h.zone == zone{
                        h.infected = h.transfer(1.0);
                        println!("{} {} {} {} {} {}",h.x,h.y,h.z,10,time,h.zone); //10 is now an interaction type driven by the infected feed
                    }
                })                       
            }
        }
    }    
    fn infect(mut vector:Vec<host>,loc_x:u64,loc_y:u64,loc_z:u64,zone:usize)->Vec<host>{
        if let Some(first_host) = vector.iter_mut().filter(|host_| host_.zone == zone).min_by_key(|host| {
            let dx = host.origin_x as i64 - loc_x as i64;
            let dy = host.origin_y as i64 - loc_y as i64;
            let dz = host.origin_z as i64 - loc_z as i64;
            (dx*dx + dy*dy+dz*dz) as u64
        }) 
        {if !first_host.infected{first_host.infected=true;}}
        vector
    }
    fn infect_multiple(mut vector:Vec<host>,loc_x:u64,loc_y:u64,loc_z:u64,n:usize,zone:usize, colonized:bool)->Vec<host>{ //homogeneous application ->Periodically apply across space provided,->Once per location
        let mut filtered_vector: Vec<&mut host> = vector.iter_mut().filter(|host| host.zone == zone).collect();

        filtered_vector.sort_by_key(|host| {
            let dx = host.origin_x as i64 - loc_x as i64;
            let dy = host.origin_y as i64 - loc_y as i64;
            let dz = host.origin_z as i64 - loc_z as i64;
            (dx*dx + dy*dy+dz*dz) as u64
        });
        for host in filtered_vector.iter_mut().take(n){
            host.infected = true;
            if colonized{host.colonized = true;}
            println!("{} {} {} {} {} {}",host.x,host.y,host.z,0,0.0,host.zone);
        }
        vector
    }
//     fn transport(mut vector:&mut Vec<host>,space:&mut Vec<Zone_3D>, influx: bool){ //Also to change ;size if you change number of zones
//         let mut output:Vec<host> = Vec::new();
//         for zone in (0..space.len()).rev(){
//             // let mut __:u32 = space.clone()[zone].capacity;
//             if &space[zone].capacity>&0 && zone>0{ //If succeeding zones (obviously zone 0 doesn't have do to this - that needs to be done with a replace mechanism)
//                 // let zone_toedit:&mut Zone_3D = &mut space[zone];
//                 vector.iter_mut().for_each(|mut x| {
//                     if x.zone == zone-1 && x.time>ages[zone-1]&& x.motile == 0 && space[zone].capacity>0{ //Hosts in previous zone that have spent enough time spent in previous zone
//                         //Find the first available segment
//                         // println!("Transporting...");
//                         // println!("Current szone")
//                         // __ -= 1;
//                         // println!("Capacity for zone {} is now:{} - pre subtraction",zone-1,space[zone-1].capacity);
//                         space[zone-1].subtract(x.origin_x.clone(),x.origin_y.clone(),x.origin_z.clone()); //move host from previous zone
//                         // println!("Capacity for zone {} is now:{} - post subtraction",zone-1,space[zone-1].capacity);
//                         // println!("---------------------------------------");
//                         // println!("{} capacity for zone {} vs {} for zone {}", &space[zone-1].capacity, zone-1,&space[zone].capacity,zone);
//                         x.zone += 1;
//                         // println!("Moved to zone {}",x.zone);
//                         x.time = 0.0;
//                         x.prob1 = LISTOFPROBABILITIES[x.zone];
//                         // println!("Going to deduct capacity @  zone {} with a capacity of {}", zone,zone_toedit.clone().zone);
//                         // println!("Apparently think that zone {} has {} space left",zone,space[zone].capacity);
//                         // println!("Capacity for zone {} is now:{} - pre addition",zone,space[zone].capacity);
//                         let vars:[u64;7] =  space[zone].add();
//                         // println!("Capacity for zone {} is now:{} - post addition",zone,space[zone].capacity);
//                         if vars[0] != 0{
//                             x.origin_x = vars[1];
//                             x.origin_y = vars[2];
//                             x.origin_z = vars[3];
//                             x.range_x = vars[4];
//                             x.range_y = vars[5];
//                             x.range_z = vars[6];

//                             //Maybe try moving the hosts randomly within each new section otherwise they all will infect each other at origin
//                             let mean_x:f64 = (x.origin_x as f64) + ((x.range_x as f64)/2.0) as f64;
//                             let std_x:f64 = ((x.range_x as f64)/SPORADICITY) as f64;
//                             // let max_x:f64 = x.range_x as f64;
//                             let mean_y:f64 = (x.origin_y as f64) + ((x.range_y as f64)/2.0) as f64;
//                             let std_y:f64 = ((x.range_y as f64)/SPORADICITY) as f64;
//                             // let max_y:f64 = x.range_y as f64;              
//                             //Baseline starting point in new region
//                             x.x = normal(mean_x,std_x,(x.origin_x+x.range_x) as f64);
//                             x.y = normal(mean_y,std_x,(x.origin_y+x.range_y) as f64);
//                             x.z = x.origin_z as f64;
//                         }
//                     }
//                 })
//             // output.append(&mut vector);
//             }
//             else if zone == 0 && space[zone].capacity>0 && influx{ //replace mechanism : influx is determined by INFLUX and PERIOD OF INLFUX 
//                 // let mut zone_0:&mut Zone = &mut space[0];
//                 for _ in 0..space[zone].clone().capacity as usize{
//                     // let [x,y]:[u64;2] = space[zone].add();
//                     //Roll probability
//                     let mut rng = thread_rng();
//                     let roll = Uniform::new(0.0, 1.0);
//                     let rollnumber: f64 = rng.sample(roll);
//                     let [condition,x,y,z,range_x,range_y,range_z] = space[0].add(); 
//                     if rollnumber<PROBABILITY_OF_INFECTION && condition != 0{
//                         vector.push(host::new_inf(0,0.2,x as f64,y as f64,z as f64,RESTRICTION,range_x,range_y,range_z));
//                     }
//                     else if condition != 0{
//                         vector.push(host::new(0,0.2,x as f64,y as f64,z as f64,RESTRICTION,range_x,range_y,range_z));
//                     }
//             }
//         }
//     }
// }



    fn transfer(&self, mult:f64)->bool{ //using prob1 as the probability of contracting disease  (in other words, no separation of events between transferring and capturing disease. If something is infected, it is always infected. Potentially.... the prospective new host will not get infected, but the INFECTED is always viably transferring)
        let mut rng = thread_rng();
        let roll = Uniform::new(0.0, 1.0);
        let rollnumber: f64 = rng.sample(roll);
        // println!("DISEASE   {}",rollnumber);
        rollnumber < self.prob1*mult
    }
    fn new(zone:usize, std:f64,loc_x:f64, loc_y:f64,loc_z:f64,restriction:bool,range_x:u64,range_y:u64,range_z:u64, atc0:f64, atc1:f64,probability:f64)->host{
        //We shall make it such that the chicken is spawned within the bottom left corner of each "restricted grid" - ie cage
        let prob:f64 = probability;
        //Add a random age generator
        host{infected:false,number_of_times_infected:0,time_infected:0.0,generation_time:gamma(atc0,atc1)*24.0,colonized:false,motile:0,zone:zone,prob1:prob,prob2:std,x:loc_x as f64,y:loc_y as f64,z:loc_z as f64,perched:false,eating:false,eat_x:0.0,eat_y:0.0,eating_time:0.0,age:normal_(MIN_AGE,MEAN_AGE,STD_AGE,MAX_AGE),time:0.0, origin_x:loc_x as u64,origin_y:loc_y as u64,origin_z: loc_z as u64,restrict:restriction,range_x:range_x,range_y:range_y,range_z:range_z}
    }
    fn new_inf(zone:usize, std:f64,loc_x:f64, loc_y:f64,loc_z:f64,restriction:bool,range_x:u64,range_y:u64,range_z:u64, atc0:f64, atc1:f64,probability:f64)->host{ //presumably a newly infected chicken that spreads disease is colonized
        let prob:f64 = probability;
        host{infected:true,number_of_times_infected:0,time_infected:0.0,generation_time:gamma(atc0,atc1)*24.0,colonized:true,motile:0,zone:zone,prob1:prob,prob2:std,x:loc_x as f64,y:loc_y as f64,z:loc_z as f64,perched:false,eating:false,eat_x:0.0,eat_y:0.0,eating_time:0.0,age:normal_(MIN_AGE,MEAN_AGE,STD_AGE,MAX_AGE),time:0.0, origin_x:loc_x as u64,origin_y:loc_y as u64,origin_z: loc_z as u64,restrict:restriction,range_x:range_x,range_y:range_y,range_z:range_z}
    }
    fn deposit(&mut self, consumable: bool,prob:f64)->host{ //Direct way to lay deposit from host. The function is 100% deterministic and layering a probability clause before this is typically expected
        let zone = self.zone.clone();
        let prob1:f64 = self.prob1.clone();
        let prob2:f64 = self.prob2.clone();
        let mut x:f64 = self.x.clone();
        let mut y:f64= self.y.clone();
        let mut z = self.origin_z.clone() as f64;
        if !RESTRICTION{ //If there are no containers holding the hosts (ie RESTRICTION), these hosts are keeping themselves above z = 0 by flying/floating etc, then deposits will necessary FALL to the floor ie z = 0
            z = 0.0;
        }
        // println!("Probability of infected drop now is {}",PROBABILITY_OF_HORIZONTAL_TRANSMISSION*(1.0-RATE_OF_COLONIZATION_DECAY).powf(self.time_infected));
        let mut inf = false;
        if !COLONIZATION_SPREAD_MODEL{
            inf = self.infected;
        }else if self.colonized{
            inf = true;
        }else if self.infected && self.time_infected<FECAL_SHEDDING_PERIOD && !consumable{
            inf = true;
        }else if self.infected && consumable{
            inf = roll(prob);
        }
        let range_y = self.range_y.clone();
        let range_x = self.range_x.clone();
        let range_z = self.range_z.clone();
        let restriction = self.restrict.clone();
        let origin_x = self.origin_x.clone();
        let origin_y = self.origin_y.clone();
        let origin_z = self.origin_z.clone();
        // println!("EGG BEING LAID");
        //Logic: If infected, immediately count as colonized for egg and faeces. Don't need to wait for it to be considered an infectant whichever colonization or non colonization model we use
        if consumable{
            //Nesting logic application
            if NEST{
                //egg location
                x = uniform(origin_x as f64, origin_x as f64 + NESTING_AREA*(range_x as f64));
                y = uniform(origin_y as f64, origin_y as f64 + NESTING_AREA*(range_y as f64));
                //chicken location
                self.perched = false;
                self.x = x.clone();
                self.y = y.clone();
            }
            host{infected:inf,number_of_times_infected:0,time_infected:0.0,generation_time:self.generation_time,colonized:inf,motile:1,zone:zone,prob1:prob1,prob2:prob2,x:x,y:y,z:z,perched:false,eating:self.eating,eat_x:self.eat_x,eat_y:self.eat_y,eating_time:self.eating_time,age:0.0,time:0.0,origin_x:x as u64,origin_y:y as u64,origin_z:z as u64,restrict:restriction,range_x:range_x,range_y:range_y,range_z:range_z}
            //Returning new egg host to host vector
        }
        else{//fecal shedding

            // println!("Pooping!");
            host{infected:inf,number_of_times_infected:0,time_infected:0.0,generation_time:self.generation_time,colonized:inf,motile:2,zone:zone,prob1:prob1,prob2:prob2,x:x,y:y,z:z,perched:false,eating:self.eating,eat_x:self.eat_x,eat_y:self.eat_y,eating_time:self.eating_time,age:0.0,time:0.0,origin_x:x as u64,origin_y:y as u64,origin_z:z as u64,restrict:restriction,range_x:range_x,range_y:range_y,range_z:range_z}
        }
    }
    fn deposit_all(vector:Vec<host>, probability:f64)->Vec<host>{
        //Below is an example whereby hosts deposit twice a day (fecal matter and laying eggs each once per day as an example)
        let mut vecc:Vec<host> = vector.clone();
        let mut vecc_into: Vec<host> = vector.clone().into_par_iter().filter(|x| x.motile==0 && x.zone<=SLAUGHTER_POINT).collect::<Vec<_>>(); //With this re are RETAINING the hosts and deposits within the original vector

        //.map wasn't working so we brute forced a loop
        for ele in vecc_into{
            if DEPOSIT{
                let mut rng = thread_rng();
                let mut deposit_rate:f64 = DEPOSIT_RATE;
                if DEPOSIT_RATE_AFFECTED_BY_INFECTION && ele.infected {deposit_rate*= DEPOSIT_RATE_INFECTION_MULTIPLIER;}                
                let v: &Vec<f64> = &Poisson::new(deposit_rate/24.0)
                .unwrap()
                .sample_iter(&mut rng)
                .take(1)
                .collect();            
                for _ in 0..v[0] as usize{
                    vecc.push(ele.clone().deposit(true, probability));//non consumable excrement once per day rate
                }
            }

            let mut rng = thread_rng();
            let v: &Vec<f64> = &Poisson::new(DEFECATION_RATE/24.0)
            .unwrap()
            .sample_iter(&mut rng)
            .take(1)
            .collect();            
            for _ in 0..v[0] as usize{
                vecc.push(ele.clone().deposit(false,probability));//non consumable excrement once per day rate
            }
        }
        vecc
    }
    fn land(vector:Vec<host>)->Vec<host>{
        vector.into_par_iter().filter_map(|mut x| {
            if RESTRICTION{
                x.z = x.origin_z as f64;
                Some(x)
            }else{
                x.z = 0.0;
                Some(x)
            }
        }).collect()
    }
    fn shuffle(mut self)->host{
        let mut eating_time:f64 = self.eating_time.clone();
        let mut eating:bool = self.eating.clone();
        if self.eating{
            if self.eating_time<FEED_DURATION{
                eating_time+=1.0/HOUR_STEP;
            }else{
                eating_time=0.0;
                eating = false;
            }
        }
        if self.infected{self.time_infected+=1.0/HOUR_STEP;}
        // let initial_colonization_rate_h:f64 = 1.0-(1.0-INITIAL_COLONIZATION_RATE).powf(1.0/(24.0*HOUR_STEP));
        if (TIME_OR_CONTACT && !self.colonized && self.infected)&& COLONIZATION_SPREAD_MODEL && self.motile == 0{
            if self.time_infected>self.generation_time{self.colonized = true;}
        }else if COLONIZATION_SPREAD_MODEL && !TIME_OR_CONTACT && self.number_of_times_infected>NO_TO_COLONIZE && self.infected && self.motile == 0{
            self.colonized = true;
        }
        if self.motile==0 && EVISCERATE_ZONES.contains(&self.zone) == false{ // NOT IN EVISCERATION
            //Whether the movement is negative or positive
            let mut mult:[f64;3] = [0.0,0.0,0.0];
            for index in 0..mult.len(){
                if roll(0.33){
                    if roll(0.5){
                        mult[index] = 1.0;
                    }else{
                        mult[index] = -1.0;
                    }
                }
            }

            let mut new_x:f64 = self.x.clone() as f64;
            let mut new_y:f64 = self.y.clone() as f64;
            let mut new_z:f64 = self.z.clone() as f64;
            //use truncated normal distribution (which has been forced to be normal) in order to change the values of x and y accordingly of the host - ie movement
            if self.restrict{
                // println!("We are in the restrict clause! {}", self.motile);
                // println!("Current shuffling parameter is {}", self.motile);
                if !self.eating{
                    new_x = limits::min(limits::max(self.origin_x as f64,self.x+mult[0]*normal(MEAN_MOVE,STD_MOVE,MAX_MOVE)),self.origin_x as f64+self.range_x as f64);
                }else{ // conservative movement pattern of the 2 chosen for the normal distribution when the hosts have gathered at the various nodes to feed.
                    self.perched = false;
                    new_x = limits::min(limits::max(self.origin_x as f64,self.eat_x+mult[0]*normal(limits::min(MEAN_MOVE,FEEDER_SPACING/2.0),limits::min(FEEDER_SPACING/2.0,STD_MOVE),FEEDER_SPACING)),self.origin_x as f64+self.range_x as f64);
                }
                if !self.perched{
                    if !self.eating{
                        new_y = limits::min(limits::max(self.origin_y as f64,self.y+mult[1]*normal(MEAN_MOVE,STD_MOVE,MAX_MOVE)),self.origin_y as f64+self.range_y as f64);
                    }else{
                        new_y = limits::min(limits::max(self.origin_y as f64,self.eat_y+mult[1]*normal(limits::min(MEAN_MOVE,FEEDER_SPACING/2.0),limits::min(FEEDER_SPACING/2.0,STD_MOVE),FEEDER_SPACING)),self.origin_y as f64+self.range_y as f64);
                    }
                }
                if FLY{
                    new_z = limits::min(limits::max(self.origin_z as f64,self.z+mult[2]*normal(MEAN_MOVE_Z,STD_MOVE_Z,MAX_MOVE_Z)),self.origin_z as f64+self.range_z as f64);
                }else if PERCH && roll(PERCH_FREQ){ //no need perching concept for flying creatures
                    new_z = limits::min(self.z+PERCH_HEIGHT, self.origin_z as f64+self.range_z as f64);
                    self.perched = true;
                }else if PERCH && roll(DEPERCH_FREQ){
                    new_z = self.origin_z as f64;
                    self.perched = false;
                }
            }else{
                if !self.eating{
                    new_x = limits::min(limits::max(0.0,self.x+mult[0]*normal(MEAN_MOVE,STD_MOVE,MAX_MOVE)),GRIDSIZE[self.zone as usize][0]);
                }else{
                    self.perched = false;
                    new_x = limits::min(limits::max(0.0,self.eat_x+mult[0]*normal(limits::min(MEAN_MOVE,FEEDER_SPACING/2.0),limits::min(FEEDER_SPACING/2.0,STD_MOVE),FEEDER_SPACING)),GRIDSIZE[self.zone as usize][0]);
                }
                if !self.perched{
                    if !self.eating{
                        new_y = limits::min(limits::max(0.0,self.y+mult[1]*normal(MEAN_MOVE,STD_MOVE,MAX_MOVE)),GRIDSIZE[self.zone as usize][1]);
                    }else{
                        new_y = limits::min(limits::max(0.0,self.eat_y+mult[1]*normal(limits::min(MEAN_MOVE,FEEDER_SPACING/2.0),limits::min(FEEDER_SPACING/2.0,STD_MOVE),FEEDER_SPACING)),GRIDSIZE[self.zone as usize][1]);
                    }
                }
                if FLY{
                    new_z = limits::min(limits::max(0.0,self.z+mult[2]*normal(MEAN_MOVE_Z,STD_MOVE_Z,MAX_MOVE_Z)),GRIDSIZE[self.zone as usize][2]);
                }else if PERCH && roll(PERCH_FREQ){ //no need perching concept for flying creatures
                    new_z = limits::min(self.z+PERCH_HEIGHT, self.origin_z as f64+self.range_z as f64);
                    self.perched = true;
                }else if PERCH && roll(DEPERCH_FREQ){
                    new_z = self.origin_z as f64;
                    self.perched = false;
                }
            }            
            host{infected:self.infected,number_of_times_infected:0,time_infected:self.time_infected,generation_time:self.generation_time,colonized:self.colonized,motile:self.motile,zone:self.zone,prob1:self.prob1,prob2:self.prob2,x:new_x,y:new_y,z:self.z,perched:self.perched,eating:eating,eat_x:self.eat_x,eat_y:self.eat_y,eating_time:eating_time,age:self.age+1.0/HOUR_STEP,time:self.time+1.0/HOUR_STEP,origin_x:self.origin_x,origin_y:self.origin_y,origin_z:self.origin_z,restrict:self.restrict,range_x:self.range_x,range_y:self.range_y,range_z:self.range_z}
        }else if self.motile==0 && EVISCERATE_ZONES.contains(&self.zone){
            // println!("Evisceration pending...");
            // self.motile == 1; //It should be presumably electrocuted and hung on a conveyer belt
            self.x = ((self.origin_x as f64) + (self.range_x as f64))/2.0; // square in middle
            self.y = ((self.origin_y as f64) + (self.range_y as f64))/2.0;
            self.z = (self.origin_z as f64) + (self.range_z as f64); //Place host on the top of the box to simulate suspension on the top
            // println!("Placing host @ {},{},{}",self.x,self.y,self.z);.shuffle
            self.age += 1.0/HOUR_STEP;
            self.time += 1.0/HOUR_STEP;
            self
        }
        else if self.restrict{
            //deposits by hosts do not move obviously, but they DO age, which affects collection
            self.age += 1.0/HOUR_STEP;
            self.time += 1.0/HOUR_STEP;
            if FAECAL_DROP && self.motile == 2 && self.z>0.0{
                // println!("Examining poop for shuttle drop!");
                self.z -= (poisson(PROBABILITY_OF_FAECAL_DROP/HOUR_STEP)*(STEP[self.zone][2] as u64)) as f64;
                self.z = limits::max(self.z,0.0);
            }
            self
        }
        else{
            if self.z!=0.0{self.z = 0.0;}
            self.age+= 1.0/HOUR_STEP;
            self.time+=1.0/HOUR_STEP;
            self
        }
    }
    fn shuffle_all(vector: Vec<host>)->Vec<host>{
        vector.into_par_iter().map(|x| x.shuffle()).collect()
    }
    fn dist(host1: &host, host2: &host)->bool{
        let diff_x: f64 = host1.x -host2.x;
        let diff_y: f64 = host1.y - host2.y;
        let diff_z: f64 = host1.z - host2.z;
        let t: f64 = diff_x.powf(2.0)+diff_y.powf(2.0) + diff_z.powf(2.0);
        /////
        //PRINT STATEMENT
        // if t.powf(0.5)<=TRANSFER_DISTANCE{
        //     println!("{} {} vs {} {}",&host1.x,&host1.y,&host2.x,&host2.y);
        // }
        ////
        let mut transfer_distance:f64 = TRANSFER_DISTANCE;
        if host1.motile==2{
            if host2.motile ==2{
                transfer_distance *= SIZE_FACTOR_FOR_EGGS;
                // if t.powf(0.5)<transfer_distance{println!("Egg to egg infection! @ {:?} vs {:?}, with infection status:{} vs {} respectively",[host1.x,host1.y,host1.z],[host2.x,host2.y,host2.z],host1.infected,host2.infected);}
            }else{
                transfer_distance = TRANSFER_DISTANCE/2.0 + TRANSFER_DISTANCE*SIZE_FACTOR_FOR_EGGS;
            }
        }else if host2.motile == 2{
            transfer_distance = TRANSFER_DISTANCE/2.0 + TRANSFER_DISTANCE*SIZE_FACTOR_FOR_EGGS;
        }
        t.powf(0.5)<=transfer_distance && host1.zone == host2.zone
    }
    // fn transmit(mut inventory:Vec<host>,time:usize)->Vec<host>{//Current version logic: Once the diseased host passes the "test" in fn transfer, then ALL other hosts within distance contract
    //     //Locate all infected hosts
    //     let mut cloneof: Vec<host> = inventory.clone();
    //     cloneof = cloneof.into_iter().filter_map(|mut x|{
    //         if x.infected{ //x.transfer is how we internalise the probabilistic nature (not definitive way) that a disease can or cannot spread from an infected individual
    //             Some(x)
    //         }else{
    //             None
    //         }
    //     }).collect();
    //     inventory = inventory.into_iter().filter(|x| !x.infected).collect::<Vec<host>>();    
    //     inventory = inventory.into_iter().filter_map(|mut x|{
    //         if cloneof.iter().any(|inf| host::dist(&inf,&x) && inf.zone == x.zone){
    //             let before = x.infected.clone();
    //             x.infected=x.transfer();
    //             if !before && x.infected{
    //                 if x.x!=0.0 && x.y != 0.0{println!("{} {} {} {} {}",x.x,x.y,x.z,time,x.zone);}
    //             }
    //             // println!("{} vs {}",&inf.x,&x.x,&inf.y,&x.y);
    //             Some(x)
    //         }else{
    //             Some(x)
    //         }
    //     }).collect();
    //     inventory.extend(cloneof);
    //     inventory
    // }
    fn transmit(mut inventory: Vec<host>, time: usize) -> Vec<host> {
        // Locate all infected/colonized hosts
        let mut cloneof: Vec<host> = inventory.clone();
        //Infectors
        cloneof = cloneof
            .into_par_iter()
            .filter_map(|mut x| {
                if (x.infected && !COLONIZATION_SPREAD_MODEL) || (COLONIZATION_SPREAD_MODEL && x.colonized){
                    Some(x)
                } else {
                    None
                }
            })
            .collect();
        // println!("Length of infectors is {}",cloneof.len());
        //to be infected
        if COLONIZATION_SPREAD_MODEL{
            inventory = inventory.into_par_iter().filter(|x| (!x.colonized && x.motile == 0) || (!x.infected && x.motile != 0) ).collect::<Vec<host>>();
        }else{
            inventory = inventory.into_par_iter().filter(|x| !x.infected).collect::<Vec<host>>(); //potentially to save bandwidth, let us remove the concept of colonization in eggs and faeces -> don't need to log colonization in faeces especially!
        }
        //Infection process - both elements concerned here
        inventory = inventory
            .into_par_iter()
            .filter_map(|mut x| {
                for inf in &cloneof {
                    let segment_boundary_condition:bool = !TRANSFERS_ONLY_WITHIN[x.zone] || TRANSFERS_ONLY_WITHIN[x.zone] && x.origin_x == inf.origin_x && x.origin_y == inf.origin_y && x.origin_z == inf.origin_z;
                    let hosttohost_contact_rules:bool = (HOSTTOHOST_CONTACT_SPREAD || !HOSTTOHOST_CONTACT_SPREAD && !(inf.motile == 0 && x.motile ==0));
                    let hosttoegg_contact_rules:bool = (HOSTTOEGG_CONTACT_SPREAD || (!HOSTTOEGG_CONTACT_SPREAD && !(inf.motile == 0 && x.motile == 1)));
                    let hosttofaeces_contact_rules:bool = (HOSTTOFAECES_CONTACT_SPREAD || !HOSTTOFAECES_CONTACT_SPREAD &&!(inf.motile == 0 && x.motile == 2));
                    let eggtohost_contact_rules:bool = (EGGTOHOST_CONTACT_SPREAD || !EGGTOHOST_CONTACT_SPREAD && !(inf.motile == 1 && x.motile == 0));
                    let faecestohost_contact_rules:bool = (FAECESTOHOST_CONTACT_SPREAD || !FAECESTOHOST_CONTACT_SPREAD &&!(inf.motile == 2 && x.motile == 0));
                    let eggtofaeces_contact_rules:bool = (EGGTOFAECES_CONTACT_SPREAD || !EGGTOFAECES_CONTACT_SPREAD && !(inf.motile ==  1 && x.motile == 2));
                    let faecestoegg_contact_rules:bool = (FAECESTOEGG_CONTACT_SPREAD || !FAECESTOEGG_CONTACT_SPREAD && !(inf.motile ==  2 && x.motile == 1));
                    let contact_rules:bool = hosttohost_contact_rules && hosttoegg_contact_rules && hosttofaeces_contact_rules && eggtohost_contact_rules && faecestohost_contact_rules && eggtofaeces_contact_rules && faecestoegg_contact_rules;
                    if host::dist(inf, &x) && inf.zone == x.zone && segment_boundary_condition && contact_rules && !x.infected{ //do not allow for multiple infections to happen at the same time on one host
                        let before = x.infected.clone();
                        x.infected = x.transfer(1.0);
                        if x.infected{x.number_of_times_infected+=1;}
                        if !before && x.infected {
                            if x.x != 0.0 && x.y != 0.0 {
                                let mut diagnostic:i8 = 1;
                                if x.motile>inf.motile{
                                    diagnostic = -1;
                                }
                                // Access properties of 'inf' here
                                println!(
                                    "{} {} {} {} {} {}",
                                    x.x,
                                    x.y,
                                    x.z,
                                    diagnostic*((x.motile+1) as i8) * ((inf.motile+1) as i8), 
                                    time,
                                    x.zone
                                );
                                // if x.zone == 2 && diagnostic*((x.motile+1) as i8) * ((inf.motile+1) as i8)==1{
                                //     println!("INAPPROPRIATE INTERACTION @ zone 2 Delta x : {},Delta y : {},Delta z : {} -> Segments between 2 hosts are the same? : {}",x.x-inf.x,x.y-inf.y,x.z-inf.z,x.origin_x == inf.origin_x&&x.origin_y == inf.origin_y&&x.origin_z == inf.origin_z);
                                //     panic!();
                                // }
                            }
                        }
                    }
                }
                Some(x)
            })
            .collect();
        inventory.extend(cloneof);
        inventory
    }
    
    fn cleanup(inventory:Vec<host>)->(Vec<host>,Vec<host>){
        inventory.into_par_iter().partition(|x| x.motile==2 || (!COLLECT_DEPOSITS && x.motile == 1))
    }
    fn collect(inventory:Vec<host>)->[Vec<host>;2]{   //hosts and deposits potentially get collected
        let mut collection:Vec<host> = Vec::new();
        let vec1:Vec<host> = inventory.into_iter().filter_map(|mut x| {
            // println!("Chicken in zone {}",x.zone);
            if x.motile==0 && x.age>AGE_OF_HOSTCOLLECTION && x.zone == GRIDSIZE.len()-1{
                // println!("Collecting host(s)...{} days old",x.age/24.0);
                collection.push(x);
                None
            }else if x.motile == 1 && x.age>AGE_OF_DEPOSITCOLLECTION{
                // println!("Collecting deposit(s)...");
                collection.push(x);
                None
            }else{
                Some(x)
            }
        }).collect();
        [vec1,collection]  //collection vector here to be added and pushed into the original collection vector from the start of the loop! This function merely outputs what should be ADDED to collection!
    }
    fn collect__(inventory:Vec<host>,zone:&mut Zone_3D)->[Vec<host>;2]{   //hosts and deposits potentially get collected
        let mut collection:Vec<host> = Vec::new();
        let vec1:Vec<host> = inventory.into_iter().filter_map(|mut x| {
            // println!("Chicken in zone {}",x.zone);
            // println!("GRIDSIZE - 1 is {} ",GRIDSIZE.len()-1);
            if x.motile==0 && x.time>ages[ages.len()-1] && x.zone == GRIDSIZE.len()-1{
                // println!("Collecting host(s)...{} days old",x.age/24.0);
                zone.subtract(x.origin_x,x.origin_y,x.origin_z);
                collection.push(x);
                // *capacity+=1;
                None
            }else if x.motile == 1 && x.age>AGE_OF_DEPOSITCOLLECTION && COLLECT_DEPOSITS{
                // println!("Collecting deposit(s)...");
                collection.push(x);
                // *capacity+=1;
                None
            }else{
                Some(x)
            }
        }).collect();
        [vec1,collection]  //collection vector here to be added and pushed into the original collection vector from the start of the loop! This function merely outputs what should be ADDED to collection!
    }    
    fn collect_and_replace(inventory:Vec<host>)->[Vec<host>;2]{   //same as collect but HOSTS GET REPLACED (with a Poisson rate of choosing) - note that this imports hosts, doesn't transfer from earlier zone
        let mut collection:Vec<host> = Vec::new();
        let vec1:Vec<host> = inventory.into_iter().filter_map(|mut x| {
            if x.motile==0 && x.age>AGE_OF_HOSTCOLLECTION&& x.zone == GRIDSIZE.len()-1{
                // println!("Collecting host(s)...{} days old",x.age/24.0);
                collection.push(x.clone());
                // None
                let mut rng = thread_rng();
                let roll = Uniform::new(0.0, 1.0);
                let rollnumber: f64 = rng.sample(roll);
                if rollnumber<PROBABILITY_OF_INFECTION{
                    Some(host{infected:true,age:normal_(MIN_AGE,MEAN_AGE,STD_AGE,MAX_AGE),time:0.0,..x})
                }else{
                    Some(host{infected:false,age:normal_(MIN_AGE,MEAN_AGE,STD_AGE,MAX_AGE),time:0.0,..x})
                }
            }else if x.motile == 1 && x.age>AGE_OF_DEPOSITCOLLECTION{
                // println!("Collecting deposit(s)...");
                collection.push(x);
                None
            }else{
                Some(x)
            }
        }).collect();
        [vec1,collection]  //collection vector here to be added and pushed into the original collection vector from the start of the loop! This function merely outputs what should be ADDED to collection!
    }
    fn report(inventory:&Vec<host>)->[f64;4]{ //simple function to quickly return the percentage of infected hosts
        let inf: f64 = inventory.clone().into_iter().filter(|x| {
            x.infected && x.motile==0
        }).collect::<Vec<_>>().len() as f64;
        let noofhosts: f64 = inventory.clone().into_iter().filter(|x| {
            x.motile==0
        }).collect::<Vec<_>>().len() as f64;

        let inf2: f64 = inventory.clone().into_iter().filter(|x| {
            x.infected && x.motile==1
        }).collect::<Vec<_>>().len() as f64;
        let noofhosts2: f64 = inventory.clone().into_iter().filter(|x| {
            x.motile==1
        }).collect::<Vec<_>>().len() as f64;        

        [inf/(noofhosts+1.0),inf2/(noofhosts2+1.0),noofhosts,noofhosts2]
    }
    fn zone_report(inventory:&Vec<host>,zone:usize)->[f64;7]{ //simple function to quickly return the percentage of infected hosts
        //Filter for zone
        let mut inventory:Vec<host> = inventory.clone().into_iter().filter(|x|{
            x.zone == zone
        }).collect::<Vec<_>>();
        //Mobile hosts infected calculation
        let inf: f64 = inventory.clone().into_iter().filter(|x| {
            x.infected && x.motile==0
        }).collect::<Vec<_>>().len() as f64;
        let noofhosts: f64 = inventory.clone().into_iter().filter(|x| {
            x.motile==0
        }).collect::<Vec<_>>().len() as f64;
        //Consumable immobiles infected calculation
        let inf2: f64 = inventory.clone().into_iter().filter(|x| {
            x.infected && x.motile==1
        }).collect::<Vec<_>>().len() as f64;
        let noofhosts2: f64 = inventory.clone().into_iter().filter(|x| {
            x.motile==1
        }).collect::<Vec<_>>().len() as f64;        

        //Colonization
        let inf3: f64 = inventory.clone().into_iter().filter(|x| {
            x.colonized && x.motile==0
        }).collect::<Vec<_>>().len() as f64;

        //Faeces
        let inf4: f64 = inventory.clone().into_iter().filter(|x| {
            x.infected && x.motile==2
        }).collect::<Vec<_>>().len() as f64;
        let noofhosts4: f64 = inventory.clone().into_iter().filter(|x| {
            x.motile==2
        }).collect::<Vec<_>>().len() as f64;              

        [inf/(noofhosts+1.0),inf2/(noofhosts2+1.0),noofhosts,noofhosts2,inf3/(noofhosts+1.0), inf4/(noofhosts4+1.0),noofhosts4]
    }    
    fn generate_in_grid(zone:&mut Zone_3D,hosts:&mut Vec<host>,atc0:f64, atc1:f64,probability:f64){  //Fill up each segment completely to full capacity in a zone with hosts. Also update the capacity to reflect that there is no more space
        let zone_no:usize = zone.clone().zone;
        zone.segments.iter_mut().for_each(|mut x| {
            let mean_x:f64 = ((x.range_x as f64)/2.0) as f64;
            let std_x:f64 = ((x.range_x as f64)/SPORADICITY) as f64;
            let max_x:f64 = x.range_x as f64;
            let mean_y:f64 = ((x.range_y as f64)/2.0) as f64;
            let std_y:f64 = ((x.range_y as f64)/SPORADICITY) as f64;
            let max_y:f64 = x.range_y as f64;            
            for _ in 0..x.capacity.clone() as usize{hosts.push(host::new(zone_no,0.2,x.origin_x as f64 + normal(mean_x,std_x,max_x),(x.origin_y as f64 +normal(mean_y,std_y,max_y)) as f64,x.origin_z as f64,RESTRICTION,x.range_x,x.range_y,x.range_z,atc0, atc1,probability));}
            x.capacity = 0;
            zone.capacity = 0;
        });
    }
}


fn test(parameters:[f64;8],fit_to:Vec<(usize,f64)>)->f64{
    let mut hosts: Vec<host> = Vec::new();
    let mut faecal_inventory: Vec<host> = Vec::new();
    let mut hosts_in_collection:[u64;2] = [0,1];
    let mut colonials_in_collection:[u64;2] = [0,1];
    let mut deposits_in_collection:[u64;2] = [0,1];
    let mut faecal_collection:[u64;2] = [0,1];
    let mut zones:Vec<Zone_3D> = Vec::new();
    //Set a mega vector of the parameters that you wish to change the values of 
    //Changing parameter values 
    //parameters vector is to contain the following parameters in order : [ADJUSTED COLONIZATION TIME 0,ADJUSTED COLONIZATION TIME 1,Deposit probability (horizontal), recovery rate 0, recovery rate 1, probability of disease transmission (contact),feed infection probability ]

    // let parameters:[f64,f64,f64,f64,f64,f64,f64] = [adjusted_time_to_colonize_0,adjusted_time_to_colonize_1,prob_horiz_transmission,recovery_rate_0,recovery_rate_1,pd,feed_pd];
    // let mut [adjusted_time_to_colonize_0,adjusted_time_to_colonize_1,prob_horiz_transmission,recovery_rate_0,recovery_rate_1,pd,feed_pd]: [f64;7] = [ADJUSTED_TIME_TO_COLONIZE[0],ADJUSTED_TIME_TO_COLONIZE[1],PROBABILITY_OF_HORIZONTAL_TRANSMISSION,RECOVERY_RATE[0],RECOVERY_RATE[1],LISTOFPROBABILITIES[0],FEED_INFECTION_RATE];
    //Influx parameter
    let mut influx:bool = false;
    //Generate eviscerators
    let mut eviscerators:Vec<Eviscerator> = Vec::new();
    if EVISCERATE{
        for index in 0..EVISCERATE_ZONES.len(){
            for _ in 0..NO_OF_EVISCERATORS[index]{
                eviscerators.push(Eviscerator{zone:EVISCERATE_ZONES[index],infected:false,number_of_times_infected:0})
            }
        }
    }
    
    //Initialise with hosts in the first zone only
    for grid in 0..GRIDSIZE.len(){
        zones.push(Zone_3D::generate_empty(grid,[GRIDSIZE[grid][0] as u64,GRIDSIZE[grid][1] as u64,GRIDSIZE[grid][2] as u64],STEP[grid]));
    }

    // println!("Here are the capacities for each of the zones:{},{},{}",zones[0].capacity, zones[1].capacity,zones[2].capacity);

    // host::generate_in_grid(&mut zones[0],&mut hosts, parameters[0],parameters[1],parameters[5]);
    // println!("{:?}", hosts.len());
    // for thing in hosts.clone(){
    //     println!("Located at zone {} in {} {}: MOTION PARAMS: {} for {} and {} for {}",thing.zone,thing.x,thing.y,thing.origin_x,thing.range_x,thing.origin_y,thing.range_y);
    // }
    //GENERATE INFECTED HOST
    // hosts.push(host::new_inf(1,0.2,(GRIDSIZE[0] as u64)/2,(GRIDSIZE[1] as u64)/2),true,STEP as u64,STEP as u64); // the infected
    // hosts = host::infect(hosts,400,400,0);
    // hosts = host::infect(hosts,800,800,0);
    // hosts = host::infect(hosts,130,40,0);
    // hosts = host::infect(hosts,10,10,0);
    // hosts = host::infect(hosts,300,1800,0);

    //MORE EFFICIENT WAY TO INFECT MORE hosts - insize zone 0
    let zone_to_infect:usize = 0;
    // hosts = host::infect_multiple(hosts,GRIDSIZE[zone_to_infect][0] as u64/2,GRIDSIZE[zone_to_infect][1] as u64/2,GRIDSIZE[zone_to_infect][2] as u64/2,parameters[7] as usize,0, true);
    for segment in &mut zones[0].segments {
        println!("Segment has coordinates {} {} {}", segment.origin_x, segment.origin_y, segment.origin_z);
        
        if segment.origin_x % 8 == 0 {
            segment.generate(false, false, 1, parameters[0], parameters[1], parameters[5], &mut hosts);
            segment.generate(true, false, 1, parameters[0], parameters[1], parameters[5], &mut hosts);
        }
            // if segment.origin_x % 16 == 0 {
            //     segment.generate(true,false,1,parameters[0],parameters[1],parameters[5],&mut hosts);
            // }
    }
    


    //Count number of infected
    // let it: u64 = hosts.clone().into_iter().filter(|x| x.infected).collect()::<Vec<_>>.len();
    // let mut vecc_into: Vec<host> = hosts.clone().into_iter().filter(|x| x.infected).collect::<Vec<_>>(); //With this re are RETAINING the hosts and deposits within the original vector
    // println!("NUMBER OF INFECTED hosts IS {}", vecc_into.len());
    //CSV FILE
    let filestring: String = format!("./output.csv");
    if fs::metadata(&filestring).is_ok() {
        fs::remove_file(&filestring).unwrap();
    }
    // Open the file in append mode for writing
    let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .append(true) // Open in append mode
    .open(&filestring)
    .unwrap();
    let mut wtr = Writer::from_writer(file);
    let mut counter:f64 = 0.0;
    for time in 0..LENGTH{

        let mut collect: Vec<host> = Vec::new();
        if time % (24/FAECAL_CLEANUP_FREQUENCY) ==0{
            (faecal_inventory,hosts) = host::cleanup(hosts);
        }
        // println!("{} CHECK {}",time%(PERIOD_OF_TRANSPORT  as usize),time%(PERIOD_OF_TRANSPORT  as usize) == 0);
        // if time%(PERIOD_OF_TRANSPORT  as usize)==0{
        //     // println!("Fulfilling period of transport right now");
        //     host::transport(&mut hosts,&mut zones,influx);
        //     // println!("Total number of hosts is {}: Total number of faeces is {}",  hosts.clone().into_iter().filter(|x| x.motile == 0).collect::<Vec<_>>().len() as u64,hosts.clone().into_iter().filter(|x| x.motile == 2).collect::<Vec<_>>().len() as u64)
        // }        


        for times in FEED_TIMES{
            if time%24 ==times && time>0 && (FEED_1 || FEED_2){
                for spaces in FEED_ZONES{
                    hosts = zones[spaces].clone().feed_setup(hosts,time.clone(),parameters[6]);
                }
            }
        }
        if EVISCERATE{
            for zone in EVISCERATE_ZONES{
                // println!("Evisceration occurring at zone {}",zone);
                zones[zone].eviscerate(&mut eviscerators,&mut hosts,time.clone());
            }
        }
        let mut FinalZone:&mut Zone_3D = &mut zones[GRIDSIZE.len()-1];
        [hosts,collect] = host::collect__(hosts,&mut FinalZone);

        for unit in 0..HOUR_STEP as usize{
            // println!("Number of poop is {}",hosts.clone().into_iter().filter(|x| x.motile == 2).collect::<Vec<_>>().len() as u64);
            hosts = host::shuffle_all(hosts);
            hosts = host::transmit(hosts,time.clone());
            if FLY && unit != 0 && (unit % FLY_FREQ as usize) == 0{
                hosts = host::land(hosts);
            }
        } //Say hosts move/don't move every 15min - 4 times per hour
        host::recover(&mut hosts,parameters[3],parameters[4]);
        hosts = host::deposit_all(hosts, parameters[2]);
        //Collect the hosts and deposits as according
        // println!("Number of infected eggs in soon to be collection is {}",collect.clone().into_iter().filter(|x| x.motile == 1 && x.infected).collect::<Vec<_>>().len() as f64);
        // feast.append(&mut collect);
        //Update Collection numbers
        let no_of_infected_hosts: u64 = collect.clone().into_par_iter().filter(|x| x.motile == 0 && x.infected).collect::<Vec<_>>().len() as u64;
        let no_of_colonized_hosts:u64 = collect.clone().into_par_iter().filter(|x| x.motile == 0 && x.colonized).collect::<Vec<_>>().len() as u64;
        let no_of_hosts: u64 = collect.clone().into_par_iter().filter(|x| x.motile == 0).collect::<Vec<_>>().len() as u64;
        let no_of_deposits: u64 = collect.clone().into_par_iter().filter(|x| x.motile == 1).collect::<Vec<_>>().len() as u64;
        let no_of_infected_deposits: u64 = collect.clone().into_par_iter().filter(|x| x.motile == 1 && x.infected).collect::<Vec<_>>().len() as u64;
        let no_of_faeces: u64 = faecal_inventory.clone().into_par_iter().filter(|x| x.motile == 2).collect::<Vec<_>>().len() as u64;
        let no_of_infected_faeces:u64 = faecal_inventory.clone().into_par_iter().filter(|x| x.motile == 2 && x.infected).collect::<Vec<_>>().len() as u64;

        hosts_in_collection[0] += no_of_infected_hosts;
        colonials_in_collection[0] += no_of_colonized_hosts;
        hosts_in_collection[1] += no_of_hosts;
        colonials_in_collection[1] += no_of_hosts;
        deposits_in_collection[0] += no_of_infected_deposits;
        deposits_in_collection[1] += no_of_deposits;
        faecal_collection[0] += no_of_infected_faeces;
        faecal_collection[1] += no_of_faeces;

        if INFLUX && time%PERIOD_OF_INFLUX as usize==0 && time>0{
            influx = true;
            // println!("Influx just got changed to true");
        }else{
            influx = false;
        }
        // let mut count:u8 = 0;
        // for i in zones.clone(){
        //     println!("{} : {}", count,i.capacity);
        //     count+=1;
        // }
        //Farm
        let no_of_zones:usize = GRIDSIZE.len();
        let collection_zone_no:u8 = no_of_zones as u8+1;
        //Call once

        let [mut perc,mut perc2,mut total_hosts,mut total_hosts2,mut perc3,mut perc4,mut total_hosts4] = host::zone_report(&hosts,0);            
        let no = perc.clone()*total_hosts;      
        perc = perc*100.0;
        let no2 = perc2.clone()*total_hosts2;        
        perc2 = perc2*100.0;
        let no3 = perc3.clone()*total_hosts;
        perc3 *= 100.0;
        let no4 = perc4.clone()*total_hosts4;
        perc4 = perc4*100.0;            
        wtr.write_record(&[
            perc.to_string(),
            total_hosts.to_string(),
            no.to_string(),
            perc2.to_string(),
            total_hosts2.to_string(),
            no2.to_string(),
            perc3.to_string(),
            no3.to_string(),
            perc4.to_string(),
            total_hosts4.to_string(),
            no4.to_string(),
            format!("Zone {}", 0),
        ]);
 
        //Collection
        // let [mut _perc,mut _perc2,mut _total_hosts,mut _total_hosts2] = host::report(&feast);
        let _no = hosts_in_collection[0];
        let _perc = (hosts_in_collection[0] as f64)/(hosts_in_collection[1] as f64) * 100.0;
        let _no2 = deposits_in_collection[0];
        let _perc2 = (deposits_in_collection[0] as f64)/(deposits_in_collection[1] as f64)*100.0;
        let _total_hosts = hosts_in_collection[1];
        let _total_hosts2 = deposits_in_collection[1];
        let _no3 = colonials_in_collection[0];
        let _perc3 = (colonials_in_collection[0] as f64)/(colonials_in_collection[1] as f64) * 100.0;
        let _no4 = faecal_collection[0];
        let _perc4 = (faecal_collection[0] as f64)/(faecal_collection[1] as f64)*100.0;
        let _total_faeces = faecal_collection[1];
        // println!("{} {} {} {} {} {}",perc,total_hosts,no,perc2,total_hosts2,no2);    
        // println!("{} {} {} {} {} {} {} {} {} {} {} {}",perc,total_hosts,no,perc2,total_hosts2,no2,_perc,_total_hosts,_no,_perc2,_total_hosts2,_no2);
        wtr.write_record(&[
            _perc.to_string(),
            _total_hosts.to_string(),
            _no.to_string(),
            _perc2.to_string(),
            _total_hosts2.to_string(),
            _no2.to_string(),
            _perc3.to_string(),            
            _no3.to_string(),
            _perc4.to_string(),
            _total_faeces.to_string(),
            _no4.to_string(),
            "Collection Zone".to_string(),
        ])
        .unwrap();
        for ele in fit_to.clone(){
            // println!("Length of the fit_to vector is {}", fit_to.len());
            // println!("Current time result we are trying to compare is {}",ele.0);
            if ele.0 == time{
                counter+=(ele.1 - perc3).powf(2.0);
                println!("Error report for {}: Trying to fit to {}, achieved {} instead @ time {}",parameters[5],ele.1,perc,ele.0 );
            }
        }           

        // if host::report(&hosts)[2]<5.0{break;}
    }
    wtr.flush().unwrap();
    // println!("{} {} {} {} {} {}",STEP[0][0],STEP[0][1],STEP[0][2],LENGTH,GRIDSIZE.len(), TRANSFER_DISTANCE); //Last 5 lines are going to be zone config lines that need to be picked out in plotter.py
    for zone in 0..GRIDSIZE.len(){
        println!("{} {} {} {} {} {}",GRIDSIZE[zone][0],GRIDSIZE[zone][1],GRIDSIZE[zone][2],1000,0,zone);
    }
    for zone in 0..GRIDSIZE.len(){
        // println!("{} {} {} {} {} {}",GRIDSIZE[zone][0],GRIDSIZE[zone][1],GRIDSIZE[zone][2],1000,0,zone);
        println!("{} {} {} {} {} {}",STEP[zone][0]+100000,STEP[zone][1]+100000,STEP[zone][2]+100000,GRIDSIZE[zone][0]+100000.0,GRIDSIZE[zone][1]+100000.0,GRIDSIZE[zone][2]+100000.0);  //Paramters for R file to extract and plot
    }          
    (counter/(fit_to.len() as f64).powf(0.5))
    // println!("{} {} {} {} {} {}",GRIDSIZE[0][0],GRIDSIZE[0][1],GRIDSIZE[0][2],0,0,0); //Last 5 lines are going to be zone config lines that need to be picked out in plotter.py
}





fn parameterize(ind:Vec<usize>,epoch:usize, rollover:usize,values_and_deltas:Vec<[f64;3]>,fit_to:Vec<(usize,f64)>)->Vec<[f64;9]>{
    //The input vector of values and deltas is meant to consist ofslices strictly of size 3 that designate the range of the values that we want to change between, and the delta step with
    //which we wish to intiailly step

    //We will consider a deceleration and acceleration mechanic shortly
    let mut output:Vec<[f64;9]> = Vec::new(); //All parameter/factor values, terminating with the associated MSE value
    //First consider that we only wish to allow one or more out of the 8 available variables to be changed.
    //Extract public values
    let mut ini:[f64;8] = [ADJUSTED_TIME_TO_COLONIZE[0],ADJUSTED_TIME_TO_COLONIZE[1],PROBABILITY_OF_HORIZONTAL_TRANSMISSION,RECOVERY_RATE[0],RECOVERY_RATE[1],LISTOFPROBABILITIES[0],FEED_INFECTION_RATE,HOST_0];
    //Change out the values we wish to change with the average of the values presented inside rollover for the indices indicated by ind
    let mut tt:usize = 0;
    for ele in ind.clone(){
        ini[ele] = values_and_deltas[tt][0]+values_and_deltas[tt][1];
        ini[ele] /= 2.0;
        tt+=1;
    }
    let mut prev_ini:[f64;8] = ini.clone();
    //MSE objective scores
    let mut MSE_previous:f64 = test(ini,fit_to.clone());
    let mut MSE:f64 = test(ini,fit_to.clone());
    let MSE_0:f64 = (MSE_previous+MSE+test(ini,fit_to.clone()))/3.0; //take the average of 3 readings as a baseline to compare to
    //MSE deltas
    let mut delta:f64 =(MSE-MSE_previous).abs();
    let mut delta_previous:f64 = 1.0;
    let mut delta_ratio:f64 = delta/delta_previous;
    let mut delta_ratio_previous:f64 = 1.0;
    let mut direction:bool = MSE<MSE_previous; // intial check if previous is less than
    
    //Weighing down algorithm
    let mut are_we_close:bool = false;
    for run in 0..epoch{
        let mut anchors:Vec<[f64;9]> = Vec::new();
        //How many times have you approached/diverged from the goal?
        let mut reward_counter:f64 = 1.0;
        let mut direction_polarity:f64 = 1.0;
        let mut tt:usize = 0;
        //Multiplier to maintain correct direction towards optimal solution
        for var_index in ind.clone(){
            println!("Variables {:?}",ini.clone());
            // println!("Here is a sample test value {}", test(ini, fit_to.clone()));
            MSE_previous = MSE.clone();
            let mut ini2:[f64;8] = ini.clone();
            ini2[var_index] = values_and_deltas[tt][1] - ini2[var_index];
            MSE = test(ini,fit_to.clone());
            //Mirror test
            if run%(epoch.clone()/10)==0{
                if test(ini2,fit_to.clone()) <MSE{ini[var_index] = ini2[var_index];}
            }
            delta_previous = delta.clone();
            delta = MSE - MSE_previous; //if negative, we are approaching solution -> we do not need to consider abs() here because MSE is innately positive
            delta_ratio_previous = delta_ratio.clone();
            delta_ratio = (delta/delta_previous).abs();
            println!("MSE previous {} vs MSE {} at epoch {} - fitted factor value is {}", MSE_previous,MSE,run.clone(),ini[var_index]);
            let mut unit:[f64;9] = [0.0;9];
            for thing in 0..ini.len(){
                unit[thing]+=ini[thing];
            }
            unit[8] += MSE;
            output.push(unit);
            if MSE<1.0 && PINPOINT{break;}
            if MSE<MSE_previous{
                prev_ini = ini.clone();
                //in otherwords, if we are moving in the direction that we want... -> maintain polarity and just add delta at least
                if direction{
                    //and the prior iteration was ALSO in the right direction,AND there has been an increase in the decrease in our discrepancy
                    if delta_ratio>=1.0{reward_counter*=(1.0+ACCELERATION);}
                    else{
                        //decelerate
                        reward_counter *= DECELERATION;
                    }
                }
                if MSE/MSE_0<0.05{
                    reward_counter *= MSE/MSE_0;
                }
                // if MSE/MSE_0<PERCENTILE_MONITOR{
                //     if !are_we_close{are_we_close = true;}
                //     anchors.push([ini[var_index],ini[ini.len()-1]]);
                // }
            }else if MSE == MSE_previous && ini[var_index]>=values_and_deltas[tt][0] && ini[var_index]<=values_and_deltas[tt][1]{
                println!("We reached some potential minima... Investigating");
                direction_polarity*=-1.0;
                reward_counter*=(1.0+ACCELERATION);
            }else{// i.e. MSE has increased now!
                if delta_previous<0.0{
                    //Here, it means we were approaching solution previously!!
                    //Reorientate to previous solution
                    ini = prev_ini.clone();
                    //
                    reward_counter = limits::min(DECELERATION*DECELERATION,MSE/MSE_0);
                }else{
                    reward_counter = limits::max(1.0+ACCELERATION,MSE/MSE_0);
                    direction_polarity *= -1.0;
                }
            }
            if direction_polarity>0.0 && ini[var_index]==values_and_deltas[tt][1] || direction_polarity<0.0 && ini[var_index] == values_and_deltas[tt][0]{
                direction_polarity*=-1.0;
            }
            ini[var_index] += direction_polarity*reward_counter*values_and_deltas[tt][2];
            ini[var_index] = limits::max(values_and_deltas[tt][0],ini[var_index]);
            ini[var_index] = limits::min(values_and_deltas[tt][1],ini[var_index]);
            //Weight
            if output.len()>5{
                // anchors:Vec<[f64;9]> = output.clone().sort_by(|a,b| a[8].partial_cmp(&b[8]).unwrap());
                // anchors = anchors.truncate((PERCENTILE_MONITOR*output.len() as f64) as usize);
                let percentile = calculate_percentile(&output,PERCENTILE_MONITOR);
                anchors = output.clone().into_iter().filter(|x| x[output[0].len()-1]<=limits::min(percentile,MSE)).collect();
                // anchors.retain(|x| x[8] < percentile);
                println!("Percentile value is {}",percentile);
                println!("Anchors vector is {:?}",anchors.clone());
                if anchors.len()>1 && run>epoch.clone()/2{
                    // ini[var_index] *= (1.0-WEIGHT);
                    let sum__:f64 = anchors.iter().map(|s| s[var_index]).sum();
                    ini[var_index] += WEIGHT*sum__/(anchors.len() as f64);   
                    ini[var_index] /= (1.0+WEIGHT);         
                }
            }
            direction = MSE<MSE_previous && delta_previous>= 0.0;
            tt+=1;
        }
    }
    let perc:f64 = calculate_percentile(&output,PERCENTILE_MONITOR);
    let mut anchors:Vec<[f64;9]> = output.clone().into_iter().filter(|x| x[output[0].len()-1]<=perc).collect();

    for index in ind{
        let mut min_max_first: (f64, f64) = (f64::MAX, f64::MIN);
        for slice in &anchors {
            // Calculate min and max for the first element (index 0) of each slice
            let first_element:f64 = slice[index];
            min_max_first.0 = min_max_first.0.min(first_element);
            min_max_first.1 = min_max_first.1.max(first_element);
        }    
        println!("Range of Factor {} for {} percentile values is {:?}", index,PERCENTILE_MONITOR,min_max_first);
    }


    output
}

fn calculate_percentile(data: &Vec<[f64; 9]>, percentile: f64) -> f64 {
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a[8].partial_cmp(&b[8]).unwrap());
    let index = (percentile* (sorted_data.len() as f64 - 1.0)) as usize;
    sorted_data[index][8]
}

fn main(){
    let ind:Vec<usize> = vec![3,4,5];
    let epochs:usize = 500;
    //Changing parameter values 
    //parameters vector is to contain the following parameters in order : [ADJUSTED COLONIZATION TIME 0,ADJUSTED COLONIZATION TIME 1,Deposit probability (horizontal), recovery rate 0, recovery rate 1, probability of disease transmission (contact),feed infection probability ]
    let parameters:[f64;8] = [ADJUSTED_TIME_TO_COLONIZE[0],ADJUSTED_TIME_TO_COLONIZE[1],PROBABILITY_OF_HORIZONTAL_TRANSMISSION,RECOVERY_RATE[0],RECOVERY_RATE[1],LISTOFPROBABILITIES[0],FEED_INFECTION_RATE,HOST_0];
    // let delta:Vec<[f64;9]> = parameterize(ind.clone(),epochs,1,vec![[0.1,0.9,0.1]],vec![(168,50.0),(336,48.9),(504,42.5),(672,52.5),(840,60.0),(1008,70.0),(1176,70.0),(1344,70.0)]); //percentage values MUST be in percentage NOT actual <1 numbers -> reason: using MSE
    let delta:Vec<[f64;9]> = parameterize(ind.clone(),epochs,1,vec![[0.001,0.007,0.0002],[0.007,0.01,0.003],[0.1,0.9,0.1]],vec![(168,17.5),(336,35.0),(504,45.0),(672,55.0),(840,62.5),(1008,72.5),(1176,72.5),(1344,72.5)]); //percentage values MUST be in percentage NOT actual <1 numbers -> reason: using MSE
    println!("------------------------------------------------------------------------------------------");
    // println!("Optimized variable value is now operation is {} versus the original {}, with a final MSE score of {}",delta[delta.len()-1][6], (0.95+0.1)/2.0,delta[delta.len()-1][8]);

    // Extract the last value of each slice as the y-values
    let y_values: Vec<f64> = delta.iter().map(|&slice| slice[8]).collect();
    let hypertext:Vec<String> = delta.iter().map(|&slice| format!("Generation Time Gamma Shape:{} <br> Generation Time Gamma Rate:{} <br> Deposit Contamination Probability:{} <br> Recovery Rate Gamma Shape:{} <br> Recovery Rate Gamma Rate:{} <br> PROBABILITY OF INFECTION:{} <br> Feed Infection rate:{} <br> Number of infected Host 0s:{}",slice[0],slice[1],slice[2],slice[3],slice[4],slice[5],slice[6],slice[7])).collect();
    let min_y = y_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let minimum_line:Vec<f64> = vec![min_y;delta.len()];
    println!("min_y vector is {:?}",min_y);
    // Create a scatterplot
    // let x_values:Vec<f64> = delta.iter().map(|&slice| slice[ind[0]]).collect();
    let scatter = Scatter::new((0..delta.len()).collect(),y_values.clone())
        .mode(Mode::Lines)
        .marker(Marker::new().color(Rgba::new(0, 91, 75, 0.35)))
        .text_array(hypertext.clone())
        .name("Scatter Plot");
    let scatter_addon = Scatter::new((0..delta.len()).collect(),y_values.clone())
        .mode(Mode::Markers)
        .marker(Marker::new().color(Rgba::new(0, 118, 97, 0.7)))
        .text_array(hypertext.clone())
        .name("Scatter Plot");        

    let mut plot = Plot::new();
    plot.add_trace(scatter);
    plot.add_trace(scatter_addon);

    let trace1 = Scatter::new((0..delta.len()).collect(), minimum_line)
    .mode(Mode::Lines)
    .name("Minima")
    .line(plotly::common::Line::new().color("purple").width(0.8 as f64).dash(plotly::common::DashType::LongDashDot));
    plot.add_trace(trace1);
    // Customize the layout
    let layout = Layout::new().x_axis(Axis::new().title(Title::new("Epochs"))).y_axis(Axis::new().title(Title::new("MSE discrepancy"))).height(860).width(1860).plot_background_color(Rgba::new(255,246,235,0.7)).paper_background_color(Rgba::new(255,246,235,0.8));
    plot.set_layout(layout);

    // Show the plot (generates an HTML file)
    plot.show();
    plot.write_html("Epoch_Plot.html");

    for ele in ind.clone(){
        let x_values:Vec<f64> = delta.iter().map(|&slice| slice[ele]).collect();
        let scatter = Scatter::new(x_values,y_values.clone())
        .mode(Mode::Markers)
        .marker(Marker::new().color(Rgba::new(0, 91, 75, 0.35)))
        .text_array(hypertext.clone())
        .name("Scatter Plot");
        let mut plot2 = Plot::new();
        plot2.add_trace(scatter);     
        let layout = Layout::new().x_axis(Axis::new().title(Title::new(&(format!("Factor {}",ele))))).y_axis(Axis::new().title(Title::new("MSE discrepancy"))).height(860).width(1860).plot_background_color(Rgba::new(255,246,235,0.7)).paper_background_color(Rgba::new(255,246,235,0.8));
        plot2.set_layout(layout);
        plot2.show();
        plot2.write_html(format!("Factor_{}.html",ele));
    }

    if ind.len()>=2{
        let filestring: String = format!("./output.csv");
        if fs::metadata(&filestring).is_ok() {
            fs::remove_file(&filestring).unwrap();
        }
        // Open the file in append mode for writing
        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true) // Open in append mode
        .open(&filestring)
        .unwrap();
        let mut wtr = Writer::from_writer(file);
        for row in delta.iter() {
            // Convert the f64 values to strings
            let row_as_strings: Vec<String> = row.iter().map(|&f| f.to_string()).collect();
            wtr.write_record(&row_as_strings).unwrap();
        }
        wtr.flush().unwrap();

    }



}

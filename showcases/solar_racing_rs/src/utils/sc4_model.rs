use std::fmt;

pub struct Car {
    // Car Parameters
    name: String,
    v_lon: f64, // m/s -- Longitudinal Velocity of the Solar Car
    v_lat: f64, // m/s -- Lateral Velocity of the Solar Car

    // Measurements
    area: f64, // m^2 -- Frontal area of Solar Car
    track: f64, // m -- Distance between outrigger wheel and occupant cell wheels
    length: f64, // m -- Solar Car's chord length
    width: f64, // m -- Solar Car's width (widest part of the car)

    // Mass Parameters
    mass: f64, // kg -- Car Mass including driver
    cog_h: f64, // m -- Distance from CoG to ground
    rotational_inertial: f64, // kgm^2 -- Polar moment of inertia around the CoG

    // Traction Parameters
    traction_f: f64, // [-] -- Traction coefficient front
    traction_r: f64, // [-] -- Traction coefficient rear

    // Aerodynamic values
    drag_force: f64, // N
    lift_force: f64, // N
    side_force: f64, // N
    roll_moment: f64, // Nm
    yaw_moment: f64, // Nm
    pitch_moment: f64, // Nm

    // Magic Numbers
    B: f64, // For the magic formula
    C: f64, // For the magic formula
    D: f64, // For the magic formula
    E: f64, // For the magic formula
}

pub struct OffsetCat {
    // Car Parameters
    name: String,
    v_lon: f64, // m/s -- Longitudinal Velocity of the Solar Car
    v_lat: f64, // m/s -- Lateral Velocity of the Solar Car

    // Measurements
    area: f64, // m^2 -- Frontal area of Solar Car
    track: f64, // m -- Distance between outrigger wheel and occupant cell wheels
    length: f64, // m -- Solar Car's chord length
    width: f64, // m -- Solar Car's width (widest part of the car)

    // Mass Parameters
    mass: f64, // kg -- Car Mass including driver
    cog_f: f64, // m -- Distance from CoG to front axle
    cog_m: f64, // m -- Distance from CoG to outrigger axle
    cog_r: f64, // m -- Distance from CoG to rear axle
    cog_h: f64, // m -- Distance from CoG to ground
    rotational_inertial: f64, // kgm^2 -- Polar moment of inertia around the CoG

    // Traction Parameters
    traction_f: f64, // [-] -- Traction coefficient front
    traction_r: f64, // [-] -- Traction coefficient rear

    // Aerodynamic coefficients
    drag_coefficient: f64, // N -- Drag force coefficient
    lift_coefficient: f64, // N -- Lift force coefficient
    sideforce_coefficient: f64, // N -- Side force coefficient
    roll_coefficient: f64, // Nm -- Roll moment coefficient
    yaw_coefficient: f64, // Nm -- Yaw moment coefficient
    pitch_coefficient: f64, // Nm -- Pitch moment coefficient

    // Aerodynamic values
    drag_force: f64, // N
    lift_force: f64, // N
    side_force: f64, // N
    roll_moment: f64, // Nm
    yaw_moment: f64, // Nm
    pitch_moment: f64, // Nm

    // Magic Numbers
    B: f64, // For the magic formula
    C: f64, // For the magic formula
    D: f64, // For the magic formula
    E: f64, // For the magic formula

    // Front, middle, and rear axle distances from CoG as proportion of overall
    // track length (front and rear will add to 1)
    cog_f_prop: f64, // [-]
    cog_m_prop: f64, // [-]
    cog_r_prop: f64, // [-]
}

impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "====================================\n\
                    Car Parameters:\n\
                    \t-name: {}\n\
                    \t-v_lon: {} m/s\n\
                    \t-v_lat: {} m/s\n\
                    \n\
                    Measurements:\n\
                    \t-area: {} m^2\n\
                    \t-track: {} m\n\
                    \t-length: {} m\n\
                    \t-width: {} m\n\
                    \n\
                    Mass Parameters:\n\
                    \t-mass: {} kg\n\
                    \t-cog_h: {} m\n\
                    \t-rotational_inertial: {} kgm^2\n\
                    \n\
                    Traction Parameters:\n\
                    \t-traction_f: {}\n\
                    \t-traction_r: {}\n\
                    \n\
                    Aerodynamic values:\n\
                    \t-drag_force: {} N\n\
                    \t-lift_force: {} N\n\
                    \t-side_force: {} N\n\
                    \t-roll_moment: {} Nm\n\
                    \t-yaw_moment: {} Nm\n\
                    \t-pitch_moment: {} Nm\n\
                    \n\
                    Magic Numbers:\n\
                    \t-B: {}\n\
                    \t-C: {}\n\
                    \t-D: {}\n\
                    \t-E: {}",
               self.name, self.v_lon, self.v_lat,
               self.area, self.track, self.length, self.width,
               self.mass, self.cog_h, self.rotational_inertial,
               self.traction_f, self.traction_r,
               self.drag_force, self.lift_force, self.side_force,
               self.roll_moment, self.yaw_moment, self.pitch_moment,
               self.B, self.C, self.D, self.E)
    }
}

impl fmt::Display for OffsetCat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "====================================\n\
                    Car Parameters:\n\
                    \t-name: {}\n\
                    \t-v_lon: {} m/s\n\
                    \t-v_lat: {} m/s\n\
                    \n\
                    Measurements:\n\
                    \t-area: {} m^2\n\
                    \t-track: {} m\n\
                    \t-length: {} m\n\
                    \t-width: {} m\n\
                    \n\
                    Mass Parameters:\n\
                    \t-mass: {} kg\n\
                    \t-cog_f: {} m\n\
                    \t-cog_m: {} m\n\
                    \t-cog_r: {} m\n\
                    \t-cog_h: {} m\n\
                    \t-rotational_inertial: {} kgm^2\n\
                    \n\
                    Traction Parameters:\n\
                    \t-traction_f: {}\n\
                    \t-traction_r: {}\n\
                    \n\
                    Aerodynamic coefficients:\n\
                    \t-drag_coefficient: {} N\n\
                    \t-lift_coefficient: {} N\n\
                    \t-sideforce_coefficient: {} N\n\
                    \t-roll_coefficient: {} Nm\n\
                    \t-yaw_coefficient: {} Nm\n\
                    \t-pitch_coefficient: {} Nm\n\
                    \n\
                    Aerodynamic values:\n\
                    \t-drag_force: {} N\n\
                    \t-lift_force: {} N\n\
                    \t-side_force: {} N\n\
                    \t-roll_moment: {} Nm\n\
                    \t-yaw_moment: {} Nm\n\
                    \t-pitch_moment: {} Nm\n\
                    \n\
                    Magic Numbers:\n\
                    \t-B: {}\n\
                    \t-C: {}\n\
                    \t-D: {}\n\
                    \t-E: {}\n\
                    \n\
                    Axle Distances from CoG (Proportions):\n\
                    \t-cog_f_prop: {}\n\
                    \t-cog_m_prop: {}\n\
                    \t-cog_r_prop: {}",
               self.name, self.v_lon, self.v_lat,
               self.area, self.track, self.length, self.width,
               self.mass, self.cog_f, self.cog_m, self.cog_r, self.cog_h, self.rotational_inertial,
               self.traction_f, self.traction_r,
               self.drag_coefficient, self.lift_coefficient, self.sideforce_coefficient,
               self.roll_coefficient, self.yaw_coefficient, self.pitch_coefficient,
               self.drag_force, self.lift_force, self.side_force,
               self.roll_moment, self.yaw_moment, self.pitch_moment,
               self.B,
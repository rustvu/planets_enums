#[derive(Debug, Clone)]
enum CelestialBody {
    // Unit variant: no data
    BlackHole,
    // Tuple variant: unnamed fields
    Asteroid(f64, f64), // diameter (m), mass (kg)
    // Struct variant: named fields
    Planet {
        name: String,
        radius: f64,
        mass: f64,
    },
}

impl CelestialBody {
    // Associated constant
    const G: f64 = 6.67e-11;

    // Method to compute surface gravity (if applicable)
    fn surface_gravity(&self) -> Option<f64> {
        match self {
            CelestialBody::Planet { radius, mass, .. } => Some(Self::G * mass / (radius * radius)),
            CelestialBody::Asteroid(diameter, mass) => {
                let radius = diameter / 2.0;
                Some(Self::G * mass / (radius * radius))
            }
            CelestialBody::BlackHole => None, // Not defined for black holes here
        }
    }

    // Method to display a greeting
    fn greet(&self) {
        match self {
            CelestialBody::Planet { name, .. } => println!("Hello, planet {name}!"),
            CelestialBody::Asteroid(_, _) => println!("Watch out for that asteroid!"),
            CelestialBody::BlackHole => println!("Don't get too close to the black hole!"),
        }
    }
}

fn main() {
    let earth = CelestialBody::Planet {
        name: "Earth".to_string(),
        radius: 6.378e6,
        mass: 5.972e24,
    };
    let ceres = CelestialBody::Asteroid(9.46e5, 9.393e20);
    let singularity = CelestialBody::BlackHole;

    for body in [&earth, &ceres, &singularity] {
        println!("{:?}", body);
        body.greet();
        match body.surface_gravity() {
            Some(g) => println!("Surface gravity: {g}"),
            None => println!("Surface gravity: undefined"),
        }
        println!();
    }

    if let CelestialBody::Planet { name, radius, .. } = earth {
        println!("{} has a diameter of {} meters", name, 2.0 * radius);
    }
}

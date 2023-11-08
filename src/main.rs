#[derive(Debug, Clone)]
struct Planet {
    name: String,
    radius: f64, // meters
    mass: f64,   // kg
}

impl Planet {
    fn new(name: &str, radius: f64, mass: f64) -> Self {
        Self {
            name: name.to_string(),
            radius,
            mass,
        }
    }

    fn surface_gravity(&self) -> f64 {
        6.67e-11 * self.mass / (self.radius * self.radius)
    }

    fn ring_diameter(&self) -> Option<f64> {
        if self.name == "Saturn" {
            Some(2.0 * self.radius)
        } else {
            None
        }
    }

    fn shrink(&mut self, scale: f64) {
        self.radius *= scale;
    }

    fn annihilate(self) {
        println!("Good bye, {}!", self.name);
    }
}

enum Body {
    Planet(Planet),
    Star,
    Comet { name: String, period: f64 },
}

impl Body {
    fn name(&self) -> &str {
        match self {
            Body::Planet(p) => &p.name,
            Body::Star => "Star",
            Body::Comet { name, .. } => name,
        }
    }
}

enum PlanetList {
    Cons(Planet, Box<PlanetList>),
    Nil,
}

impl PlanetList {
    fn new() -> Self {
        PlanetList::Nil
    }

    fn prepend(self, planet: Planet) -> Self {
        PlanetList::Cons(planet, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            PlanetList::Cons(_, tail) => 1 + tail.len(),
            PlanetList::Nil => 0,
        }
    }
}

fn main() {
    let earth = Planet::new("Earth", 6.378e6, 5.972e24);

    let mut gaia = earth.clone();
    gaia.shrink(0.5);
    println!("{:?}", gaia);
    gaia.annihilate();

    println!("{:?}, surface gravity: {}", earth, earth.surface_gravity());

    //
    let saturn = Planet::new("Saturn", 6.378e6, 5.972e24);
    println!("Earth's ring diameter: {:?}", earth.ring_diameter());
    println!("Saturn's ring diameter: {:?}", saturn.ring_diameter());

    //
    let sun = Body::Star;
    let comet = Body::Comet {
        name: "Halley".to_string(),
        period: 76.0,
    };
    let planet = Body::Planet(Planet::new("Earth", 6.378e6, 5.972e24));
    println!(
        "sun: {}, comet: {}, planet: {}",
        sun.name(),
        comet.name(),
        planet.name()
    );

    //
    let mut planets = PlanetList::new();
    planets = planets.prepend(Planet::new("Mercury", 2.439e6, 3.301e23));
    planets = planets.prepend(Planet::new("Venus", 6.052e6, 4.867e24));
    planets = planets.prepend(Planet::new("Earth", 6.378e6, 5.972e24));
    planets = planets.prepend(Planet::new("Mars", 3.393e6, 6.417e23));
    println!("number of planets: {}", planets.len());
}

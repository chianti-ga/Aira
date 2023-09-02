use std::fmt::{Display, Formatter};

pub const CBOX_DEFAULT: &str = "0 0 0 0 0 0";

#[derive(Clone)]
pub struct PropsData {
    pub modelname: String,
    pub body: String,
    pub surfaceprop: String,
    pub contents: String,
    pub cdmaterials: String,
    pub sequence: String,
    pub texturegroup: Vec<String>,
    pub cbox: String,
    pub collisionmodel: CollisionModel,
}

#[derive(Clone)]
pub struct CollisionModel {
    pub modelname: String,
    pub automass: bool,
    pub mass: String,
    pub concave: bool,
}

impl Default for CollisionModel {
    fn default() -> CollisionModel {
        CollisionModel {
            modelname: "".to_string(),
            automass: true,
            mass: "0.0".to_string(),
            concave: true,
        }
    }
}

impl Display for PropsData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "$modelname \"{}\"", self.modelname.replace("smd", "mdl"))?;

        writeln!(f, "$body {} \"{}\"", self.modelname.replace(".smd", ""), self.body)?;

        writeln!(f, "$surfaceprop \"{}\"", self.surfaceprop)?;

        writeln!(f, "$contents \"{}\"", self.contents)?;

        writeln!(f, "$cdmaterials \"{}\"", self.cdmaterials)?;

        writeln!(f, "$sequence idle \"{}\"", self.sequence)?;

        write!(f, "$texturegroup skins\n{{")?;

        self.texturegroup.iter().for_each(|texture| {
            write!(f, " {{ {} }}", texture).unwrap();
        });
        writeln!(f, "}}")?;
        writeln!(f, "$cbox {}", self.cbox)?;
        writeln!(f, "{}", self.collisionmodel)
    }
}

impl Display for CollisionModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "$collisionmodel \"{}\" {{\n ", self.modelname)?;
        if self.automass {
            writeln!(f, "$automass")?;
        } else {
            writeln!(f, "$mass {}", self.mass)?;
        }
        if self.concave {
            writeln!(f, "$concave")?;
        }
        write!(f, "}}")
    }
}

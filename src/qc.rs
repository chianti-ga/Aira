use std::fmt::{Display, Formatter};

const CBOX_DEFAULT: [f32; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

pub struct PropsData {
    pub modelname: String,
    pub body: String,
    pub surfaceprop: String,
    pub contents: String,
    pub cdmaterials: String,
    pub texturegroup: Vec<String>,
    pub cbox: [f32; 6],
    pub collisionmodel: CollisionModel,
}

pub struct CollisionModel {
    pub automass: bool,
    pub mass: f32,
    pub concave: bool,
}

impl Default for PropsData {
    fn default() -> PropsData {
        PropsData {
            modelname: "".to_string(),
            body: "".to_string(),
            surfaceprop: "default".to_string(),
            contents: "solid".to_string(),
            cdmaterials: "".to_string(),
            texturegroup: Vec::new(),
            cbox: CBOX_DEFAULT,
            collisionmodel: CollisionModel {
                automass: true,
                mass: 0.0,
                concave: true,
            },
        }
    }
}

impl Display for PropsData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\"$modelname\" \"{}\"", self.modelname)?;

        writeln!(f, "\"$body\" \"{}\"", self.body)?;

        writeln!(f, "\"$surfaceprop\" \"{}\"", self.surfaceprop)?;

        writeln!(f, "\"$contents\" \"{}\"", self.contents)?;

        write!(f, "\"$texturegroup\" \"skins\"\n{{")?;

        self.texturegroup.iter().for_each(|texture| {
            write!(f, " {{ {} }}", texture).unwrap();
        });
        writeln!(f, "}}")?;
        write!(f, "\"cbox\"")?;
        self.cbox.iter().for_each(|coord| {
            write!(f, " {} ", coord).unwrap()
        });
        writeln!(f, "{}", self.collisionmodel)
    }
}

impl Display for CollisionModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"$collisionmodel\" {{\n ")?;
        if self.automass {
            writeln!(f, "\"$automass\"")?;
        } else {
            writeln!(f, "\"{}\"", self.mass)?;
        }
        if self.concave {
            writeln!(f, "\"$concave\"")?;
        }
        write!(f, "}}")
    }
}

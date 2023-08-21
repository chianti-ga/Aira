const CBOX_DEFAULT: [f32; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

pub struct PropsData {
    pub modelname: String,
    pub bodygroup: String,
    pub surfaceprop: String,
    pub contents: String,
    pub cdmaterials: String,
    pub cbox: [f32; 6],
    pub collisionmodel: String,
}

impl Default for PropsData {
    fn default() -> PropsData {
        PropsData {
            modelname: "".to_string(),
            bodygroup: "".to_string(),
            surfaceprop: "".to_string(),
            contents: "".to_string(),
            cdmaterials: "".to_string(),
            cbox: CBOX_DEFAULT,
            collisionmodel: "".to_string(),
        }
    }
}

impl ToString for PropsData {
    fn to_string(&self) -> String {
        format!(
            "modelname:{}\n\
        bodygroup:{}\n\
        surfaceprop:{}\n\
        contents:{}\n\
        cdmaterials:{}\n\
        cbox:{:?}\n\
        collisionmodel{}",
            self.modelname,
            self.bodygroup,
            self.surfaceprop,
            self.contents,
            self.cdmaterials,
            self.cbox,
            self.collisionmodel
        )
    }
}

pub fn create_qc_props(data: PropsData) {
    //Debug
    println!("{}", data.to_string());
}

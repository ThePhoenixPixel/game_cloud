use serde::Serialize;

#[derive(Serialize)]
pub struct Software{
    pub software_type: String,
    pub name: String,
    pub max_ram: u32,
}
impl Software{
    pub fn new() -> Software{
        Software{
            software_type: "Server".to_string(),
            name: "paper".to_string(),
            max_ram: 1024,
        }
    }
    //software type
    pub fn get_software_type(&self) -> &String {
        &self.software_type
    }

    pub fn set_software_type(&mut self, software_type: String) {
        self.software_type = software_type;
    }

    //name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    //max ram
    pub fn get_max_ram(&self) -> &u32 {
        &self.max_ram
    }

    pub fn set_max_ram(&mut self, max_ram: u32) {
        self.max_ram = max_ram;
    }

}
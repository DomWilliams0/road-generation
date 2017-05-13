#[derive(Debug)]
pub enum RoadError {
    Settings(&'static str)
}


pub struct RoadmapBuilder {
    width: i32,
    height: i32
}

impl RoadmapBuilder {

    pub fn new() -> RoadmapBuilder {
        RoadmapBuilder {
            width: 256,
            height: 256
        }
    }

    pub fn size<'a>(&'a mut self, w: i32, h: i32) -> &'a mut RoadmapBuilder {
        self.width = w;
        self.height = h;
        self
    }

    pub fn generate(&self) -> Result<RoadMap, RoadError> {
        Err(RoadError::Settings("Not implemented"))
    }


}

#[derive(Debug)]
pub struct RoadMap {

}

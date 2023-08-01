pub struct HomeInfoProvider {
    pub(crate) default_temp: f32,
}

impl HomeInfoProvider {
    pub async fn get_current_temp(&self) -> Result<f32, ()> {
        Ok(self.default_temp)
    }
}


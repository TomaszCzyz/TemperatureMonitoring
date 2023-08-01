use crate::conditions_comparer::AtmosphereData;

pub struct HomeInfoProvider {
    pub(crate) default_temp: f32,
}

impl HomeInfoProvider {
    pub async fn get_current_temp(&self) -> Result<AtmosphereData, ()> {
        Ok(AtmosphereData {
            temperature: self.default_temp,
            humidity: 0,
        })
    }
}


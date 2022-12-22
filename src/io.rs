use esp_idf_hal::{
    adc::{AdcChannelDriver, AdcConfig, AdcDriver, Atten11dB, ADC1},
    gpio::{Gpio26, Gpio36, InputOutput, PinDriver},
};

pub struct IOPort<'a> {
    pub in_out: PinDriver<'a, Gpio26, InputOutput>,
    pub driver: AdcDriver<'a, ADC1>,
    pub channel_driver: AdcChannelDriver<'a, Gpio36, Atten11dB<ADC1>>,
}

impl<'a> IOPort<'a> {
    pub fn new(in_out: Gpio26, input: Gpio36, adc1: ADC1) -> anyhow::Result<Self> {
        let in_out = PinDriver::input_output(in_out)?;
        let driver = AdcDriver::new(adc1, &AdcConfig::new())?;
        let channel_driver = AdcChannelDriver::new(input)?;
        Ok(Self {
            in_out,
            driver,
            channel_driver,
        })
    }

    pub fn read(&mut self) -> anyhow::Result<u16> {
        Ok(self.driver.read(&mut self.channel_driver)?)
    }
}

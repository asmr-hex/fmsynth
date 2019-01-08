use synthesis::waveform::WaveType;

// Operator is the basic building block for
// FM synthesis components.
pub struct Operator {
    frequency: f64,
    amplitude: f64,
    sample_rate: f64,
    // waveform: Waveform,
    // envelope: Envelope,
}

impl Operator {
    pub fn new(
        frequency: f64,
        amplitude: f64,
        sample_rate:f64,
        waveform: WaveType,
    ) -> Operator {

        match waveform {
            WaveType::Sine => println!("hiii"),
        }
        
        Operator {frequency, amplitude, sample_rate}
    }
}

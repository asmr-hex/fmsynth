mod synthesis;


fn main() {
    println!("starting synth...");
    let operator = synthesis::Operator::new(
        440.0,
        1.0,
        44100.0,
        synthesis::WaveType::Sine,
    ); 
}

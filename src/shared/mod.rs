
pub mod components;
pub mod config;
pub mod gamedata;

// pub fn copy_varstore(from: &Option<Box<VarStore>>) -> Option<Box<VarStore>> {
    // let mut tmp = VarStore::new(Device::cuda_if_available());
    // match tmp.copy(from.as_ref().unwrap()) {
    //     Ok(_) => {}
    //     Err(_) => panic!("Error copying varstore..."),
    // }
    // Some(Box::new(tmp))
    // None
// }

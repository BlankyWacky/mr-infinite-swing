/// Holds the configuration toggles for the utility.
pub struct Config {
    pub infinite_swing: bool,
    pub allow_flying: bool,
}

/// A struct that holds all of the needed memory offsets for the swing component.
pub struct SwingOffsets {
    pub max_angle_can_adjust: usize,
    pub threshold_angle_to_stop_swing: usize,
    pub threshold_speed_to_stop_swing: usize,
    pub min_speed_of_swing: usize,
    pub regular_gravity_scale: usize,
    pub value_of_camera_force: usize,
}

/// Represents the camera's current rotation.
pub struct CameraRotation {
    pub pitch: f64,
}


// Main Utility Function
pub fn apply_swing_mods(
    process: &mut (impl Process + MemoryView),
    swinging_component: Address,
    offsets: &SwingOffsets,
    camera_rotation: &CameraRotation,
    config: &Config,
) -> Result<()> {
    // Apply modifications for infinite swing if enabled
    if config.infinite_swing {
        modify_swingcomponent_offset(process, swinging_component, offsets.max_angle_can_adjust, 100000.0)?;
        modify_swingcomponent_offset(process, swinging_component, offsets.threshold_angle_to_stop_swing, 100000.0)?;
        modify_swingcomponent_offset(process, swinging_component, offsets.threshold_speed_to_stop_swing, -100000.0)?;
        modify_swingcomponent_offset(process, swinging_component, offsets.min_speed_of_swing, -100000.0)?;
    }

    // Apply modifications for flying if enabled
    if config.allow_flying {
        let gravity_scale = (camera_rotation.pitch / 85.0) * -2.0;
        modify_swingcomponent_offset(process, swinging_component, offsets.regular_gravity_scale, gravity_scale as f32)?;
        modify_swingcomponent_offset(process, swinging_component, offsets.value_of_camera_force, 500000.0)?;
    }

    Ok(())
}


fn modify_swingcomponent_offset(
    process: &mut (impl Process + MemoryView),
    swinging_component: Address,
    offset: usize,
    value: f32,
) -> Result<()> {
    let addr = swinging_component + offset;

    // Try to read the current value directly as an f32.
    match read_pod::<f32>(process, addr) {
        Ok(current_value) => {
            if current_value != value {
                if let Err(e) = write_pod(process, addr, &value) {
                    error!("Failed to write f32 at address {:x}: {}", addr, e);
                }
            }
        }
        Err(e) => {
            error!("Failed to read f32 at address {:x}: {}", addr, e);
        }
    }

    Ok(())
}

// Example
fn run_utility_loop(process: &mut (impl Process + MemoryView)) -> Result<()> {
    // 1. Offsets can easily be found on dumpspace
    let offsets = SwingOffsets {
        max_angle_can_adjust: 0x1A8,
        threshold_angle_to_stop_swing: 0x1AC,
        threshold_speed_to_stop_swing: 0x1B0,
        min_speed_of_swing: 0x1B4,
        regular_gravity_scale: 0x1C0,
        value_of_camera_force: 0x1D8,
    };

    // 2. Setup config
    let config = Config {
        infinite_swing: true,
        allow_flying: true,
    };

    // This would be in a loop
    // loop {
        // 3. Find the address of the component (UCharacterSwingingComponent)
        let swinging_component_addr = Address::from(0x7FFABCD0000); // Placeholder address
  
        // 4. Get camera rotations (only pitch is needed)
        let current_camera_rotation = CameraRotation { pitch: 45.0 }; // Placeholder data

        // 5. Call the utility function with the prepared data.
        apply_swing_mods(
            process,
            swinging_component_addr,
            &offsets,
            &current_camera_rotation,
            &config,
        )?;

        // std::thread::sleep(std::time::Duration::from_millis(16));
    // }

    Ok(())
}

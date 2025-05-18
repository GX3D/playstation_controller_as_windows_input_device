// Import necessary modules, including threading, atomic operations, and external libraries.
use std::{
    fmt, process::exit, ptr::eq, sync::{
        atomic::{AtomicBool, Ordering}, Arc
    }, thread, time::{self, Duration}
};

use rdev::{Event, EventType, Key, listen};
use vigem_client::{TargetId, XButtons, XGamepad, Xbox360Wired};


// use dualsense_rs::properties::trigger_effect::TriggerEffect;
// use dualsense_rs::DualSense;


// use dict::{ Dict, DictIface }


pub mod controller_io;



fn main() -> Result<(), Box<dyn std::error::Error>> {




    // let mut dict = Dict::<String>::new();
    
    // Shared flags for the controller state and main loop control.
    let is_active = Arc::new(AtomicBool::new(false));
    let running = Arc::new(AtomicBool::new(true));

    // Register a Ctrl+C handler to gracefully shutdown the application.
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\nCtrl+C received, shutting down...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    // Setup a separate thread for listening to keyboard events.
    let active_clone_for_listener = is_active.clone();
    let running_clone_for_listener = running.clone();
    // let toggle_key = Key::F9;

    thread::spawn(move || {

        
        let api = hidapi::HidApi::new().expect("error0");
        // Print out information about all connected devices
        for device in api.device_list() {
            // println!("{:?}", device);
        }

        println!("Opening...");
        // Connect to device using its VID and PID (Ruyi controller)

        
        // Gamepad	                Vendor Id	Product Id	    Notes
        // Microsoft Xbox One	    045e	    02d1	         
        // Ruyi Wireless	        0483	    5751	        Not yet available
        // Sony PS4	                054c	    09cc	         
        // Sony PS5	                054c	    0ce6	         
        // Nintendo Switch Pro	    057e	    2009
        let (VID, PID) = (0x054c, 0x0ce6);
        let device = api.open(VID, PID).expect("error1");
        let manufacturer = device.get_manufacturer_string().expect("error2");//.unwrap();
        let product = device.get_product_string().expect("error3");
        println!("Product {:?} Device {:?}", manufacturer, product);



        let edvs = dualsense_rs::DualSense::list_devices();
        // dbg!(&edvs);
        
        let mut etarget1: Option<hidapi::DeviceInfo>=None;
        for a in edvs  {
            if a.product_id() == 3302 {
                println!("gefunden");
                etarget1 = Some(a);
            }
        }
        // dbg!(&etarget1);
        let euwarp: hidapi::DeviceInfo = etarget1.unwrap();
        // let mut econtroller = DualSense::new_path(euwarp.path().to_str().unwrap());
        let mut mode = controller_io::mode_type::Usb;

        let opty: hidapi::BusType = euwarp.bus_type();
        print!("controller::BusType  =  ");
        // dbg!(&opty);
        if hidapi::BusType::Bluetooth as i32 == opty as i32 {
            mode = controller_io::mode_type::Bluetooth;

            // By default, bluetooth-connected DualSense only sends input report 0x01 which omits motion and touchpad data.
            // Reading feature report 0x05 causes it to start sending input report 0x31.
            //
            // Note: The Gamepad API will do this for us if it enumerates the gamepad.
            // Other applications like Steam may have also done this already.
            let mut bougger: [u8; 64] = [0u8; 64];
            bougger[0] = 0x05;
            device.get_feature_report(&mut bougger[..]).expect("failed to receive feature-report 0x05");

            // tr_share_byte = 6;
            // tr_x_byte = 5;
            println!("BusType::Bluetooth");
        }else if hidapi::BusType::Usb as i32 == opty as i32 {
            mode = controller_io::mode_type::Usb;
            // tr_share_byte = 6;
            // tr_x_byte = 5;
            println!("BusType::Usb");
        }else if hidapi::BusType::Spi as i32 == opty as i32 {
            // tr_share_byte = 6;
            // tr_x_byte = 5;
            println!("BusType::Spi");
            println!("unknown byte layout for this mode !!");
            println!("-> exiting");
            exit(-1);
        }
        println!(" ---- ");
        
    
        // let mut i: i32 = 0;

        // dead/trigger zones
        let deadzone_stick_l_x = 8;
        let deadzone_stick_l_y = 8;
        let deadzone_stick_r_x = 32;
        let deadzone_stick_r_y = 32;
        let zenzone_1_stick_r_x = 64;
        let zenzone_1_stick_r_y = 64;
        let zenzone_2_stick_r_x = 112;
        let zenzone_2_stick_r_y = 112;

        // speed settings
        let zenith_r_x_0 = 12;
        let zenith_r_y_0 = 12;
        let zenith_r_x_1 = 6;
        let zenith_r_y_1 = 6;
        let zenith_r_x_2 = 0;
        let zenith_r_y_2 = 0;


        let mut debloat_x = 0;
        let mut debloat_square = 0;
        let mut debloat_triangle = 0;
        let mut debloat_circle = 0;

        let mut debloat_touch_0_lpos_x: f32 = 0.0;
        let mut debloat_touch_0_lpos_y: f32 = 0.0;
        let mut touch0_last_state_active= false;

        let mut relay_joy_r_x = 0;
        let mut relay_joy_r_y = 0;

        let mut debloat_dpad_up = 0;
        let mut debloat_dpad_down = 0;
        let mut debloat_dpad_left = 0;
        let mut debloat_dpad_right = 0;

        let mut debloat_L2 = 0;
        let unbloat_L2 = 8;
        let mut debloat_R2 = 0;
        let unbloat_R2 = 8;

        
        let mut debloat_L1 = 0;
        let unbloat_L1 = 8;
        let mut debloat_R1 = 0;
        let unbloat_R1 = 8;


        let poll_tms = 8;
        let mut active = true;
        let mut activity_switch_la = false; // last active
        let     activity_switch_delay = 10000; // hold-down time to trigger `active` switch  (unit.ms)
        let mut activity_switch_tms = 0; // counter (unit.ms)

        // activity_switch  animation
        let     ast__anim__tx = 250; // limiter (unit.ms)
        let mut ast__anim__cc = 0; // counter (unit.ms)
        let mut ast__anim__st = false; // state

        loop {

            let mut buf = [0u8; 64 ];
            let res: usize = device.read_timeout(&mut buf[..], 250).expect("error4");
            let _ = &buf[..res];
            
            if controller_io::get_bool_state(buf, controller_io::button_bool::CREATE_SHARE, mode) {
                activity_switch_tms += poll_tms;
                println!(" act swt ::  {activity_switch_tms} ms / {activity_switch_delay} ms");
                if activity_switch_tms > activity_switch_delay {
                    activity_switch_la = true;
                }else{
                    ast__anim__cc += poll_tms;
                    if ast__anim__cc > ast__anim__tx{
                        ast__anim__cc = 0;
                        ast__anim__st = !ast__anim__st;
                        if ast__anim__st {
                            // let mut dtx= [0u8; 48];
                            // device.write(dtx);
                        }
                        // TODO: toggle mute led with animation as indicator
                    }
                }
            }else if activity_switch_la{
                activity_switch_tms = 0;
                activity_switch_la = false;
                active = !active;
                println!("switched active state to {active}");
            }

            if active {
                // println!("Read: {:3?}", &buf[..res]);

                let lsx = controller_io::get_value_state(buf, controller_io::button_value::STICK_L_X, mode );
                let lsy = controller_io::get_value_state(buf, controller_io::button_value::STICK_L_Y, mode );
                let rsx = controller_io::get_value_state(buf, controller_io::button_value::STICK_R_X, mode );
                let rsy = controller_io::get_value_state(buf, controller_io::button_value::STICK_R_Y, mode );
                

                

                if controller_io::get_bool_state(buf, controller_io::button_bool::L2, mode){
                    if debloat_L2 <= 0 {
                        debloat_L2 = 1;
                        let _ = simulate::press(simulate::Key::PageDown).unwrap();
                    }else{
                        if debloat_L2 < unbloat_L2 {debloat_L2 += 1;}
                    }
                }else if debloat_L2 > 0 {
                    debloat_L2 -= 1;
                    let _ = simulate::release(simulate::Key::PageDown).unwrap();
                }

                if controller_io::get_bool_state(buf, controller_io::button_bool::R2, mode){
                    if debloat_R2 <= 0 {
                        debloat_R2 = 1;
                        let _ = simulate::press(simulate::Key::PageUp).unwrap();
                    }else{
                        if debloat_R2 < unbloat_R2 {debloat_R2 += 1;}
                    }
                }else if debloat_R2 > 0 {
                    debloat_R2 -= 1;
                    let _ = simulate::release(simulate::Key::PageUp).unwrap();
                }
                

                if controller_io::get_bool_state(buf, controller_io::button_bool::L1, mode){
                    if debloat_L1 <= 0 {
                        debloat_L1 = 1;
                        let _ = simulate::press(simulate::Key::BrowserBack).unwrap();
                        let _ = simulate::release(simulate::Key::BrowserBack).unwrap();
                    }else{
                        if debloat_L1 < unbloat_L1 {debloat_L1 += 1;}
                    }
                }else if debloat_L1 > 0 {
                    debloat_L1 -= 1;
                }

                if controller_io::get_bool_state(buf, controller_io::button_bool::R1, mode){
                    if debloat_R1 <= 0 {
                        debloat_R1 = 1;
                        let _ = simulate::press(simulate::Key::BrowserForward).unwrap();
                        let _ = simulate::release(simulate::Key::BrowserForward).unwrap();
                    }else{
                        if debloat_R1 < unbloat_R1 {debloat_R1 += 1;}
                    }
                }else if debloat_R1 > 0 {
                    debloat_R1 -= 1;
                }

                
                if controller_io::get_bool_state(buf, controller_io::button_bool::X, mode){
                    if debloat_x <= 0 {
                        debloat_x = 1;
                        let _ = simulate::press(simulate::Key::MouseLeft).unwrap();
                    }else{
                        if debloat_x < 4 {debloat_x += 1;}
                    }
                }else if debloat_x > 0 {
                    debloat_x -= 1;
                    let _ = simulate::release(simulate::Key::MouseLeft).unwrap();
                }

                if controller_io::get_bool_state(buf, controller_io::button_bool::SQUARE, mode){
                    if debloat_square <= 0 {
                        debloat_square = 1;
                        let _ = simulate::press(simulate::Key::MouseRight).unwrap();
                    }else{
                        if debloat_square < 3 {debloat_square += 1;}
                    }
                }else if debloat_square > 0 {
                    debloat_square -= 1;
                    let _ = simulate::release(simulate::Key::MouseRight).unwrap();
                }

                if controller_io::get_bool_state(buf, controller_io::button_bool::TRIANGLE, mode){
                    if debloat_triangle <= 0 {
                        debloat_triangle = 1;
                        let _ = simulate::press(simulate::Key::Control).unwrap();
                    }else{
                        if debloat_triangle < 3 {debloat_triangle += 1;}
                    }
                }else if debloat_triangle > 0 {
                    debloat_triangle -= 1;
                    let _ = simulate::release(simulate::Key::Control).unwrap();
                }

                if controller_io::get_bool_state(buf, controller_io::button_bool::O, mode){
                    if debloat_circle <= 0 {
                        debloat_circle = 1;
                        let _ = simulate::press(simulate::Key::Shift).unwrap();
                    }else{
                        if debloat_circle < 3 {debloat_circle += 1;}
                    }
                }else if debloat_circle > 0 {
                    debloat_circle -= 1;
                    let _ = simulate::release(simulate::Key::Shift).unwrap();
                }


                
                if controller_io::get_bool_state(buf, controller_io::button_bool::DPAD_DOWN, mode){
                    if debloat_dpad_down <= 0 {
                        debloat_dpad_down = 1;
                        let _ = simulate::press(simulate::Key::Down).unwrap();
                        let _ = simulate::release(simulate::Key::Down).unwrap();
                    }else{
                        if debloat_dpad_down < 8 {debloat_dpad_down += 1;}
                    }
                }else if debloat_dpad_down > 0 {
                    debloat_dpad_down -= 1;
                }

                if controller_io::get_bool_state(buf, controller_io::button_bool::DPAD_UP, mode){
                    if debloat_dpad_up <= 0 {
                        debloat_dpad_up = 1;
                        let _ = simulate::press(simulate::Key::Up).unwrap();
                        let _ = simulate::release(simulate::Key::Up).unwrap();
                    }else{
                        if debloat_dpad_up < 8 {debloat_dpad_up += 1;}
                    }
                }else if debloat_dpad_up > 0 {
                    debloat_dpad_up -= 1;
                }

                if controller_io::get_bool_state(buf, controller_io::button_bool::DPAD_LEFT, mode){
                    if debloat_dpad_left <= 0 {
                        debloat_dpad_left = 1;
                        let _ = simulate::press(simulate::Key::Left).unwrap();
                        let _ = simulate::release(simulate::Key::Left).unwrap();
                    }else{
                        if debloat_dpad_left < 8 {debloat_dpad_left += 1;}
                    }
                }else if debloat_dpad_left > 0 {
                    debloat_dpad_left -= 1;
                }

                if controller_io::get_bool_state(buf, controller_io::button_bool::DPAD_RIGHT, mode){
                    if debloat_dpad_right <= 0 {
                        debloat_dpad_right = 1;
                        let _ = simulate::press(simulate::Key::Right).unwrap();
                        let _ = simulate::release(simulate::Key::Right).unwrap();
                    }else{
                        if debloat_dpad_right < 8 {debloat_dpad_right += 1;}
                    }
                }else if debloat_dpad_right > 0 {
                    debloat_dpad_right -= 1;
                }
                

                if lsx > deadzone_stick_l_x || lsx < -deadzone_stick_l_x || lsy > deadzone_stick_l_y || lsy < -deadzone_stick_l_y {
                    let mut msx = 0;
                    if lsx!=0{ msx = ((lsx as f32) / 12.8) as i32;}
                    let mut msy = 0;
                    if lsy!=0 {msy = ((lsy as f32) / 12.8) as i32;}

                    // Move the mouse 100 pixels  left, 50 pixels down
                    simulate::move_mouse_relative(msx, msy).unwrap();

                    // // Move the mouse at the center of the screen.
                    // simulate::move_mouse_absolute(0.5, 0.5).unwrap();

                    // std::thread::sleep(Duration::from_millis(50));
                    
                }



                if rsx > deadzone_stick_r_x || rsx < -deadzone_stick_r_x || rsy > deadzone_stick_r_y || rsy < -deadzone_stick_r_y {
                    if relay_joy_r_x == 0 {
                        if rsx > zenzone_2_stick_r_x {
                            relay_joy_r_x = zenith_r_x_2;
                            let _ = simulate::send(simulate::Key::Right);
                        }else if rsx < -zenzone_2_stick_r_x {
                            relay_joy_r_x = zenith_r_x_2;
                            let _ = simulate::send(simulate::Key::Left);
                        }else if rsx > zenzone_1_stick_r_x {
                            relay_joy_r_x = zenith_r_x_1;
                            let _ = simulate::send(simulate::Key::Right);
                        }else if rsx < -zenzone_1_stick_r_x {
                            relay_joy_r_x = zenith_r_x_1;
                            let _ = simulate::send(simulate::Key::Left);
                        }else if rsx > deadzone_stick_r_x {
                            relay_joy_r_x = zenith_r_x_0;
                            let _ = simulate::send(simulate::Key::Right);
                        }else if rsx < -deadzone_stick_r_x {
                            relay_joy_r_x = zenith_r_x_0;
                            let _ = simulate::send(simulate::Key::Left);
                        }

                        if rsy > zenzone_2_stick_r_y {
                            relay_joy_r_x = zenith_r_y_2;
                            let _ = simulate::send(simulate::Key::Down);
                        }else if rsy < -zenzone_2_stick_r_y {
                            relay_joy_r_x = zenith_r_y_2;
                            let _ = simulate::send(simulate::Key::Up);
                        }else if rsy > zenzone_1_stick_r_y {
                            relay_joy_r_x = zenith_r_y_1;
                            let _ = simulate::send(simulate::Key::Down);
                        }else if rsy < -zenzone_1_stick_r_y {
                            relay_joy_r_x = zenith_r_y_1;
                            let _ = simulate::send(simulate::Key::Up);
                        }else if rsy > deadzone_stick_r_y {
                            relay_joy_r_x = zenith_r_y_0;
                            let _ = simulate::send(simulate::Key::Down);
                        }else if rsy < -deadzone_stick_r_y {
                            relay_joy_r_x = zenith_r_y_0;
                            let _ = simulate::send(simulate::Key::Up);
                        }
                    }else if relay_joy_r_x > 0 {
                        relay_joy_r_x -= 1
                    }
                }
                

                if controller_io::get_bool_state(buf, controller_io::button_bool::TOUCHPOINT_0, mode){
                    let tp0x = (((((controller_io::get_value_state(buf, controller_io::button_value::TOUCHPOINT_0_X, mode) as f32)+1.0) / 1920.0)-(1.0/1920.0)) * 1.0);
                    let tp0y = (((((controller_io::get_value_state(buf, controller_io::button_value::TOUCHPOINT_0_Y, mode) as f32)+1.0) / 1080.0)-(1.0/1080.0)) * 1.0);
                    if !touch0_last_state_active {
                        debloat_touch_0_lpos_x = tp0x;
                        debloat_touch_0_lpos_y = tp0y;
                        touch0_last_state_active = true;
                    }
                    if debloat_touch_0_lpos_x != tp0x || debloat_touch_0_lpos_y != tp0y {
                        println!("tp0  {:9}  :  {:9}", controller_io::get_value_state(buf, controller_io::button_value::TOUCHPOINT_0_X, mode), controller_io::get_value_state(buf, controller_io::button_value::TOUCHPOINT_0_Y, mode));
                        let div_x = (((tp0x - debloat_touch_0_lpos_x) * 1920.0) * 0.2) as i32;
                        let div_y = (((tp0y - debloat_touch_0_lpos_y) * 1080.0) * 0.2) as i32;
                        debloat_touch_0_lpos_x = tp0x;
                        debloat_touch_0_lpos_y = tp0y;
                        simulate::move_mouse_absolute(tp0x*1.10, tp0y*1.05).expect("error tp0");
                        // simulate::move_mouse_relative(div_x, div_y).expect("error tp0");
                    }else{
                        // touch0_last_state_active = false;
                    }
                }else{
                    touch0_last_state_active = false;
                }
            }
            std::thread::sleep(Duration::from_millis(poll_tms));
            

        }
        
    });




    // Main loop that sends controller updates based on the active state.
    // let mut current_active_state = false;
    while running.load(Ordering::SeqCst) {
        let current_active_state = is_active.load(Ordering::SeqCst);
        thread::sleep(time::Duration::from_millis(50));
    }

    // // Cleanup: release any pressed buttons and unplug the virtual controller.
    // println!("Exiting main loop. Unplugging controller...");
    // gamepad_state.buttons = XButtons::default();
    // let _ = target.update(&gamepad_state);
    // target.unplug()?;
    // println!("Virtual controller unplugged. Goodbye!");

    Ok(())
}



// // prefix `lt` indicator for threaded application starter

// fn lt_thread_binding_joystick() {
    
// }


// fn lt_thread_keybinding_mouse(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

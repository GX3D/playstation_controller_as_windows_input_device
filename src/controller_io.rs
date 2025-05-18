use std::{u8};




#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum button_bool {
    X = 0x00,
    O = 0x01,
    SQUARE = 0x02,
    TRIANGLE = 0x03,
    L1 = 0x04,
    R1 = 0x05,
    CREATE_SHARE = 0x06,
    OPTION_MENU = 0x07,
    L3 = 0x08,
    R3 = 0x09,
    DPAD_UP = 0x0a,
    DPAD_DOWN = 0x0b,
    DPAD_LEFT = 0x0c,
    DPAD_RIGHT = 0x0d,
    PS = 0x0e,
    TOUCHPAD = 0x0f,
    MUTE = 0x10,
    TOUCHPOINT_0 = 0x11,
    TOUCHPOINT_1 = 0x12,
    L2_ADAPTIVE_TRIGGER_EFFECT = 0x13,
    R2_ADAPTIVE_TRIGGER_EFFECT = 0x14,
    CHARGING = 0x15,
    FULLY_CHARED = 0x16,
    L2 = 0x17,
    R2 = 0x18,
}

impl std::fmt::Display for button_bool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
          button_bool::X => write!(f, "X"),
          button_bool::O => write!(f, "O"),
          button_bool::SQUARE => write!(f, "SQUARE"),
          button_bool::TRIANGLE => write!(f, "TRIANGLE"),
          button_bool::L1 => write!(f, "L1"),
          button_bool::R1 => write!(f, "R1"),
          button_bool::CREATE_SHARE => write!(f, "CREATE_SHARE"),
          button_bool::OPTION_MENU => write!(f, "OPTION_MENU"),
          button_bool::L3 => write!(f, "L3"),
          button_bool::R3 => write!(f, "R3"),
          button_bool::DPAD_UP => write!(f, "DPAD_UP"),
          button_bool::DPAD_DOWN => write!(f, "DPAD_DOWN"),
          button_bool::DPAD_LEFT => write!(f, "DPAD_LEFT"),
          button_bool::DPAD_RIGHT => write!(f, "DPAD_RIGHT"),
          button_bool::PS => write!(f, "PS"),
          button_bool::TOUCHPAD => write!(f, "TOUCHPAD"),
          button_bool::MUTE => write!(f, "MUTE"),
          button_bool::TOUCHPOINT_0 => write!(f, "TOUCHPOINT_0"),
          button_bool::TOUCHPOINT_1 => write!(f, "TOUCHPOINT_1"),
          button_bool::L2_ADAPTIVE_TRIGGER_EFFECT => write!(f, "L2_ADAPTIVE_TRIGGER_EFFECT"),
          button_bool::R2_ADAPTIVE_TRIGGER_EFFECT => write!(f, "R2_ADAPTIVE_TRIGGER_EFFECT"),
          button_bool::CHARGING => write!(f, "CHARGING"),
          button_bool::FULLY_CHARED => write!(f, "FULLY_CHARED"),
          button_bool::L2 => write!(f, "L2"),
          button_bool::R2 => write!(f, "R2"),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum button_value {
    L2 = 0x00,
    R2 = 0x01,
    TOUCHPOINT_0_ID = 0x02,
    TOUCHPOINT_0_X= 0x03,
    TOUCHPOINT_0_Y = 0x04,
    TOUCHPOINT_1_ID = 0x05,
    TOUCHPOINT_1_X = 0x06,
    TOUCHPOINT_1_Y = 0x07,
    // _______ = 0x08,
    // _______ = 0x09,
    STICK_L_X = 0x0a,
    STICK_L_Y = 0x0b,
    STICK_R_X = 0x0c,
    STICK_R_Y = 0x0d,
    GYRO_X = 0x0e,
    GYRO_Y = 0x0f,
    GYRO_Z = 0x10,
    ACCELEROMETER_X = 0x11,
    ACCELEROMETER_Y = 0x12,
    ACCELEROMETER_Z = 0x13,
    BATTERY_LEVEL = 0x14,
    L2_TRIGGER_STATE = 0x15,
    R2_TRIGGER_STATE = 0x16,
    // _______ = 0x17,
    // _______ = 0x18,
    // _______ = 0x19,
    // _______ = 0x1a,
    // _______ = 0x1b,
    // _______ = 0x1c,
    // _______ = 0x1d,
    // _______ = 0x1e,
    // _______ = 0x1f,
}


impl std::fmt::Display for button_value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
          button_value::L2 => write!(f, "L2"),
          button_value::R2 => write!(f, "R2"),
          button_value::TOUCHPOINT_0_ID => write!(f, "TOUCHPOINT_0_ID"),
          button_value::TOUCHPOINT_0_X => write!(f, "TOUCHPOINT_0_X"),
          button_value::TOUCHPOINT_0_Y => write!(f, "TOUCHPOINT_0_Y"),
          button_value::TOUCHPOINT_1_ID => write!(f, "TOUCHPOINT_1_ID"),
          button_value::TOUCHPOINT_1_X => write!(f, "TOUCHPOINT_1_X"),
          button_value::TOUCHPOINT_1_Y => write!(f, "TOUCHPOINT_1_Y"),
          button_value::STICK_L_X => write!(f, "STICK_L_X"),
          button_value::STICK_L_Y => write!(f, "STICK_L_Y"),
          button_value::STICK_R_X => write!(f, "STICK_R_X"),
          button_value::STICK_R_Y => write!(f, "STICK_R_Y"),
          button_value::GYRO_X => write!(f, "GYRO_X"),
          button_value::GYRO_Y => write!(f, "GYRO_Y"),
          button_value::GYRO_Z => write!(f, "GYRO_Z"),
          button_value::ACCELEROMETER_X => write!(f, "ACCELEROMETER_X"),
          button_value::ACCELEROMETER_Y => write!(f, "ACCELEROMETER_Y"),
          button_value::ACCELEROMETER_Z => write!(f, "ACCELEROMETER_Z"),
          button_value::BATTERY_LEVEL => write!(f, "BATTERY_LEVEL"),
          button_value::L2_TRIGGER_STATE => write!(f, "L2_TRIGGER_STATE"),
          button_value::R2_TRIGGER_STATE => write!(f, "R2_TRIGGER_STATE"),
        }
    }
}

fn bit_checker( byte:u8, mask: u8 ) -> bool {
  // https://users.rust-lang.org/t/extracting-bits-from-bytes/77110
  // check all the bits from LSB to MSB
  // for i in 0..8 {
  //   if i == bit_index{
      // let mask = 1 << i;
      let it_is_set = (mask & byte) > 0;
      return it_is_set;
  //   }
  // }
  // return false;
}





fn read_bool_from_buffer_inverted( buffer: [u8; 64 ], byte_index: usize, bit_index: usize ) -> bool{
  return !d_read_bool_from_buffer( buffer, byte_index, bit_index, true );
}
fn read_bool_from_buffer( buffer: [u8; 64 ], byte_index: usize, bit_index: usize ) -> bool{
  return d_read_bool_from_buffer( buffer, byte_index, bit_index, false );
}
fn d_read_bool_from_buffer( buffer: [u8; 64 ], byte_index: usize, bit_index: usize, inverted:bool ) -> bool{

  let itm = &buffer[byte_index];
    // if itm.bitand(rhs) {
      // return bit_checker(*itm,  0b10000000);
  return bit_checker( *itm, 1 << bit_index );

  // return false;
  
}

fn read_i32_from_buffer( buffer: [u8; 64 ], byte_index: usize, bit_mask: u8 ) -> i32{
  let mona = buffer[byte_index] & bit_mask;
  return mona as i32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum mode_type {
  Bluetooth,
  Usb
}
pub fn normalizeButton (val:u8) -> bool{
  // src:
  // >> const normalizeButton = value => {
  // >>   return value ? 1.0 : 0.0;
  // >> };
  if val > 0 {
    return true;
  }else{
    return false;
  }
}

pub fn get_bool_state( buffer: [u8; 64 ], key: button_bool, mode:mode_type ) -> bool{
  // match key as i32 {
  if mode as i32 == mode_type::Usb as i32{


    // let axes0 = buffer[0+1];
    // let axes1 = buffer[1+1];
    // let axes2 = buffer[2+1];
    // let axes3 = buffer[3+1];
    // let axes4 = buffer[4+1];
    // let axes5 = buffer[5+1];
    let seqNum = buffer[6+1];
    let buttons0 = buffer[7+1];
    let buttons1 = buffer[8+1];
    let buttons2 = buffer[9+1];
    let buttons3 = buffer[10+1];
    // let timestamp0 = buffer[11+1];
    // let timestamp1 = buffer[12+1];
    // let timestamp2 = buffer[13+1];
    // let timestamp3 = buffer[14+1];
    // let gyroX0 = buffer[15+1];
    // let gyroX1 = buffer[16+1];
    // let gyroY0 = buffer[17+1];
    // let gyroY1 = buffer[18+1];
    // let gyroZ0 = buffer[19+1];
    // let gyroZ1 = buffer[20+1];
    // let accelX0 = buffer[21+1];
    // let accelX1 = buffer[22+1];
    // let accelY0 = buffer[23+1];
    // let accelY1 = buffer[24+1];
    // let accelZ0 = buffer[25+1];
    // let accelZ1 = buffer[26+1];
    // let sensorTimestamp0 = buffer[27+1];
    // let sensorTimestamp1 = buffer[28+1];
    // let sensorTimestamp2 = buffer[29+1];
    // let sensorTimestamp3 = buffer[30+1];
    // byte 31?
    // let touch00 = buffer[32+1];
    // let touch01 = buffer[33+1];
    // let touch02 = buffer[34+1];
    // let touch03 = buffer[35+1];
    // let touch10 = buffer[36+1];
    // let touch11 = buffer[37+1];
    // let touch12 = buffer[38+1];
    // let touch13 = buffer[39+1];
    	// byte 40?
    // let r2feedback = buffer[41+1];
    // let l2feedback = buffer[42+1];
    // bytes 43-51?
    let battery0 = buffer[52+1];
    let battery1 = buffer[53+1];
    // bytes 54-58?
    
    let dpad = buttons0 & 0b00001111;
    let up = dpad == 0 || dpad == 1 || dpad == 7;
    let down = dpad == 3 || dpad == 4 || dpad == 5;
    let left = dpad == 5 || dpad == 6 || dpad == 7;
    let right = dpad == 1 || dpad == 2 || dpad == 3;
    let square = normalizeButton(buttons0 & 0b00010000);
    let cross = normalizeButton(buttons0 & 0b00100000);
    let circle = normalizeButton(buttons0 & 0b01000000);
    let triangle = normalizeButton(buttons0 & 0b10000000);
    let l1 = normalizeButton(buttons1 & 0b00000001);
    let r1 = normalizeButton(buttons1 & 0b00000010);
    let l2 = normalizeButton(buttons1 & 0b00000100);
    let r2 = normalizeButton(buttons1 & 0b00001000);
    let create = normalizeButton(buttons1 & 0b00010000);
    let options = normalizeButton(buttons1 & 0b00100000);
    let l3 = normalizeButton(buttons1 & 0b01000000);
    let r3 = normalizeButton(buttons1 & 0b10000000);
    let ps = normalizeButton(buttons2 & 0b00000001);
    let touchpad = normalizeButton(buttons2 & 0b00000010);
    let mute = normalizeButton(buttons2 & 0b00000100);
    

    let touch00 = buffer[32+1];
    let touch10 = buffer[36+1];
    
    if key as i32 == button_bool::X as i32 {
      return cross;//read_bool_from_buffer(buffer,8,5);
    }else if key as i32 == button_bool::O as i32 {
      return circle;//read_bool_from_buffer(buffer,8,6);
    }else if key as i32 == button_bool::SQUARE as i32 {
      return square;//read_bool_from_buffer(buffer,8,4);
    }else if key as i32 == button_bool::TRIANGLE as i32 {
      return triangle;//read_bool_from_buffer(buffer,8,7);
    }else if key as i32 == button_bool::L1 as i32 {
      return l1;//read_bool_from_buffer(buffer,9,0);
    }else if key as i32 == button_bool::R1 as i32 {
      return r1//read_bool_from_buffer(buffer,9,1);
    }else if key as i32 == button_bool::L2 as i32 {
      return l2;//read_bool_from_buffer(buffer,9,2);
    }else if key as i32 == button_bool::R2 as i32 {
      return r2;//read_bool_from_buffer(buffer,9,3);
    }else if key as i32 == button_bool::CREATE_SHARE as i32 {
      return create;//read_bool_from_buffer(buffer,9,4);
    }else if key as i32 == button_bool::OPTION_MENU as i32 {
      return options;//read_bool_from_buffer(buffer,9,5);
    }else if key as i32 == button_bool::L3 as i32 {
      return l3;//read_bool_from_buffer(buffer,9,6);
    }else if key as i32 == button_bool::R3 as i32 {
      return r3;//read_bool_from_buffer(buffer,9,7);
    }else if key as i32 == button_bool::PS as i32 {
      return ps;//read_bool_from_buffer(buffer,10,0);
    }else if key as i32 == button_bool::MUTE as i32 {
      return mute;//read_bool_from_buffer(buffer,10,2);
    }else if key as i32 == button_bool::TOUCHPAD as i32 {
      return touchpad;//read_bool_from_buffer(buffer,10,1);
    }else if key as i32 == button_bool::TOUCHPOINT_0 as i32 {
      return !normalizeButton(touch00 & 0b10000000);//read_bool_from_buffer_inverted(buffer,35,0);
    }else if key as i32 == button_bool::TOUCHPOINT_1 as i32 {
      return !normalizeButton(touch10 & 0b10000000);//read_bool_from_buffer_inverted(buffer,39,0);
    }else if key as i32 == button_bool::DPAD_UP as i32 {
      return up;
    }else if key as i32 == button_bool::DPAD_LEFT as i32 {
      return left;
    }else if key as i32 == button_bool::DPAD_RIGHT as i32 {
      return right;
    }else if key as i32 == button_bool::DPAD_DOWN as i32 {
      return down;
    }else{
      println!(" MODE::USB 0x01    unknown key  {} ", key as i32);
      return false;
    }
  }else if mode as i32 == mode_type::Bluetooth as i32{
    if buffer[0] == 0b00000001 {

      // let axes0 = [0+1];
      // let axes1 = [1+1];
      // let axes2 = [2+1];
      // let axes3 = [3+1];
      let buttons0 = buffer[4+1];
      let buttons1 = buffer[5+1];
      let buttons2 = buffer[6+1];
      // let axes4 = [7+1];
      // let axes5 = [8+1];
        
      let dpad = buttons0 & 0b00001111;
      let up = dpad == 0 || dpad == 1 || dpad == 7;
      let down = dpad == 3 || dpad == 4 || dpad == 5;
      let left = dpad == 5 || dpad == 6 || dpad == 7;
      let right = dpad == 1 || dpad == 2 || dpad == 3;
      let square = normalizeButton(buttons0 & 0b00010000);
      let cross = normalizeButton(buttons0 & 0b00100000);
      let circle = normalizeButton(buttons0 & 0b01000000);
      let triangle = normalizeButton(buttons0 & 0b10000000);
      let l1 = normalizeButton(buttons1 & 0b00000001);
      let r1 = normalizeButton(buttons1 & 0b00000010);
      let l2 = normalizeButton(buttons1 & 0b00000100);
      let r2 = normalizeButton(buttons1 & 0b00001000);
      let create = normalizeButton(buttons1 & 0b00010000);
      let options = normalizeButton(buttons1 & 0b00100000);
      let l3 = normalizeButton(buttons1 & 0b01000000);
      let r3 = normalizeButton(buttons1 & 0b10000000);
      let ps = normalizeButton(buttons2 & 0b00000001);
      let touchpad = normalizeButton(buttons2 & 0b00000010);
      // let mute = normalizeButton(buttons2 & 0b00000100);
      
      if key as i32 == button_bool::X as i32 {
        return cross;//read_bool_from_buffer(buffer,8,5);
      }else if key as i32 == button_bool::O as i32 {
        return circle;//read_bool_from_buffer(buffer,8,6);
      }else if key as i32 == button_bool::SQUARE as i32 {
        return square;//read_bool_from_buffer(buffer,8,4);
      }else if key as i32 == button_bool::TRIANGLE as i32 {
        return triangle;//read_bool_from_buffer(buffer,8,7);
      }else if key as i32 == button_bool::L1 as i32 {
        return l1;//read_bool_from_buffer(buffer,9,0);
      }else if key as i32 == button_bool::R1 as i32 {
        return r1//read_bool_from_buffer(buffer,9,1);
      }else if key as i32 == button_bool::L2 as i32 {
        return l2;//read_bool_from_buffer(buffer,9,2);
      }else if key as i32 == button_bool::R2 as i32 {
        return r2;//read_bool_from_buffer(buffer,9,3);
      }else if key as i32 == button_bool::CREATE_SHARE as i32 {
        return create;//read_bool_from_buffer(buffer,9,4);
      }else if key as i32 == button_bool::OPTION_MENU as i32 {
        return options;//read_bool_from_buffer(buffer,9,5);
      }else if key as i32 == button_bool::L3 as i32 {
        return l3;//read_bool_from_buffer(buffer,9,6);
      }else if key as i32 == button_bool::R3 as i32 {
        return r3;//read_bool_from_buffer(buffer,9,7);
      }else if key as i32 == button_bool::PS as i32 {
        return ps;//read_bool_from_buffer(buffer,10,0);
      }else if key as i32 == button_bool::TOUCHPAD as i32 {
        return touchpad;//read_bool_from_buffer(buffer,10,1);
      }else if key as i32 == button_bool::DPAD_UP as i32 {
        return up;
      }else if key as i32 == button_bool::DPAD_LEFT as i32 {
        return left;
      }else if key as i32 == button_bool::DPAD_RIGHT as i32 {
        return right;
      }else if key as i32 == button_bool::DPAD_DOWN as i32 {
        return down;
      }else{
        println!(" MODE::Bluetooth 0x01    unknown key  {} ", key as i32);
        return false;
      }



    }else if buffer[0] == 0b00110001 {
      let oef = 1;
      // byte 0?
      // let axes0 = buffer[1+oef];
      // let axes1 = buffer[2+oef];
      // let axes2 = buffer[3+oef];
      // let axes3 = buffer[4+oef];
      // let axes4 = buffer[5+oef];
      // let axes5 = buffer[6+oef];
      // byte 7?
      let buttons0 = buffer[8+oef];
      let buttons1 = buffer[9+oef];
      let buttons2 = buffer[10+oef];
      // byte 11?
      // let timestamp0 = buffer[12+oef];
      // let timestamp1 = buffer[13+oef];
      // let timestamp2 = buffer[14+oef];
      // let timestamp3 = buffer[15+oef];
      // let gyroX0 = buffer[16+oef];
      // let gyroX1 = buffer[17+oef];
      // let gyroY0 = buffer[18+oef];
      // let gyroY1 = buffer[19+oef];
      // let gyroZ0 = buffer[20+oef];
      // let gyroZ1 = buffer[21+oef];
      // let accelX0 = buffer[22+oef];
      // let accelX1 = buffer[23+oef];
      // let accelY0 = buffer[24+oef];
      // let accelY1 = buffer[25+oef];
      // let accelZ0 = buffer[26+oef];
      // let accelZ1 = buffer[27+oef];
      // bytes 28-32?
      let touch00 = buffer[33+oef];
      // let touch01 = buffer[34+oef];
      // let touch02 = buffer[35+oef];
      // let touch03 = buffer[36+oef];
      let touch10 = buffer[37+oef];
      // let touch11 = buffer[38+oef];
      // let touch12 = buffer[39+oef];
      // let touch13 = buffer[40+oef];
      // // byte 41?
      // let r2feedback = buffer[42+oef];
      // let l2feedback = buffer[43+oef];
      // // bytes 44-52?
      // let battery0 = buffer[53+oef];
      // let battery1 = buffer[54+oef];
      // // bytes 55-76?


      
      let dpad = buttons0 & 0b00001111;
      let up = dpad == 0 || dpad == 1 || dpad == 7;
      let down = dpad == 3 || dpad == 4 || dpad == 5;
      let left = dpad == 5 || dpad == 6 || dpad == 7;
      let right = dpad == 1 || dpad == 2 || dpad == 3;
      let square = normalizeButton(buttons0 & 0b00010000);
      let cross = normalizeButton(buttons0 & 0b00100000);
      let circle = normalizeButton(buttons0 & 0b01000000);
      let triangle = normalizeButton(buttons0 & 0b10000000);
      let l1 = normalizeButton(buttons1 & 0b00000001);
      let r1 = normalizeButton(buttons1 & 0b00000010);
      let l2 = normalizeButton(buttons1 & 0b00000100);
      let r2 = normalizeButton(buttons1 & 0b00001000);
      let create = normalizeButton(buttons1 & 0b00010000);
      let options = normalizeButton(buttons1 & 0b00100000);
      let l3 = normalizeButton(buttons1 & 0b01000000);
      let r3 = normalizeButton(buttons1 & 0b10000000);
      let ps = normalizeButton(buttons2 & 0b00000001);
      let touchpad = normalizeButton(buttons2 & 0b00000010);
      let mute = normalizeButton(buttons2 & 0b00000100);


      if key as i32 == button_bool::X as i32 {
        return cross;//read_bool_from_buffer(buffer,8,5);
      }else if key as i32 == button_bool::O as i32 {
        return circle;//read_bool_from_buffer(buffer,8,6);
      }else if key as i32 == button_bool::SQUARE as i32 {
        return square;//read_bool_from_buffer(buffer,8,4);
      }else if key as i32 == button_bool::TRIANGLE as i32 {
        return triangle;//read_bool_from_buffer(buffer,8,7);
      }else if key as i32 == button_bool::L1 as i32 {
        return l1;//read_bool_from_buffer(buffer,9,0);
      }else if key as i32 == button_bool::R1 as i32 {
        return r1//read_bool_from_buffer(buffer,9,1);
      }else if key as i32 == button_bool::L2 as i32 {
        return l2;//read_bool_from_buffer(buffer,9,2);
      }else if key as i32 == button_bool::R2 as i32 {
        return r2;//read_bool_from_buffer(buffer,9,3);
      }else if key as i32 == button_bool::CREATE_SHARE as i32 {
        return create;//read_bool_from_buffer(buffer,9,4);
      }else if key as i32 == button_bool::OPTION_MENU as i32 {
        return options;//read_bool_from_buffer(buffer,9,5);
      }else if key as i32 == button_bool::L3 as i32 {
        return l3;//read_bool_from_buffer(buffer,9,6);
      }else if key as i32 == button_bool::R3 as i32 {
        return r3;//read_bool_from_buffer(buffer,9,7);
      }else if key as i32 == button_bool::PS as i32 {
        return ps;//read_bool_from_buffer(buffer,10,0);
      }else if key as i32 == button_bool::MUTE as i32 {
        return mute;//read_bool_from_buffer(buffer,10,2);
      }else if key as i32 == button_bool::TOUCHPAD as i32 {
        return touchpad;//read_bool_from_buffer(buffer,10,1);
      }else if key as i32 == button_bool::TOUCHPOINT_0 as i32 {
        return !normalizeButton(touch00 & 0b10000000);//read_bool_from_buffer_inverted(buffer,35,0);
      }else if key as i32 == button_bool::TOUCHPOINT_1 as i32 {
        return !normalizeButton(touch10 & 0b10000000);//read_bool_from_buffer_inverted(buffer,39,0);
      }else if key as i32 == button_bool::DPAD_UP as i32 {
        return up;
      }else if key as i32 == button_bool::DPAD_LEFT as i32 {
        return left;
      }else if key as i32 == button_bool::DPAD_RIGHT as i32 {
        return right;
      }else if key as i32 == button_bool::DPAD_DOWN as i32 {
        return down;
      }else{
        println!(" MODE::Bluetooth 0x31    unknown key  {} ", key as i32);
        return false;
      }
    }else{
      println!(" mode Bluetooth has invalid report id  {:2x?}  ( {:3?} )", buffer[0], buffer[0]);
    }
  }


  return false;
  // }
}


















pub fn get_value_state( buffer: [u8; 64 ], key: button_value, mode:mode_type ) -> i32{
  // match key as i32 {
  if mode as i32 == mode_type::Usb as i32{
    
    let axes0 = buffer[0+1];
    let axes1 = buffer[1+1];
    let axes2 = buffer[2+1];
    let axes3 = buffer[3+1];
    let axes4 = buffer[4+1];
    let axes5 = buffer[5+1];
    let seqNum = buffer[6+1];
    let buttons0 = buffer[7+1];
    let buttons1 = buffer[8+1];
    let buttons2 = buffer[9+1];
    let buttons3 = buffer[10+1];
    let timestamp0 = buffer[11+1];
    let timestamp1 = buffer[12+1];
    let timestamp2 = buffer[13+1];
    let timestamp3 = buffer[14+1];
    let gyroX0 = buffer[15+1];
    let gyroX1 = buffer[16+1];
    let gyroY0 = buffer[17+1];
    let gyroY1 = buffer[18+1];
    let gyroZ0 = buffer[19+1];
    let gyroZ1 = buffer[20+1];
    let accelX0 = buffer[21+1];
    let accelX1 = buffer[22+1];
    let accelY0 = buffer[23+1];
    let accelY1 = buffer[24+1];
    let accelZ0 = buffer[25+1];
    let accelZ1 = buffer[26+1];
    let sensorTimestamp0 = buffer[27+1];
    let sensorTimestamp1 = buffer[28+1];
    let sensorTimestamp2 = buffer[29+1];
    let sensorTimestamp3 = buffer[30+1];
    // byte 31?
    let touch00 = buffer[32+1];
    let touch01 = buffer[33+1];
    let touch02 = buffer[34+1];
    let touch03 = buffer[35+1];
    let touch10 = buffer[36+1];
    let touch11 = buffer[37+1];
    let touch12 = buffer[38+1];
    let touch13 = buffer[39+1];
    	// byte 40?
    let r2feedback = buffer[41+1];
    let l2feedback = buffer[42+1];
    // bytes 43-51?
    let battery0 = buffer[52+1];
    let battery1 = buffer[53+1];
    // bytes 54-58?


    let touch0active = !(touch00 & 0b10000000);
    let touch0id = (touch00 & 0b01111111);
    let touch0x = (((touch02 & 0b00001111) as u16) << 8) | (touch01 as u16);
    let touch0y = ((touch03 as u16) << 4) | (((touch02 & 0b11110000) >> 4) as u16);
    let touch1active = !(touch10 & 0b10000000);
    let touch1id = (touch10 & 0b01111111);
    let touch1x = (((touch12 & 0b00001111) as u16) << 8) | (touch11 as u16);
    let touch1y = ((touch13 as u16) << 4) | (((touch12 & 0b11110000) >> 4) as u16);
    


    let mut gyrox = ((gyroX1 as u16) << 8) | (gyroX0 as u16);
    if gyrox > 0x7FFF {gyrox -= 0b1000000000000000;}
    let mut gyroy = ((gyroY1 as u16) << 8) | (gyroY0 as u16);
    if gyroy > 0x7FFF {gyroy -= 0b1000000000000000;}
    let mut gyroz = ((gyroZ1 as u16) << 8) | (gyroZ0 as u16);
    if gyroz > 0x7FFF {gyroz -= 0b1000000000000000;}
    let mut accelx = ((accelX1 as u16) << 8) | (accelX0 as u16);
    if accelx > 0x7FFF {accelx -= 0b1000000000000000;}
    let mut accely = ((accelY1 as u16) << 8) | (accelY0 as u16);
    if accely > 0x7FFF {accely -= 0b1000000000000000;}
    let mut accelz = ((accelZ1 as u16) << 8) | (accelZ0 as u16);
    if accelz > 0x7FFF {accelz -= 0b1000000000000000;}
    
    let batteryLevelPercent = battery0 & 0b00001111;
    let batteryFull = !!(battery0 & 0b00100000);
    let batteryCharging = !!(battery1 & 0b00001000);



  
    if key as i32 == button_value::L2 as i32 { return axes4 as i32;
    }else if key as i32 == button_value::R2 as i32 { return axes5 as i32;
    }else if key as i32 == button_value::STICK_L_X as i32 { return (axes0 as i32) - 127;
    }else if key as i32 == button_value::STICK_L_Y as i32 { return (axes1 as i32) - 127;
    }else if key as i32 == button_value::STICK_R_X as i32 { return (axes2 as i32) - 127;
    }else if key as i32 == button_value::STICK_R_Y as i32 { return (axes3 as i32) - 127;
    }else if key as i32 == button_value::TOUCHPOINT_0_ID as i32 { return touch0id as i32;
    }else if key as i32 == button_value::TOUCHPOINT_1_ID as i32 { return touch1id as i32;
    }else if key as i32 == button_value::TOUCHPOINT_0_X as i32 { return touch0x as i32;
    }else if key as i32 == button_value::TOUCHPOINT_0_Y as i32 { return touch0y as i32;
    }else if key as i32 == button_value::TOUCHPOINT_1_X as i32 { return touch1x as i32;
    }else if key as i32 == button_value::TOUCHPOINT_1_Y as i32 { return touch1y as i32;
    }else if key as i32 == button_value::GYRO_X as i32 { return gyrox as i32;
    }else if key as i32 == button_value::GYRO_Y as i32 { return gyroy as i32;
    }else if key as i32 == button_value::GYRO_Z as i32 { return gyroz as i32;
    }else if key as i32 == button_value::ACCELEROMETER_X as i32 { return accelx as i32;
    }else if key as i32 == button_value::ACCELEROMETER_Y as i32 { return accely as i32;
    }else if key as i32 == button_value::ACCELEROMETER_Z as i32 { return accelz as i32;
    }else if key as i32 == button_value::BATTERY_LEVEL as i32 { return batteryLevelPercent as i32;
    }else if key as i32 == button_value::L2_TRIGGER_STATE as i32 { println!("WARNING `L2_TRIGGER_STATE` has not yet been implemented");return -1;
    }else if key as i32 == button_value::R2_TRIGGER_STATE as i32 { println!("WARNING `R2_TRIGGER_STATE` has not yet been implemented");return -1;
    }else {
      println!("MODE::USB 0x01    unknown key  {} ", key);
      return -1;
    }
  }else if mode as i32 == mode_type::Bluetooth as i32 {
    if buffer[0] == 0b00000001 {

      let axes0 = buffer[0+1];
      let axes1 = buffer[1+1];
      let axes2 = buffer[2+1];
      let axes3 = buffer[3+1];
      // let buttons0 = buffer[4+1];
      // let buttons1 = buffer[5+1];
      // let buttons2 = buffer[6+1];
      let axes4 = buffer[7+1];
      let axes5 = buffer[8+1];
        

    // let touch0active = !(touch00 & 0b10000000);
    // let touch0id = (touch00 & 0b01111111);
    // let touch0x = (((touch02 & 0b00001111) as u16) << 8) | (touch01 as u16);
    // let touch0y = ((touch03 as u16) << 4) | (((touch02 & 0b11110000) >> 4) as u16);
    // let touch1active = !(touch10 & 0b10000000);
    // let touch1id = (touch10 & 0b01111111);
    // let touch1x = (((touch12 & 0b00001111) as u16) << 8) | (touch11 as u16);
    // let touch1y = ((touch13 as u16) << 4) | (((touch12 & 0b11110000) >> 4) as u16);
    
      
    if key as i32 == button_value::L2 as i32 { return axes4 as i32;
    }else if key as i32 == button_value::R2 as i32 { return axes5 as i32;
    }else if key as i32 == button_value::STICK_L_X as i32 { return (axes0 as i32) - 127;
    }else if key as i32 == button_value::STICK_L_Y as i32 { return (axes1 as i32) - 127;
    }else if key as i32 == button_value::STICK_R_X as i32 { return (axes2 as i32) - 127;
    }else if key as i32 == button_value::STICK_R_Y as i32 { return (axes3 as i32) - 127;
    }else{
      println!(" MODE::Bluetooth 0x01    unknown key  {} ", key as i32);
      return -1;
    }



    }else if buffer[0] == 0b00110001 {
      let oef = 1;
      // byte 0?
      let axes0 = buffer[1+oef];
      let axes1 = buffer[2+oef];
      let axes2 = buffer[3+oef];
      let axes3 = buffer[4+oef];
      let axes4 = buffer[5+oef];
      let axes5 = buffer[6+oef];
      // byte 7?
      let buttons0 = buffer[8+oef];
      let buttons1 = buffer[9+oef];
      let buttons2 = buffer[10+oef];
      // byte 11?
      let timestamp0 = buffer[12+oef];
      let timestamp1 = buffer[13+oef];
      let timestamp2 = buffer[14+oef];
      let timestamp3 = buffer[15+oef];
      let gyroX0 = buffer[16+oef];
      let gyroX1 = buffer[17+oef];
      let gyroY0 = buffer[18+oef];
      let gyroY1 = buffer[19+oef];
      let gyroZ0 = buffer[20+oef];
      let gyroZ1 = buffer[21+oef];
      let accelX0 = buffer[22+oef];
      let accelX1 = buffer[23+oef];
      let accelY0 = buffer[24+oef];
      let accelY1 = buffer[25+oef];
      let accelZ0 = buffer[26+oef];
      let accelZ1 = buffer[27+oef];
      // bytes 28-32?
      let touch00 = buffer[33+oef];
      let touch01 = buffer[34+oef];
      let touch02 = buffer[35+oef];
      let touch03 = buffer[36+oef];
      let touch10 = buffer[37+oef];
      let touch11 = buffer[38+oef];
      let touch12 = buffer[39+oef];
      let touch13 = buffer[40+oef];
      // byte 41?
      let r2feedback = buffer[42+oef];
      let l2feedback = buffer[43+oef];
      // bytes 44-52?
      let battery0 = buffer[53+oef];
      let battery1 = buffer[54+oef];
      // bytes 55-76?



      let touch0active = !(touch00 & 0b10000000);
      let touch0id = (touch00 & 0b01111111);
      let touch0x = (((touch02 & 0b00001111) as u16) << 8) | (touch01 as u16);
      let touch0y = ((touch03 as u16) << 4) | (((touch02 & 0b11110000) >> 4) as u16);
      let touch1active = !(touch10 & 0b10000000);
      let touch1id = (touch10 & 0b01111111);
      let touch1x = (((touch12 & 0b00001111) as u16) << 8) | (touch11 as u16);
      let touch1y = ((touch13 as u16) << 4) | (((touch12 & 0b11110000) >> 4) as u16);
        
      let mut gyrox = ((gyroX1 as u16) << 8) | (gyroX0 as u16);
      if gyrox > 0x7FFF {gyrox -= 0b1000000000000000;}
      let mut gyroy = ((gyroY1 as u16) << 8) | (gyroY0 as u16);
      if gyroy > 0x7FFF {gyroy -= 0b1000000000000000;}
      let mut gyroz = ((gyroZ1 as u16) << 8) | (gyroZ0 as u16);
      if gyroz > 0x7FFF {gyroz -= 0b1000000000000000;}
      let mut accelx = ((accelX1 as u16) << 8) | (accelX0 as u16);
      if accelx > 0x7FFF {accelx -= 0b1000000000000000;}
      let mut accely = ((accelY1 as u16) << 8) | (accelY0 as u16);
      if accely > 0x7FFF {accely -= 0b1000000000000000;}
      let mut accelz = ((accelZ1 as u16) << 8) | (accelZ0 as u16);
      if accelz > 0x7FFF {accelz -= 0b1000000000000000;}
      
      let batteryLevelPercent = battery0 & 0b00001111;
      let batteryFull = !!(battery0 & 0b00100000);
      let batteryCharging = !!(battery1 & 0b00001000);

      if key as i32 == button_value::L2 as i32 { return axes4 as i32;
      }else if key as i32 == button_value::R2 as i32 { return axes5 as i32;
      }else if key as i32 == button_value::STICK_L_X as i32 { return (axes0 as i32) - 127;
      }else if key as i32 == button_value::STICK_L_Y as i32 { return (axes1 as i32) - 127;
      }else if key as i32 == button_value::STICK_R_X as i32 { return (axes2 as i32) - 127;
      }else if key as i32 == button_value::STICK_R_Y as i32 { return (axes3 as i32) - 127;
      }else if key as i32 == button_value::TOUCHPOINT_0_ID as i32 { return touch0id as i32;
      }else if key as i32 == button_value::TOUCHPOINT_1_ID as i32 { return touch1id as i32;
      }else if key as i32 == button_value::TOUCHPOINT_0_X as i32 { return touch0x as i32;
      }else if key as i32 == button_value::TOUCHPOINT_0_Y as i32 { return touch0y as i32;
      }else if key as i32 == button_value::TOUCHPOINT_1_X as i32 { return touch1x as i32;
      }else if key as i32 == button_value::TOUCHPOINT_1_Y as i32 { return touch1y as i32;
      }else if key as i32 == button_value::GYRO_X as i32 { return gyrox as i32;
      }else if key as i32 == button_value::GYRO_Y as i32 { return gyroy as i32;
      }else if key as i32 == button_value::GYRO_Z as i32 { return gyroz as i32;
      }else if key as i32 == button_value::ACCELEROMETER_X as i32 { return accelx as i32;
      }else if key as i32 == button_value::ACCELEROMETER_Y as i32 { return accely as i32;
      }else if key as i32 == button_value::ACCELEROMETER_Z as i32 { return accelz as i32;
      }else if key as i32 == button_value::BATTERY_LEVEL as i32 { return batteryLevelPercent as i32;
      }else if key as i32 == button_value::L2_TRIGGER_STATE as i32 { println!("WARNING `L2_TRIGGER_STATE` has not yet been implemented");return -1;
      }else if key as i32 == button_value::R2_TRIGGER_STATE as i32 { println!("WARNING `R2_TRIGGER_STATE` has not yet been implemented");return -1;
      }else{
        println!(" MODE::Bluetooth 0x31    unknown key  {} ", key as i32);
        return -1;
      }
    }else{
      println!(" mode Bluetooth has invalid report id  {:2x?}  ( {:3?} )", buffer[0], buffer[0]);
    }
  }


  return 0;
  // }
}






















// // ALL CODE BELOW THIS COMMENT IS  INCOMPLETE AND HAS NOT BEEN FINISHED YET

// pub mod write {
//     use super::mode_type;

//   #[repr(C)]
//   #[derive(Copy, Clone, Debug)]
//   pub enum selector_key {
//     COMPATIBLE_VIBRATION = 0x00,
//     HAPTICS_SELECT = 0x01,
//     MIC_MUTE_LED_CONTROL_ENABLE = 0x02,
//     POWER_SAVE_CONTROL_ENABLE = 0x03,
//     LIGHTBAR_CONTROL_ENABLE = 0x04,
//     RELEASE_LEDS = 0x05,
//     PLAYER_INDICATOR_CONTROL_ENABLE = 0x06,
//     MOTOR_LEFT = 0x07,
//     MOTOR_RIGHT = 0x08,
//     MUTE_LED = 0x09,
//     POWER_SAVE_CONTROL_MIC_MUTE = 0x0a,
//     LIGHTBAR_SETUP_CONTROL_ENABLE = 0x0b,
//     LEDs = 0x0c,
//     LED_RED = 0x0d,
//     LED_GREEN = 0x0e,
//     LED_BLUE = 0x0f,
//     // _______ = 0x10,
//     // _______ = 0x11,
//     // _______ = 0x12,
//     // _______ = 0x13,
//     // _______ = 0x14,
//     // _______ = 0x15,
//     // _______ = 0x16,
//     // _______ = 0x17,
//     // _______ = 0x18,
//     // _______ = 0x19,
//     // _______ = 0x1a,
//     // _______ = 0x1b,
//     // _______ = 0x1c,
//     // _______ = 0x1d,
//     // _______ = 0x1e,
//     // _______ = 0x1f,

//   }
//   fn gt_byte_key_map(){
//     let mut kbyte = [0u8; 128];
//     let mut bit_k: [u8; 128] = [0u8; 128]; // note: if bit == 8 :   all 
//     kbyte[selector_key::COMPATIBLE_VIBRATION as usize] = 0;
//     bit_k[selector_key::COMPATIBLE_VIBRATION as usize] = 0; // bit
//     kbyte[selector_key::HAPTICS_SELECT as usize] = 0;
//     bit_k[selector_key::HAPTICS_SELECT as usize] = 1; // bit

//     kbyte[selector_key::MIC_MUTE_LED_CONTROL_ENABLE as usize] = 1;
//     bit_k[selector_key::MIC_MUTE_LED_CONTROL_ENABLE as usize] = 0; // bit
//     kbyte[selector_key::POWER_SAVE_CONTROL_ENABLE as usize] = 1;
//     bit_k[selector_key::POWER_SAVE_CONTROL_ENABLE as usize] = 1; // bit
//     kbyte[selector_key::LIGHTBAR_CONTROL_ENABLE as usize] = 1;
//     bit_k[selector_key::LIGHTBAR_CONTROL_ENABLE as usize] = 2; // bit
//     kbyte[selector_key::RELEASE_LEDS as usize] = 1;
//     bit_k[selector_key::RELEASE_LEDS as usize] = 3; // bit
//     kbyte[selector_key::PLAYER_INDICATOR_CONTROL_ENABLE as usize] = 1;
//     bit_k[selector_key::PLAYER_INDICATOR_CONTROL_ENABLE as usize] = 4; // bit

//     kbyte[selector_key::MOTOR_RIGHT as usize] = 2;
//     bit_k[selector_key::MOTOR_RIGHT as usize] = 8; // bit

//     kbyte[selector_key::MOTOR_LEFT as usize] = 3;
//     bit_k[selector_key::MOTOR_LEFT as usize] = 8; // bit

//     kbyte[selector_key::MUTE_LED as usize] = 8;
//     bit_k[selector_key::MUTE_LED as usize] = 7; // bit // tek all, but since only 0|1 can use bit7 only instead

//     kbyte[selector_key::POWER_SAVE_CONTROL_MIC_MUTE as usize] = 9;
//     bit_k[selector_key::POWER_SAVE_CONTROL_MIC_MUTE as usize] = 4; // bit

//     kbyte[selector_key::LIGHTBAR_SETUP_CONTROL_ENABLE as usize] = 39;
//     bit_k[selector_key::LIGHTBAR_SETUP_CONTROL_ENABLE as usize] = 1; // bit

//     kbyte[selector_key::LEDs as usize] = 41;
//     bit_k[selector_key::LEDs as usize] = 0; // bit

//     kbyte[selector_key::LED_RED as usize] = 44;
//     bit_k[selector_key::LED_RED as usize] = 8; // bit
//     kbyte[selector_key::LED_GREEN as usize] = 45;
//     bit_k[selector_key::LED_GREEN as usize] = 8; // bit
//     kbyte[selector_key::LED_BLUE as usize] = 46;
//     bit_k[selector_key::LED_BLUE as usize] = 8; // bit

//     // kbyte[selector_key::_______ as usize] = 1;
//     // bit_k[selector_key::_______ as usize] = 0; // bit
//   }


//   pub fn create_output(mut outputSeq_:i8, mode:mode_type ) -> [u8; 78]{
//     // write buffer has to be `78` (77) bits because https://nondebug.github.io/dualsense/dualsense-explorer.html line 895
//     // write-buffer for usb mode:         [u8; 48] 
//     // write-buffer for bluetooth mode:   [u8; 78] 
//     let mut reportData: [u8; 78] = [0u8; 78];

//     if mode as i32 == mode_type::Usb as i32{
//       let reportId = 0x02;
//       reportData[0] = reportId;

//     }else if mode as i32 == mode_type::Bluetooth as i32{
//       let reportId = 0x31;
//       reportData[0] = reportId;

//       // seq_tag
//       reportData[0] = (outputSeq_ as u8) << 4 ;
//       if outputSeq_+1 == 16 {
//         outputSeq_ = 0;
//       }

//       // tag
//       reportData[1] = 0x10; // DS_OUTPUT_TAG
      
//       println!(" mode Bluetooth is not yet implemented");
//     }

//     return reportData;
//   }


//   pub fn prepare_output( output: &mut [u8], outputSeq_: &mut i8, mode:mode_type ){
//     // write buffer has to be `78` (77) bits because https://nondebug.github.io/dualsense/dualsense-explorer.html line 895
//     // write-buffer for usb mode:         [u8; 48] 
//     // write-buffer for bluetooth mode:   [u8; 78] 

//     if mode as i32 == mode_type::Usb as i32{

//     }else if mode as i32 == mode_type::Bluetooth as i32{

//       // seq_tag
//       let mini = *outputSeq_ as u8;
//       output[0] = (mini as u8) << 4 ;
//       if (*outputSeq_)+1 == 16 {
//         *outputSeq_ = 0;
//       }else{
//         *outputSeq_ + 1;
//       }

//       // tag
//       output[1] = 0x10; // DS_OUTPUT_TAG
      
//       println!(" mode Bluetooth is not yet implemented");
//     }

//     // return reportData; 
//   }


//   pub fn set_bit( output: &mut [u8], key: selector_key, mode:mode_type ) {
//     // write buffer has to be `78` (77) bits because https://nondebug.github.io/dualsense/dualsense-explorer.html line 895
//     // write-buffer for usb mode:         [u8; 48] 
//     // write-buffer for bluetooth mode:   [u8; 78] 
//     let mut index_offset = 1;
//     // NOTE:  we offset by 1, because we have prepend the report-id, unlike the https://nondebug.github.io/dualsense/dualsense-explorer.html does
//     //        this was the byte indices match those of https://nondebug.github.io/dualsense/dualsense-explorer.html
//     if mode as i32 == mode_type::Usb as i32{
//       // reportData = new Uint8Array(47);
//       index_offset = 1; // DataView(reportData.buffer, 0, 47);
//     }else if mode as i32 == mode_type::Bluetooth as i32{
//       // reportData = new Uint8Array(77);
//       index_offset = 1+2; // DataView(reportData.buffer, 2, 47);
//     }


//   }


// }

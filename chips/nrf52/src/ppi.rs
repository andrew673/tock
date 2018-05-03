use kernel::common::VolatileCell;

pub const PPI_BASE: usize = 0x4001F000;
#[repr(C)]
pub struct PPIRegs {
    pub tasks_chg0_en: VolatileCell<u32>,
    pub tasks_chg0_dis: VolatileCell<u32>,
    pub tasks_chg1_en: VolatileCell<u32>,
    pub tasks_chg1_dis: VolatileCell<u32>,
    pub tasks_chg2_en: VolatileCell<u32>,
    pub tasks_chg2_dis: VolatileCell<u32>,
    pub tasks_chg3_en: VolatileCell<u32>,
    pub tasks_chg3_dis: VolatileCell<u32>,
    pub tasks_chg4_en: VolatileCell<u32>,
    pub tasks_chg4_dis: VolatileCell<u32>,
    pub tasks_chg5_en: VolatileCell<u32>,
    pub tasks_chg5_dis: VolatileCell<u32>,
    _reserved1: [u32; 308],
    pub chen: VolatileCell<u32>,
    pub chenset: VolatileCell<u32>,
    pub chenclr: VolatileCell<u32>,
    pub ch0_eep: VolatileCell<u32>,
    pub ch0_tep: VolatileCell<u32>,
    pub ch1_eep: VolatileCell<u32>,
    pub ch1_tep: VolatileCell<u32>,
    pub ch2_eep: VolatileCell<u32>,
    pub ch2_tep: VolatileCell<u32>,
    pub ch3_eep: VolatileCell<u32>,
    pub ch3_tep: VolatileCell<u32>,
    pub ch4_eep: VolatileCell<u32>,
    pub ch4_tep: VolatileCell<u32>,
    pub ch5_eep: VolatileCell<u32>,
    pub ch5_tep: VolatileCell<u32>,
    pub ch6_eep: VolatileCell<u32>,
    pub ch6_tep: VolatileCell<u32>,
    pub ch7_eep: VolatileCell<u32>,
    pub ch7_tep: VolatileCell<u32>,
    pub ch8_eep: VolatileCell<u32>,
    pub ch8_tep: VolatileCell<u32>,
    pub ch9_eep: VolatileCell<u32>,
    pub ch9_tep: VolatileCell<u32>,
    pub ch10_eep: VolatileCell<u32>,
    pub ch10_tep: VolatileCell<u32>,
    pub ch11_eep: VolatileCell<u32>,
    pub ch11_tep: VolatileCell<u32>,
    pub ch12_eep: VolatileCell<u32>,
    pub ch12_tep: VolatileCell<u32>,
    pub ch13_eep: VolatileCell<u32>,
    pub ch13_tep: VolatileCell<u32>,
    pub ch14_eep: VolatileCell<u32>,
    pub ch14_tep: VolatileCell<u32>,
    pub ch15_eep: VolatileCell<u32>,
    pub ch15_tep: VolatileCell<u32>,
    pub ch16_eep: VolatileCell<u32>,
    pub ch16_tep: VolatileCell<u32>,
    pub ch17_eep: VolatileCell<u32>,
    pub ch17_tep: VolatileCell<u32>,
    pub ch18_eep: VolatileCell<u32>,
    pub ch18_tep: VolatileCell<u32>,
    pub ch19_eep: VolatileCell<u32>,
    pub ch19_tep: VolatileCell<u32>,
    _reserved2: [u32; 148],
    pub chg: [VolatileCell<u32>; 6],
    _reserved3: [u32; 62],
    pub fork_tep: [VolatileCell<u32>; 32],
}

pub struct PPIStruct {
    regs: *const PPIRegs,
}

pub static mut PPI: PPIStruct = PPIStruct::new();

impl PPIStruct {
    pub const fn new() -> PPIStruct {
        PPIStruct {
            regs: PPI_BASE as *const PPIRegs,
        }
    }

    pub fn enable(&self, pins: u32) {
        let regs = unsafe { &*self.regs };
        regs.chenset.set(pins);
    }

    pub fn disable(&self, pins: u32) {
        let regs = unsafe { &*self.regs };
        regs.chenclr.set(pins);
    }
}

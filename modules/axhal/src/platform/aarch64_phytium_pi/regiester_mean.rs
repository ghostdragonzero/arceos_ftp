/*
这个方式来定义寄存器是使用的这个库来封装一些功能
需要在cargo.toml函数里的依赖添加这个create 包
 */
use tock_registers::interfaces::ReadWriteable;
use tock_registers::interfaces::Readable;
use tock_registers::interfaces::Writeable;
use tock_registers::register_bitfields;
use tock_registers::register_structs;
use tock_registers::registers::ReadOnly;
use tock_registers::registers::ReadWrite;

//定义一个设备寄存器
/*
(0x0000 => wrr:ReadWrite<u32, DWORD::Register>)
偏移量     寄存器名称  寄存器的读写性  寄存器的长度  寄存器内的数据类型
*/
register_structs! {
    pub WdtRegisters {
        (0x0000 => wrr:ReadWrite<u32, DWORD::Register>),
        (0x0004 => _resv:[u8;0xfc8]),
        (0x0fcc => iidr:ReadWrite<u32, DWORD::Register>),
        (0x0fd0 => _resv4:[u8;48]),
        (0x1000 =>wcs:ReadWrite<u32,CONTROL::Register>),
        (0x1004 => _resv2:[u8;0x4]),
        (0x1008 =>wor:ReadWrite<u32,DWORD::Register>),
        (0x100c => _resv3:[u8;0xc]),
        (0x1018 => @END),
    }
}
/*
自定义的寄存器的值类型，用于个更加易读的设定寄存器的值
以CONTROL为例子
CONTROL[WDR_EN OFFSET(0) NUMBITS(1), WS0  OFFSET(1) NUMBITS(1), WS1 OFFSET(2) NUMBITS(1)]
offset 对应偏移量 numbits 代表这个数据需要几个位长
实际使用中可以这样使用
self.wcs.modify(CONTROL::WDR_EN.val(1));
代码更易读
告诉你这一位是使能的作用

 */
register_bitfields![u32, DWORD[DATA OFFSET(0) NUMBITS(32)],
CONTROL[WDR_EN OFFSET(0) NUMBITS(1), WS0  OFFSET(1) NUMBITS(1), WS1 OFFSET(2) NUMBITS(1)]
];

/*
定义这个设备的一些功能函数通常包括
new  通过提供设备的基地址来创建这个设备
init 使能这个设备需要进行的写寄存器的操作

后面就是各种功能函数

 */
impl WdtRegisters {
    pub fn new(base: usize) -> &'static mut Self {
        unsafe { &mut *(base as *mut WdtRegisters) }
    }
}
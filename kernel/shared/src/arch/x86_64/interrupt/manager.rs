/*! x86_64 interrupt management implementation */

use x86_64::{
    instructions::{
        interrupts,
        segmentation::set_cs,
        tables::load_tss
    },
    structures::{
        gdt::{
            Descriptor,
            GlobalDescriptorTable
        },
        idt::{
            InterruptDescriptorTable,
            InterruptStackFrame as X64InterruptStackFrame,
            PageFaultErrorCode
        },
        tss::TaskStateSegment
    }
};

use crate::{
    addr::{
        virt::VirtAddr,
        Address
    },
    arch::x86_64::interrupt::stack_frame::HwInterruptStackFrame,
    interrupt::{
        manager::{
            HwInterruptManagerBase,
            InterruptManagerException,
            InterruptManagerHandlers,
            InterruptReason
        },
        stack_frame::InterruptStackFrame
    },
    logger::debug
};

/**
 * Global static mutable reference to the global `InterruptManager`'s
 * `InterruptManagerHandlers`, enabled when called
 * `InterruptManager::enable_as_global()`
 */
static mut INTERRUPT_HANDLERS: Option<&'static mut InterruptManagerHandlers> = None;

static mut BSP_INIT_TSS: TaskStateSegment = TaskStateSegment::new();
static mut BSP_INIT_GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

/**
 * x86_64 `HwInterruptManagerBase` implementation
 */
pub struct HwInterruptManager {
    m_idt: InterruptDescriptorTable
}

impl HwInterruptManager {
    /**
     * Constructs an empty `HwInterruptManager`
     */
    const fn new() -> Self {
        Self { m_idt: InterruptDescriptorTable::new() }
    }

    /**  
     * Handles the hardware exception
     */
    fn hw_except_handler(stack_frame: &mut X64InterruptStackFrame,
                         exception: InterruptManagerException) {
        if let Some(intr_handlers) = unsafe { INTERRUPT_HANDLERS.as_mut() } {
            let hw_stack_frame = HwInterruptStackFrame::wrap_ptr(stack_frame);
            intr_handlers.handle_hw_intr_callback(InterruptStackFrame::new(hw_stack_frame),
                                                  InterruptReason::Exception(exception));
        }
    }

    /**
     * Handles the hardware interrupt
     */
    fn hw_intr_handler(stack_frame: &mut X64InterruptStackFrame, intr_num: usize) {
        if let Some(intr_handlers) = unsafe { INTERRUPT_HANDLERS.as_mut() } {
            let hw_stack_frame = HwInterruptStackFrame::wrap_ptr(stack_frame);
            intr_handlers.handle_hw_intr_callback(InterruptStackFrame::new(hw_stack_frame),
                                                  InterruptReason::Interrupt(intr_num));
        }
    }
}

impl HwInterruptManagerBase for HwInterruptManager {
    const CONST_NEW: Self = HwInterruptManager::new();
    const INTR_COUNT: usize = 256 - Self::INTR_OFFSET;
    const INTR_OFFSET: usize = 32;

    unsafe fn enable_as_global(&'static mut self,
                               intr_handlers: &'static mut InterruptManagerHandlers) {
        /* store the given interrupt handler */
        if INTERRUPT_HANDLERS.is_none() {
            INTERRUPT_HANDLERS = Some(intr_handlers);
        } else {
            panic!("Loading HwInterruptManager twice...");
        }

        /* initialize each IDT field with the right hardware handler */
        {
            self.m_idt.double_fault.set_handler_fn(except_double_fault);
            self.m_idt.divide_error.set_handler_fn(except_divide_error);
            self.m_idt.invalid_opcode.set_handler_fn(except_invalid_op);
            self.m_idt.page_fault.set_handler_fn(except_page_fault);
            self.m_idt.simd_floating_point.set_handler_fn(except_floating_point);
            self.m_idt.x87_floating_point.set_handler_fn(except_floating_point);

            self.m_idt[Self::INTR_OFFSET].set_handler_fn(intr_handler_0);
            self.m_idt[Self::INTR_OFFSET + 1].set_handler_fn(intr_handler_1);
            self.m_idt[Self::INTR_OFFSET + 2].set_handler_fn(intr_handler_2);
            self.m_idt[Self::INTR_OFFSET + 3].set_handler_fn(intr_handler_3);
            self.m_idt[Self::INTR_OFFSET + 4].set_handler_fn(intr_handler_4);
            self.m_idt[Self::INTR_OFFSET + 5].set_handler_fn(intr_handler_5);
            self.m_idt[Self::INTR_OFFSET + 6].set_handler_fn(intr_handler_6);
            self.m_idt[Self::INTR_OFFSET + 7].set_handler_fn(intr_handler_7);
            self.m_idt[Self::INTR_OFFSET + 8].set_handler_fn(intr_handler_8);
            self.m_idt[Self::INTR_OFFSET + 9].set_handler_fn(intr_handler_9);
            self.m_idt[Self::INTR_OFFSET + 10].set_handler_fn(intr_handler_10);
            self.m_idt[Self::INTR_OFFSET + 11].set_handler_fn(intr_handler_11);
            self.m_idt[Self::INTR_OFFSET + 12].set_handler_fn(intr_handler_12);
            self.m_idt[Self::INTR_OFFSET + 13].set_handler_fn(intr_handler_13);
            self.m_idt[Self::INTR_OFFSET + 14].set_handler_fn(intr_handler_14);
            self.m_idt[Self::INTR_OFFSET + 15].set_handler_fn(intr_handler_15);
            self.m_idt[Self::INTR_OFFSET + 16].set_handler_fn(intr_handler_16);
            self.m_idt[Self::INTR_OFFSET + 17].set_handler_fn(intr_handler_17);
            self.m_idt[Self::INTR_OFFSET + 18].set_handler_fn(intr_handler_18);
            self.m_idt[Self::INTR_OFFSET + 19].set_handler_fn(intr_handler_19);
            self.m_idt[Self::INTR_OFFSET + 20].set_handler_fn(intr_handler_20);
            self.m_idt[Self::INTR_OFFSET + 21].set_handler_fn(intr_handler_21);
            self.m_idt[Self::INTR_OFFSET + 22].set_handler_fn(intr_handler_22);
            self.m_idt[Self::INTR_OFFSET + 23].set_handler_fn(intr_handler_23);
            self.m_idt[Self::INTR_OFFSET + 24].set_handler_fn(intr_handler_24);
            self.m_idt[Self::INTR_OFFSET + 25].set_handler_fn(intr_handler_25);
            self.m_idt[Self::INTR_OFFSET + 26].set_handler_fn(intr_handler_26);
            self.m_idt[Self::INTR_OFFSET + 27].set_handler_fn(intr_handler_27);
            self.m_idt[Self::INTR_OFFSET + 28].set_handler_fn(intr_handler_28);
            self.m_idt[Self::INTR_OFFSET + 29].set_handler_fn(intr_handler_29);
            self.m_idt[Self::INTR_OFFSET + 30].set_handler_fn(intr_handler_30);
            self.m_idt[Self::INTR_OFFSET + 31].set_handler_fn(intr_handler_31);
            self.m_idt[Self::INTR_OFFSET + 32].set_handler_fn(intr_handler_32);
            self.m_idt[Self::INTR_OFFSET + 33].set_handler_fn(intr_handler_33);
            self.m_idt[Self::INTR_OFFSET + 34].set_handler_fn(intr_handler_34);
            self.m_idt[Self::INTR_OFFSET + 35].set_handler_fn(intr_handler_35);
            self.m_idt[Self::INTR_OFFSET + 36].set_handler_fn(intr_handler_36);
            self.m_idt[Self::INTR_OFFSET + 37].set_handler_fn(intr_handler_37);
            self.m_idt[Self::INTR_OFFSET + 38].set_handler_fn(intr_handler_38);
            self.m_idt[Self::INTR_OFFSET + 39].set_handler_fn(intr_handler_39);
            self.m_idt[Self::INTR_OFFSET + 40].set_handler_fn(intr_handler_40);
            self.m_idt[Self::INTR_OFFSET + 41].set_handler_fn(intr_handler_41);
            self.m_idt[Self::INTR_OFFSET + 42].set_handler_fn(intr_handler_42);
            self.m_idt[Self::INTR_OFFSET + 43].set_handler_fn(intr_handler_43);
            self.m_idt[Self::INTR_OFFSET + 44].set_handler_fn(intr_handler_44);
            self.m_idt[Self::INTR_OFFSET + 45].set_handler_fn(intr_handler_45);
            self.m_idt[Self::INTR_OFFSET + 46].set_handler_fn(intr_handler_46);
            self.m_idt[Self::INTR_OFFSET + 47].set_handler_fn(intr_handler_47);
            self.m_idt[Self::INTR_OFFSET + 48].set_handler_fn(intr_handler_48);
            self.m_idt[Self::INTR_OFFSET + 49].set_handler_fn(intr_handler_49);
            self.m_idt[Self::INTR_OFFSET + 50].set_handler_fn(intr_handler_50);
            self.m_idt[Self::INTR_OFFSET + 51].set_handler_fn(intr_handler_51);
            self.m_idt[Self::INTR_OFFSET + 52].set_handler_fn(intr_handler_52);
            self.m_idt[Self::INTR_OFFSET + 53].set_handler_fn(intr_handler_53);
            self.m_idt[Self::INTR_OFFSET + 54].set_handler_fn(intr_handler_54);
            self.m_idt[Self::INTR_OFFSET + 55].set_handler_fn(intr_handler_55);
            self.m_idt[Self::INTR_OFFSET + 56].set_handler_fn(intr_handler_56);
            self.m_idt[Self::INTR_OFFSET + 57].set_handler_fn(intr_handler_57);
            self.m_idt[Self::INTR_OFFSET + 58].set_handler_fn(intr_handler_58);
            self.m_idt[Self::INTR_OFFSET + 59].set_handler_fn(intr_handler_59);
            self.m_idt[Self::INTR_OFFSET + 60].set_handler_fn(intr_handler_60);
            self.m_idt[Self::INTR_OFFSET + 61].set_handler_fn(intr_handler_61);
            self.m_idt[Self::INTR_OFFSET + 62].set_handler_fn(intr_handler_62);
            self.m_idt[Self::INTR_OFFSET + 63].set_handler_fn(intr_handler_63);
            self.m_idt[Self::INTR_OFFSET + 64].set_handler_fn(intr_handler_64);
            self.m_idt[Self::INTR_OFFSET + 65].set_handler_fn(intr_handler_65);
            self.m_idt[Self::INTR_OFFSET + 66].set_handler_fn(intr_handler_66);
            self.m_idt[Self::INTR_OFFSET + 67].set_handler_fn(intr_handler_67);
            self.m_idt[Self::INTR_OFFSET + 68].set_handler_fn(intr_handler_68);
            self.m_idt[Self::INTR_OFFSET + 69].set_handler_fn(intr_handler_69);
            self.m_idt[Self::INTR_OFFSET + 70].set_handler_fn(intr_handler_70);
            self.m_idt[Self::INTR_OFFSET + 71].set_handler_fn(intr_handler_71);
            self.m_idt[Self::INTR_OFFSET + 72].set_handler_fn(intr_handler_72);
            self.m_idt[Self::INTR_OFFSET + 73].set_handler_fn(intr_handler_73);
            self.m_idt[Self::INTR_OFFSET + 74].set_handler_fn(intr_handler_74);
            self.m_idt[Self::INTR_OFFSET + 75].set_handler_fn(intr_handler_75);
            self.m_idt[Self::INTR_OFFSET + 76].set_handler_fn(intr_handler_76);
            self.m_idt[Self::INTR_OFFSET + 77].set_handler_fn(intr_handler_77);
            self.m_idt[Self::INTR_OFFSET + 78].set_handler_fn(intr_handler_78);
            self.m_idt[Self::INTR_OFFSET + 79].set_handler_fn(intr_handler_79);
            self.m_idt[Self::INTR_OFFSET + 80].set_handler_fn(intr_handler_80);
            self.m_idt[Self::INTR_OFFSET + 81].set_handler_fn(intr_handler_81);
            self.m_idt[Self::INTR_OFFSET + 82].set_handler_fn(intr_handler_82);
            self.m_idt[Self::INTR_OFFSET + 83].set_handler_fn(intr_handler_83);
            self.m_idt[Self::INTR_OFFSET + 84].set_handler_fn(intr_handler_84);
            self.m_idt[Self::INTR_OFFSET + 85].set_handler_fn(intr_handler_85);
            self.m_idt[Self::INTR_OFFSET + 86].set_handler_fn(intr_handler_86);
            self.m_idt[Self::INTR_OFFSET + 87].set_handler_fn(intr_handler_87);
            self.m_idt[Self::INTR_OFFSET + 88].set_handler_fn(intr_handler_88);
            self.m_idt[Self::INTR_OFFSET + 89].set_handler_fn(intr_handler_89);
            self.m_idt[Self::INTR_OFFSET + 90].set_handler_fn(intr_handler_90);
            self.m_idt[Self::INTR_OFFSET + 91].set_handler_fn(intr_handler_91);
            self.m_idt[Self::INTR_OFFSET + 92].set_handler_fn(intr_handler_92);
            self.m_idt[Self::INTR_OFFSET + 93].set_handler_fn(intr_handler_93);
            self.m_idt[Self::INTR_OFFSET + 94].set_handler_fn(intr_handler_94);
            self.m_idt[Self::INTR_OFFSET + 95].set_handler_fn(intr_handler_95);
            self.m_idt[Self::INTR_OFFSET + 96].set_handler_fn(intr_handler_96);
            self.m_idt[Self::INTR_OFFSET + 97].set_handler_fn(intr_handler_97);
            self.m_idt[Self::INTR_OFFSET + 98].set_handler_fn(intr_handler_98);
            self.m_idt[Self::INTR_OFFSET + 99].set_handler_fn(intr_handler_99);
            self.m_idt[Self::INTR_OFFSET + 100].set_handler_fn(intr_handler_100);
            self.m_idt[Self::INTR_OFFSET + 101].set_handler_fn(intr_handler_101);
            self.m_idt[Self::INTR_OFFSET + 102].set_handler_fn(intr_handler_102);
            self.m_idt[Self::INTR_OFFSET + 103].set_handler_fn(intr_handler_103);
            self.m_idt[Self::INTR_OFFSET + 104].set_handler_fn(intr_handler_104);
            self.m_idt[Self::INTR_OFFSET + 105].set_handler_fn(intr_handler_105);
            self.m_idt[Self::INTR_OFFSET + 106].set_handler_fn(intr_handler_106);
            self.m_idt[Self::INTR_OFFSET + 107].set_handler_fn(intr_handler_107);
            self.m_idt[Self::INTR_OFFSET + 108].set_handler_fn(intr_handler_108);
            self.m_idt[Self::INTR_OFFSET + 109].set_handler_fn(intr_handler_109);
            self.m_idt[Self::INTR_OFFSET + 110].set_handler_fn(intr_handler_110);
            self.m_idt[Self::INTR_OFFSET + 111].set_handler_fn(intr_handler_111);
            self.m_idt[Self::INTR_OFFSET + 112].set_handler_fn(intr_handler_112);
            self.m_idt[Self::INTR_OFFSET + 113].set_handler_fn(intr_handler_113);
            self.m_idt[Self::INTR_OFFSET + 114].set_handler_fn(intr_handler_114);
            self.m_idt[Self::INTR_OFFSET + 115].set_handler_fn(intr_handler_115);
            self.m_idt[Self::INTR_OFFSET + 116].set_handler_fn(intr_handler_116);
            self.m_idt[Self::INTR_OFFSET + 117].set_handler_fn(intr_handler_117);
            self.m_idt[Self::INTR_OFFSET + 118].set_handler_fn(intr_handler_118);
            self.m_idt[Self::INTR_OFFSET + 119].set_handler_fn(intr_handler_119);
            self.m_idt[Self::INTR_OFFSET + 120].set_handler_fn(intr_handler_120);
            self.m_idt[Self::INTR_OFFSET + 121].set_handler_fn(intr_handler_121);
            self.m_idt[Self::INTR_OFFSET + 122].set_handler_fn(intr_handler_122);
            self.m_idt[Self::INTR_OFFSET + 123].set_handler_fn(intr_handler_123);
            self.m_idt[Self::INTR_OFFSET + 124].set_handler_fn(intr_handler_124);
            self.m_idt[Self::INTR_OFFSET + 125].set_handler_fn(intr_handler_125);
            self.m_idt[Self::INTR_OFFSET + 126].set_handler_fn(intr_handler_126);
            self.m_idt[Self::INTR_OFFSET + 127].set_handler_fn(intr_handler_127);
            self.m_idt[Self::INTR_OFFSET + 128].set_handler_fn(intr_handler_128);
            self.m_idt[Self::INTR_OFFSET + 129].set_handler_fn(intr_handler_129);
            self.m_idt[Self::INTR_OFFSET + 130].set_handler_fn(intr_handler_130);
            self.m_idt[Self::INTR_OFFSET + 131].set_handler_fn(intr_handler_131);
            self.m_idt[Self::INTR_OFFSET + 132].set_handler_fn(intr_handler_132);
            self.m_idt[Self::INTR_OFFSET + 133].set_handler_fn(intr_handler_133);
            self.m_idt[Self::INTR_OFFSET + 134].set_handler_fn(intr_handler_134);
            self.m_idt[Self::INTR_OFFSET + 135].set_handler_fn(intr_handler_135);
            self.m_idt[Self::INTR_OFFSET + 136].set_handler_fn(intr_handler_136);
            self.m_idt[Self::INTR_OFFSET + 137].set_handler_fn(intr_handler_137);
            self.m_idt[Self::INTR_OFFSET + 138].set_handler_fn(intr_handler_138);
            self.m_idt[Self::INTR_OFFSET + 139].set_handler_fn(intr_handler_139);
            self.m_idt[Self::INTR_OFFSET + 140].set_handler_fn(intr_handler_140);
            self.m_idt[Self::INTR_OFFSET + 141].set_handler_fn(intr_handler_141);
            self.m_idt[Self::INTR_OFFSET + 142].set_handler_fn(intr_handler_142);
            self.m_idt[Self::INTR_OFFSET + 143].set_handler_fn(intr_handler_143);
            self.m_idt[Self::INTR_OFFSET + 144].set_handler_fn(intr_handler_144);
            self.m_idt[Self::INTR_OFFSET + 145].set_handler_fn(intr_handler_145);
            self.m_idt[Self::INTR_OFFSET + 146].set_handler_fn(intr_handler_146);
            self.m_idt[Self::INTR_OFFSET + 147].set_handler_fn(intr_handler_147);
            self.m_idt[Self::INTR_OFFSET + 148].set_handler_fn(intr_handler_148);
            self.m_idt[Self::INTR_OFFSET + 149].set_handler_fn(intr_handler_149);
            self.m_idt[Self::INTR_OFFSET + 150].set_handler_fn(intr_handler_150);
            self.m_idt[Self::INTR_OFFSET + 151].set_handler_fn(intr_handler_151);
            self.m_idt[Self::INTR_OFFSET + 152].set_handler_fn(intr_handler_152);
            self.m_idt[Self::INTR_OFFSET + 153].set_handler_fn(intr_handler_153);
            self.m_idt[Self::INTR_OFFSET + 154].set_handler_fn(intr_handler_154);
            self.m_idt[Self::INTR_OFFSET + 155].set_handler_fn(intr_handler_155);
            self.m_idt[Self::INTR_OFFSET + 156].set_handler_fn(intr_handler_156);
            self.m_idt[Self::INTR_OFFSET + 157].set_handler_fn(intr_handler_157);
            self.m_idt[Self::INTR_OFFSET + 158].set_handler_fn(intr_handler_158);
            self.m_idt[Self::INTR_OFFSET + 159].set_handler_fn(intr_handler_159);
            self.m_idt[Self::INTR_OFFSET + 160].set_handler_fn(intr_handler_160);
            self.m_idt[Self::INTR_OFFSET + 161].set_handler_fn(intr_handler_161);
            self.m_idt[Self::INTR_OFFSET + 162].set_handler_fn(intr_handler_162);
            self.m_idt[Self::INTR_OFFSET + 163].set_handler_fn(intr_handler_163);
            self.m_idt[Self::INTR_OFFSET + 164].set_handler_fn(intr_handler_164);
            self.m_idt[Self::INTR_OFFSET + 165].set_handler_fn(intr_handler_165);
            self.m_idt[Self::INTR_OFFSET + 166].set_handler_fn(intr_handler_166);
            self.m_idt[Self::INTR_OFFSET + 167].set_handler_fn(intr_handler_167);
            self.m_idt[Self::INTR_OFFSET + 168].set_handler_fn(intr_handler_168);
            self.m_idt[Self::INTR_OFFSET + 169].set_handler_fn(intr_handler_169);
            self.m_idt[Self::INTR_OFFSET + 170].set_handler_fn(intr_handler_170);
            self.m_idt[Self::INTR_OFFSET + 171].set_handler_fn(intr_handler_171);
            self.m_idt[Self::INTR_OFFSET + 172].set_handler_fn(intr_handler_172);
            self.m_idt[Self::INTR_OFFSET + 173].set_handler_fn(intr_handler_173);
            self.m_idt[Self::INTR_OFFSET + 174].set_handler_fn(intr_handler_174);
            self.m_idt[Self::INTR_OFFSET + 175].set_handler_fn(intr_handler_175);
            self.m_idt[Self::INTR_OFFSET + 176].set_handler_fn(intr_handler_176);
            self.m_idt[Self::INTR_OFFSET + 177].set_handler_fn(intr_handler_177);
            self.m_idt[Self::INTR_OFFSET + 178].set_handler_fn(intr_handler_178);
            self.m_idt[Self::INTR_OFFSET + 179].set_handler_fn(intr_handler_179);
            self.m_idt[Self::INTR_OFFSET + 180].set_handler_fn(intr_handler_180);
            self.m_idt[Self::INTR_OFFSET + 181].set_handler_fn(intr_handler_181);
            self.m_idt[Self::INTR_OFFSET + 182].set_handler_fn(intr_handler_182);
            self.m_idt[Self::INTR_OFFSET + 183].set_handler_fn(intr_handler_183);
            self.m_idt[Self::INTR_OFFSET + 184].set_handler_fn(intr_handler_184);
            self.m_idt[Self::INTR_OFFSET + 185].set_handler_fn(intr_handler_185);
            self.m_idt[Self::INTR_OFFSET + 186].set_handler_fn(intr_handler_186);
            self.m_idt[Self::INTR_OFFSET + 187].set_handler_fn(intr_handler_187);
            self.m_idt[Self::INTR_OFFSET + 188].set_handler_fn(intr_handler_188);
            self.m_idt[Self::INTR_OFFSET + 189].set_handler_fn(intr_handler_189);
            self.m_idt[Self::INTR_OFFSET + 190].set_handler_fn(intr_handler_190);
            self.m_idt[Self::INTR_OFFSET + 191].set_handler_fn(intr_handler_191);
            self.m_idt[Self::INTR_OFFSET + 192].set_handler_fn(intr_handler_192);
            self.m_idt[Self::INTR_OFFSET + 193].set_handler_fn(intr_handler_193);
            self.m_idt[Self::INTR_OFFSET + 194].set_handler_fn(intr_handler_194);
            self.m_idt[Self::INTR_OFFSET + 195].set_handler_fn(intr_handler_195);
            self.m_idt[Self::INTR_OFFSET + 196].set_handler_fn(intr_handler_196);
            self.m_idt[Self::INTR_OFFSET + 197].set_handler_fn(intr_handler_197);
            self.m_idt[Self::INTR_OFFSET + 198].set_handler_fn(intr_handler_198);
            self.m_idt[Self::INTR_OFFSET + 199].set_handler_fn(intr_handler_199);
            self.m_idt[Self::INTR_OFFSET + 200].set_handler_fn(intr_handler_200);
            self.m_idt[Self::INTR_OFFSET + 201].set_handler_fn(intr_handler_201);
            self.m_idt[Self::INTR_OFFSET + 202].set_handler_fn(intr_handler_202);
            self.m_idt[Self::INTR_OFFSET + 203].set_handler_fn(intr_handler_203);
            self.m_idt[Self::INTR_OFFSET + 204].set_handler_fn(intr_handler_204);
            self.m_idt[Self::INTR_OFFSET + 205].set_handler_fn(intr_handler_205);
            self.m_idt[Self::INTR_OFFSET + 206].set_handler_fn(intr_handler_206);
            self.m_idt[Self::INTR_OFFSET + 207].set_handler_fn(intr_handler_207);
            self.m_idt[Self::INTR_OFFSET + 208].set_handler_fn(intr_handler_208);
            self.m_idt[Self::INTR_OFFSET + 209].set_handler_fn(intr_handler_209);
            self.m_idt[Self::INTR_OFFSET + 210].set_handler_fn(intr_handler_210);
            self.m_idt[Self::INTR_OFFSET + 211].set_handler_fn(intr_handler_211);
            self.m_idt[Self::INTR_OFFSET + 212].set_handler_fn(intr_handler_212);
            self.m_idt[Self::INTR_OFFSET + 213].set_handler_fn(intr_handler_213);
            self.m_idt[Self::INTR_OFFSET + 214].set_handler_fn(intr_handler_214);
            self.m_idt[Self::INTR_OFFSET + 215].set_handler_fn(intr_handler_215);
            self.m_idt[Self::INTR_OFFSET + 216].set_handler_fn(intr_handler_216);
            self.m_idt[Self::INTR_OFFSET + 217].set_handler_fn(intr_handler_217);
            self.m_idt[Self::INTR_OFFSET + 218].set_handler_fn(intr_handler_218);
            self.m_idt[Self::INTR_OFFSET + 219].set_handler_fn(intr_handler_219);
            self.m_idt[Self::INTR_OFFSET + 220].set_handler_fn(intr_handler_220);
            self.m_idt[Self::INTR_OFFSET + 221].set_handler_fn(intr_handler_221);
            self.m_idt[Self::INTR_OFFSET + 222].set_handler_fn(intr_handler_222);
            self.m_idt[Self::INTR_OFFSET + 223].set_handler_fn(intr_handler_223);
        }

        /* store a little static stack for double fault exceptions.
         * double fault should never occur but to catch bugs it is necessary, instead
         * of seeing the emulator reset itself
         */
        BSP_INIT_TSS.interrupt_stack_table[0] = {
            use x86_64::addr::VirtAddr as X64VirtAddr;

            const DOUBLE_FAULT_STACK_SIZE: usize = 4096 * 4;

            /* The stack for double faults is allocated into the BSS to avoid usage of
             * FrameAllocator and because this should be not really necessary.
             *
             * TODO the HAL or the HH_Loader should already load a valid GDT or TSS?
             */
            static mut STACK_SPACE: [u8; DOUBLE_FAULT_STACK_SIZE] =
                [0; DOUBLE_FAULT_STACK_SIZE];

            /* return the end of the static stack */
            X64VirtAddr::from_ptr(&STACK_SPACE) + DOUBLE_FAULT_STACK_SIZE
        };

        /* add the kernel code + data entries and the TSS segment */
        let kern_code_seg = BSP_INIT_GDT.add_entry(Descriptor::kernel_code_segment());
        let _kern_data_seg = BSP_INIT_GDT.add_entry(Descriptor::kernel_data_segment());
        let tss_seg = BSP_INIT_GDT.add_entry(Descriptor::tss_segment(&BSP_INIT_TSS));

        /* load the global descriptor table */
        BSP_INIT_GDT.load_unsafe();

        /* reload code segment and TSS register */
        set_cs(kern_code_seg);
        load_tss(tss_seg);

        /* then load the interrupt descriptor table */
        self.m_idt.load_unsafe();
    }

    fn enable_intr(&self) {
        interrupts::enable()
    }

    fn disable_intr(&self) {
        interrupts::disable()
    }

    fn intr_are_enabled(&self) -> bool {
        interrupts::are_enabled()
    }
}

/*
 * x86 INTERRUPTS HANDLERS
 */

extern "x86-interrupt" fn except_double_fault(stack_frame: X64InterruptStackFrame,
                                              error_value: u64)
                                              -> ! {
    panic!("Kernel BUG: Double fault occurred: {}\n{:#?}", error_value, stack_frame);
}

extern "x86-interrupt" fn except_divide_error(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_except_handler(&mut stack_frame,
                                          InterruptManagerException::MathDomain);
}

extern "x86-interrupt" fn except_invalid_op(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_except_handler(&mut stack_frame,
                                          InterruptManagerException::InvalidInstr);
}

extern "x86-interrupt" fn except_page_fault(mut stack_frame: X64InterruptStackFrame,
                                            _error_code: PageFaultErrorCode) {
    debug!("PageFault: {:?} -> {:x}",
           _error_code,
           VirtAddr::new(x86_64::registers::control::Cr2::read().as_u64() as usize));

    HwInterruptManager::hw_except_handler(&mut stack_frame,
                                          InterruptManagerException::PageFault);
}

extern "x86-interrupt" fn except_floating_point(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_except_handler(&mut stack_frame,
                                          InterruptManagerException::FloatingPoint);
}

extern "x86-interrupt" fn intr_handler_0(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 0);
}

extern "x86-interrupt" fn intr_handler_1(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 1);
}

extern "x86-interrupt" fn intr_handler_2(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 2);
}

extern "x86-interrupt" fn intr_handler_3(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 3);
}

extern "x86-interrupt" fn intr_handler_4(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 4);
}

extern "x86-interrupt" fn intr_handler_5(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 5);
}

extern "x86-interrupt" fn intr_handler_6(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 6);
}

extern "x86-interrupt" fn intr_handler_7(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 7);
}

extern "x86-interrupt" fn intr_handler_8(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 8);
}

extern "x86-interrupt" fn intr_handler_9(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 9);
}

extern "x86-interrupt" fn intr_handler_10(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 10);
}

extern "x86-interrupt" fn intr_handler_11(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 11);
}

extern "x86-interrupt" fn intr_handler_12(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 12);
}

extern "x86-interrupt" fn intr_handler_13(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 13);
}

extern "x86-interrupt" fn intr_handler_14(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 14);
}

extern "x86-interrupt" fn intr_handler_15(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 15);
}

extern "x86-interrupt" fn intr_handler_16(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 16);
}

extern "x86-interrupt" fn intr_handler_17(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 17);
}

extern "x86-interrupt" fn intr_handler_18(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 18);
}

extern "x86-interrupt" fn intr_handler_19(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 19);
}

extern "x86-interrupt" fn intr_handler_20(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 20);
}

extern "x86-interrupt" fn intr_handler_21(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 21);
}

extern "x86-interrupt" fn intr_handler_22(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 22);
}

extern "x86-interrupt" fn intr_handler_23(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 23);
}

extern "x86-interrupt" fn intr_handler_24(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 24);
}

extern "x86-interrupt" fn intr_handler_25(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 25);
}

extern "x86-interrupt" fn intr_handler_26(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 26);
}

extern "x86-interrupt" fn intr_handler_27(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 27);
}

extern "x86-interrupt" fn intr_handler_28(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 28);
}

extern "x86-interrupt" fn intr_handler_29(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 29);
}

extern "x86-interrupt" fn intr_handler_30(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 30);
}

extern "x86-interrupt" fn intr_handler_31(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 31);
}

extern "x86-interrupt" fn intr_handler_32(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 32);
}

extern "x86-interrupt" fn intr_handler_33(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 33);
}

extern "x86-interrupt" fn intr_handler_34(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 34);
}

extern "x86-interrupt" fn intr_handler_35(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 35);
}

extern "x86-interrupt" fn intr_handler_36(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 36);
}

extern "x86-interrupt" fn intr_handler_37(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 37);
}

extern "x86-interrupt" fn intr_handler_38(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 38);
}

extern "x86-interrupt" fn intr_handler_39(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 39);
}

extern "x86-interrupt" fn intr_handler_40(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 40);
}

extern "x86-interrupt" fn intr_handler_41(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 41);
}

extern "x86-interrupt" fn intr_handler_42(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 42);
}

extern "x86-interrupt" fn intr_handler_43(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 43);
}

extern "x86-interrupt" fn intr_handler_44(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 44);
}

extern "x86-interrupt" fn intr_handler_45(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 45);
}

extern "x86-interrupt" fn intr_handler_46(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 46);
}

extern "x86-interrupt" fn intr_handler_47(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 47);
}

extern "x86-interrupt" fn intr_handler_48(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 48);
}

extern "x86-interrupt" fn intr_handler_49(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 49);
}

extern "x86-interrupt" fn intr_handler_50(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 50);
}

extern "x86-interrupt" fn intr_handler_51(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 51);
}

extern "x86-interrupt" fn intr_handler_52(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 52);
}

extern "x86-interrupt" fn intr_handler_53(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 53);
}

extern "x86-interrupt" fn intr_handler_54(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 54);
}

extern "x86-interrupt" fn intr_handler_55(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 55);
}

extern "x86-interrupt" fn intr_handler_56(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 56);
}

extern "x86-interrupt" fn intr_handler_57(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 57);
}

extern "x86-interrupt" fn intr_handler_58(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 58);
}

extern "x86-interrupt" fn intr_handler_59(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 59);
}

extern "x86-interrupt" fn intr_handler_60(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 60);
}

extern "x86-interrupt" fn intr_handler_61(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 61);
}

extern "x86-interrupt" fn intr_handler_62(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 62);
}

extern "x86-interrupt" fn intr_handler_63(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 63);
}

extern "x86-interrupt" fn intr_handler_64(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 64);
}

extern "x86-interrupt" fn intr_handler_65(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 65);
}

extern "x86-interrupt" fn intr_handler_66(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 66);
}

extern "x86-interrupt" fn intr_handler_67(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 67);
}

extern "x86-interrupt" fn intr_handler_68(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 68);
}

extern "x86-interrupt" fn intr_handler_69(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 69);
}

extern "x86-interrupt" fn intr_handler_70(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 70);
}

extern "x86-interrupt" fn intr_handler_71(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 71);
}

extern "x86-interrupt" fn intr_handler_72(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 72);
}

extern "x86-interrupt" fn intr_handler_73(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 73);
}

extern "x86-interrupt" fn intr_handler_74(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 74);
}

extern "x86-interrupt" fn intr_handler_75(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 75);
}

extern "x86-interrupt" fn intr_handler_76(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 76);
}

extern "x86-interrupt" fn intr_handler_77(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 77);
}

extern "x86-interrupt" fn intr_handler_78(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 78);
}

extern "x86-interrupt" fn intr_handler_79(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 79);
}

extern "x86-interrupt" fn intr_handler_80(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 80);
}

extern "x86-interrupt" fn intr_handler_81(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 81);
}

extern "x86-interrupt" fn intr_handler_82(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 82);
}

extern "x86-interrupt" fn intr_handler_83(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 83);
}

extern "x86-interrupt" fn intr_handler_84(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 84);
}

extern "x86-interrupt" fn intr_handler_85(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 85);
}

extern "x86-interrupt" fn intr_handler_86(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 86);
}

extern "x86-interrupt" fn intr_handler_87(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 87);
}

extern "x86-interrupt" fn intr_handler_88(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 88);
}

extern "x86-interrupt" fn intr_handler_89(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 89);
}

extern "x86-interrupt" fn intr_handler_90(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 90);
}

extern "x86-interrupt" fn intr_handler_91(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 91);
}

extern "x86-interrupt" fn intr_handler_92(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 92);
}

extern "x86-interrupt" fn intr_handler_93(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 93);
}

extern "x86-interrupt" fn intr_handler_94(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 94);
}

extern "x86-interrupt" fn intr_handler_95(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 95);
}

extern "x86-interrupt" fn intr_handler_96(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 96);
}

extern "x86-interrupt" fn intr_handler_97(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 97);
}

extern "x86-interrupt" fn intr_handler_98(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 98);
}

extern "x86-interrupt" fn intr_handler_99(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 99);
}

extern "x86-interrupt" fn intr_handler_100(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 100);
}

extern "x86-interrupt" fn intr_handler_101(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 101);
}

extern "x86-interrupt" fn intr_handler_102(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 102);
}

extern "x86-interrupt" fn intr_handler_103(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 103);
}

extern "x86-interrupt" fn intr_handler_104(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 104);
}

extern "x86-interrupt" fn intr_handler_105(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 105);
}

extern "x86-interrupt" fn intr_handler_106(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 106);
}

extern "x86-interrupt" fn intr_handler_107(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 107);
}

extern "x86-interrupt" fn intr_handler_108(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 108);
}

extern "x86-interrupt" fn intr_handler_109(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 109);
}

extern "x86-interrupt" fn intr_handler_110(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 110);
}

extern "x86-interrupt" fn intr_handler_111(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 111);
}

extern "x86-interrupt" fn intr_handler_112(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 112);
}

extern "x86-interrupt" fn intr_handler_113(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 113);
}

extern "x86-interrupt" fn intr_handler_114(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 114);
}

extern "x86-interrupt" fn intr_handler_115(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 115);
}

extern "x86-interrupt" fn intr_handler_116(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 116);
}

extern "x86-interrupt" fn intr_handler_117(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 117);
}

extern "x86-interrupt" fn intr_handler_118(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 118);
}

extern "x86-interrupt" fn intr_handler_119(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 119);
}

extern "x86-interrupt" fn intr_handler_120(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 120);
}

extern "x86-interrupt" fn intr_handler_121(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 121);
}

extern "x86-interrupt" fn intr_handler_122(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 122);
}

extern "x86-interrupt" fn intr_handler_123(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 123);
}

extern "x86-interrupt" fn intr_handler_124(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 124);
}

extern "x86-interrupt" fn intr_handler_125(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 125);
}

extern "x86-interrupt" fn intr_handler_126(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 126);
}

extern "x86-interrupt" fn intr_handler_127(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 127);
}

extern "x86-interrupt" fn intr_handler_128(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 128);
}

extern "x86-interrupt" fn intr_handler_129(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 129);
}

extern "x86-interrupt" fn intr_handler_130(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 130);
}

extern "x86-interrupt" fn intr_handler_131(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 131);
}

extern "x86-interrupt" fn intr_handler_132(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 132);
}

extern "x86-interrupt" fn intr_handler_133(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 133);
}

extern "x86-interrupt" fn intr_handler_134(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 134);
}

extern "x86-interrupt" fn intr_handler_135(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 135);
}

extern "x86-interrupt" fn intr_handler_136(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 136);
}

extern "x86-interrupt" fn intr_handler_137(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 137);
}

extern "x86-interrupt" fn intr_handler_138(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 138);
}

extern "x86-interrupt" fn intr_handler_139(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 139);
}

extern "x86-interrupt" fn intr_handler_140(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 140);
}

extern "x86-interrupt" fn intr_handler_141(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 141);
}

extern "x86-interrupt" fn intr_handler_142(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 142);
}

extern "x86-interrupt" fn intr_handler_143(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 143);
}

extern "x86-interrupt" fn intr_handler_144(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 144);
}

extern "x86-interrupt" fn intr_handler_145(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 145);
}

extern "x86-interrupt" fn intr_handler_146(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 146);
}

extern "x86-interrupt" fn intr_handler_147(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 147);
}

extern "x86-interrupt" fn intr_handler_148(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 148);
}

extern "x86-interrupt" fn intr_handler_149(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 149);
}

extern "x86-interrupt" fn intr_handler_150(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 150);
}

extern "x86-interrupt" fn intr_handler_151(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 151);
}

extern "x86-interrupt" fn intr_handler_152(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 152);
}

extern "x86-interrupt" fn intr_handler_153(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 153);
}

extern "x86-interrupt" fn intr_handler_154(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 154);
}

extern "x86-interrupt" fn intr_handler_155(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 155);
}

extern "x86-interrupt" fn intr_handler_156(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 156);
}

extern "x86-interrupt" fn intr_handler_157(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 157);
}

extern "x86-interrupt" fn intr_handler_158(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 158);
}

extern "x86-interrupt" fn intr_handler_159(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 159);
}

extern "x86-interrupt" fn intr_handler_160(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 160);
}

extern "x86-interrupt" fn intr_handler_161(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 161);
}

extern "x86-interrupt" fn intr_handler_162(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 162);
}

extern "x86-interrupt" fn intr_handler_163(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 163);
}

extern "x86-interrupt" fn intr_handler_164(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 164);
}

extern "x86-interrupt" fn intr_handler_165(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 165);
}

extern "x86-interrupt" fn intr_handler_166(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 166);
}

extern "x86-interrupt" fn intr_handler_167(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 167);
}

extern "x86-interrupt" fn intr_handler_168(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 168);
}

extern "x86-interrupt" fn intr_handler_169(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 169);
}

extern "x86-interrupt" fn intr_handler_170(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 170);
}

extern "x86-interrupt" fn intr_handler_171(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 171);
}

extern "x86-interrupt" fn intr_handler_172(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 172);
}

extern "x86-interrupt" fn intr_handler_173(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 173);
}

extern "x86-interrupt" fn intr_handler_174(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 174);
}

extern "x86-interrupt" fn intr_handler_175(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 175);
}

extern "x86-interrupt" fn intr_handler_176(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 176);
}

extern "x86-interrupt" fn intr_handler_177(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 177);
}

extern "x86-interrupt" fn intr_handler_178(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 178);
}

extern "x86-interrupt" fn intr_handler_179(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 179);
}

extern "x86-interrupt" fn intr_handler_180(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 180);
}

extern "x86-interrupt" fn intr_handler_181(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 181);
}

extern "x86-interrupt" fn intr_handler_182(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 182);
}

extern "x86-interrupt" fn intr_handler_183(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 183);
}

extern "x86-interrupt" fn intr_handler_184(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 184);
}

extern "x86-interrupt" fn intr_handler_185(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 185);
}

extern "x86-interrupt" fn intr_handler_186(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 186);
}

extern "x86-interrupt" fn intr_handler_187(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 187);
}

extern "x86-interrupt" fn intr_handler_188(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 188);
}

extern "x86-interrupt" fn intr_handler_189(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 189);
}

extern "x86-interrupt" fn intr_handler_190(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 190);
}

extern "x86-interrupt" fn intr_handler_191(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 191);
}

extern "x86-interrupt" fn intr_handler_192(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 192);
}

extern "x86-interrupt" fn intr_handler_193(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 193);
}

extern "x86-interrupt" fn intr_handler_194(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 194);
}

extern "x86-interrupt" fn intr_handler_195(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 195);
}

extern "x86-interrupt" fn intr_handler_196(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 196);
}

extern "x86-interrupt" fn intr_handler_197(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 197);
}

extern "x86-interrupt" fn intr_handler_198(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 198);
}

extern "x86-interrupt" fn intr_handler_199(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 199);
}

extern "x86-interrupt" fn intr_handler_200(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 200);
}

extern "x86-interrupt" fn intr_handler_201(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 201);
}

extern "x86-interrupt" fn intr_handler_202(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 202);
}

extern "x86-interrupt" fn intr_handler_203(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 203);
}

extern "x86-interrupt" fn intr_handler_204(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 204);
}

extern "x86-interrupt" fn intr_handler_205(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 205);
}

extern "x86-interrupt" fn intr_handler_206(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 206);
}

extern "x86-interrupt" fn intr_handler_207(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 207);
}

extern "x86-interrupt" fn intr_handler_208(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 208);
}

extern "x86-interrupt" fn intr_handler_209(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 209);
}

extern "x86-interrupt" fn intr_handler_210(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 210);
}

extern "x86-interrupt" fn intr_handler_211(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 211);
}

extern "x86-interrupt" fn intr_handler_212(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 212);
}

extern "x86-interrupt" fn intr_handler_213(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 213);
}

extern "x86-interrupt" fn intr_handler_214(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 214);
}

extern "x86-interrupt" fn intr_handler_215(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 215);
}

extern "x86-interrupt" fn intr_handler_216(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 216);
}

extern "x86-interrupt" fn intr_handler_217(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 217);
}

extern "x86-interrupt" fn intr_handler_218(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 218);
}

extern "x86-interrupt" fn intr_handler_219(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 219);
}

extern "x86-interrupt" fn intr_handler_220(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 220);
}

extern "x86-interrupt" fn intr_handler_221(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 221);
}

extern "x86-interrupt" fn intr_handler_222(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 222);
}

extern "x86-interrupt" fn intr_handler_223(mut stack_frame: X64InterruptStackFrame) {
    HwInterruptManager::hw_intr_handler(&mut stack_frame,
                                        HwInterruptManager::INTR_OFFSET + 223);
}

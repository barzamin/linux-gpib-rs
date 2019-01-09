#![allow(bad_style)]

#[doc(hidden)]
pub type __enum_ty = ::std::os::raw::c_int;

pub const GPIB_MAX_NUM_BOARDS: u32 = 16;
pub const GPIB_MAX_NUM_DESCRIPTORS: u32 = 4096;
pub const ibsta_bit_numbers_DCAS_NUM: ibsta_bit_numbers = 0;
pub const ibsta_bit_numbers_DTAS_NUM: ibsta_bit_numbers = 1;
pub const ibsta_bit_numbers_LACS_NUM: ibsta_bit_numbers = 2;
pub const ibsta_bit_numbers_TACS_NUM: ibsta_bit_numbers = 3;
pub const ibsta_bit_numbers_ATN_NUM: ibsta_bit_numbers = 4;
pub const ibsta_bit_numbers_CIC_NUM: ibsta_bit_numbers = 5;
pub const ibsta_bit_numbers_REM_NUM: ibsta_bit_numbers = 6;
pub const ibsta_bit_numbers_LOK_NUM: ibsta_bit_numbers = 7;
pub const ibsta_bit_numbers_CMPL_NUM: ibsta_bit_numbers = 8;
pub const ibsta_bit_numbers_EVENT_NUM: ibsta_bit_numbers = 9;
pub const ibsta_bit_numbers_SPOLL_NUM: ibsta_bit_numbers = 10;
pub const ibsta_bit_numbers_RQS_NUM: ibsta_bit_numbers = 11;
pub const ibsta_bit_numbers_SRQI_NUM: ibsta_bit_numbers = 12;
pub const ibsta_bit_numbers_END_NUM: ibsta_bit_numbers = 13;
pub const ibsta_bit_numbers_TIMO_NUM: ibsta_bit_numbers = 14;
pub const ibsta_bit_numbers_ERR_NUM: ibsta_bit_numbers = 15;
pub type ibsta_bit_numbers = __enum_ty;

pub const ibsta_bits_DCAS: ibsta_bits = 1;
pub const ibsta_bits_DTAS: ibsta_bits = 2;
pub const ibsta_bits_LACS: ibsta_bits = 4;
pub const ibsta_bits_TACS: ibsta_bits = 8;
pub const ibsta_bits_ATN: ibsta_bits = 16;
pub const ibsta_bits_CIC: ibsta_bits = 32;
pub const ibsta_bits_REM: ibsta_bits = 64;
pub const ibsta_bits_LOK: ibsta_bits = 128;
pub const ibsta_bits_CMPL: ibsta_bits = 256;
pub const ibsta_bits_EVENT: ibsta_bits = 512;
pub const ibsta_bits_SPOLL: ibsta_bits = 1024;
pub const ibsta_bits_RQS: ibsta_bits = 2048;
pub const ibsta_bits_SRQI: ibsta_bits = 4096;
pub const ibsta_bits_END: ibsta_bits = 8192;
pub const ibsta_bits_TIMO: ibsta_bits = 16384;
pub const ibsta_bits_ERR: ibsta_bits = 32768;
pub type ibsta_bits = __enum_ty;

pub const device_status_mask: ::std::os::raw::c_int = 59648;
pub const board_status_mask: ::std::os::raw::c_int = 63487;
pub const iberr_code_EDVR: iberr_code = 0;
pub const iberr_code_ECIC: iberr_code = 1;
pub const iberr_code_ENOL: iberr_code = 2;
pub const iberr_code_EADR: iberr_code = 3;
pub const iberr_code_EARG: iberr_code = 4;
pub const iberr_code_ESAC: iberr_code = 5;
pub const iberr_code_EABO: iberr_code = 6;
pub const iberr_code_ENEB: iberr_code = 7;
pub const iberr_code_EDMA: iberr_code = 8;
pub const iberr_code_EOIP: iberr_code = 10;
pub const iberr_code_ECAP: iberr_code = 11;
pub const iberr_code_EFSO: iberr_code = 12;
pub const iberr_code_EBUS: iberr_code = 14;
pub const iberr_code_ESTB: iberr_code = 15;
pub const iberr_code_ESRQ: iberr_code = 16;
pub const iberr_code_ETAB: iberr_code = 20;
pub type iberr_code = __enum_ty;

pub const gpib_timeout_TNONE: gpib_timeout = 0;
pub const gpib_timeout_T10us: gpib_timeout = 1;
pub const gpib_timeout_T30us: gpib_timeout = 2;
pub const gpib_timeout_T100us: gpib_timeout = 3;
pub const gpib_timeout_T300us: gpib_timeout = 4;
pub const gpib_timeout_T1ms: gpib_timeout = 5;
pub const gpib_timeout_T3ms: gpib_timeout = 6;
pub const gpib_timeout_T10ms: gpib_timeout = 7;
pub const gpib_timeout_T30ms: gpib_timeout = 8;
pub const gpib_timeout_T100ms: gpib_timeout = 9;
pub const gpib_timeout_T300ms: gpib_timeout = 10;
pub const gpib_timeout_T1s: gpib_timeout = 11;
pub const gpib_timeout_T3s: gpib_timeout = 12;
pub const gpib_timeout_T10s: gpib_timeout = 13;
pub const gpib_timeout_T30s: gpib_timeout = 14;
pub const gpib_timeout_T100s: gpib_timeout = 15;
pub const gpib_timeout_T300s: gpib_timeout = 16;
pub const gpib_timeout_T1000s: gpib_timeout = 17;
pub type gpib_timeout = __enum_ty;

pub const eos_flags_EOS_MASK: eos_flags = 7168;
pub const eos_flags_REOS: eos_flags = 1024;
pub const eos_flags_XEOS: eos_flags = 2048;
pub const eos_flags_BIN: eos_flags = 4096;
pub type eos_flags = __enum_ty;

pub const bus_control_line_ValidDAV: bus_control_line = 1;
pub const bus_control_line_ValidNDAC: bus_control_line = 2;
pub const bus_control_line_ValidNRFD: bus_control_line = 4;
pub const bus_control_line_ValidIFC: bus_control_line = 8;
pub const bus_control_line_ValidREN: bus_control_line = 16;
pub const bus_control_line_ValidSRQ: bus_control_line = 32;
pub const bus_control_line_ValidATN: bus_control_line = 64;
pub const bus_control_line_ValidEOI: bus_control_line = 128;
pub const bus_control_line_ValidALL: bus_control_line = 255;
pub const bus_control_line_BusDAV: bus_control_line = 256;
pub const bus_control_line_BusNDAC: bus_control_line = 512;
pub const bus_control_line_BusNRFD: bus_control_line = 1024;
pub const bus_control_line_BusIFC: bus_control_line = 2048;
pub const bus_control_line_BusREN: bus_control_line = 4096;
pub const bus_control_line_BusSRQ: bus_control_line = 8192;
pub const bus_control_line_BusATN: bus_control_line = 16384;
pub const bus_control_line_BusEOI: bus_control_line = 32768;
pub type bus_control_line = __enum_ty;

pub const old_bus_control_line_BUS_DAV: old_bus_control_line = 256;
pub const old_bus_control_line_BUS_NDAC: old_bus_control_line = 512;
pub const old_bus_control_line_BUS_NRFD: old_bus_control_line = 1024;
pub const old_bus_control_line_BUS_IFC: old_bus_control_line = 2048;
pub const old_bus_control_line_BUS_REN: old_bus_control_line = 4096;
pub const old_bus_control_line_BUS_SRQ: old_bus_control_line = 8192;
pub const old_bus_control_line_BUS_ATN: old_bus_control_line = 16384;
pub const old_bus_control_line_BUS_EOI: old_bus_control_line = 32768;
pub type old_bus_control_line = __enum_ty;

pub const cmd_byte_GTL: cmd_byte = 1;
pub const cmd_byte_SDC: cmd_byte = 4;
pub const cmd_byte_PPConfig: cmd_byte = 5;
pub const cmd_byte_PPC: cmd_byte = 5;
pub const cmd_byte_GET: cmd_byte = 8;
pub const cmd_byte_TCT: cmd_byte = 9;
pub const cmd_byte_LLO: cmd_byte = 17;
pub const cmd_byte_DCL: cmd_byte = 20;
pub const cmd_byte_PPU: cmd_byte = 21;
pub const cmd_byte_SPE: cmd_byte = 24;
pub const cmd_byte_SPD: cmd_byte = 25;
pub const cmd_byte_LAD: cmd_byte = 32;
pub const cmd_byte_UNL: cmd_byte = 63;
pub const cmd_byte_TAD: cmd_byte = 64;
pub const cmd_byte_UNT: cmd_byte = 95;
pub const cmd_byte_SAD: cmd_byte = 96;
pub const cmd_byte_PPE: cmd_byte = 96;
pub const cmd_byte_PPD: cmd_byte = 112;
pub type cmd_byte = __enum_ty;

pub const ppe_bits_PPC_DISABLE: ppe_bits = 16;
pub const ppe_bits_PPC_SENSE: ppe_bits = 8;
pub const ppe_bits_PPC_DIO_MASK: ppe_bits = 7;
pub type ppe_bits = __enum_ty;

pub const gpib_addr_max: ::std::os::raw::c_int = 30;
pub const ibask_option_IbaPAD: ibask_option = 1;
pub const ibask_option_IbaSAD: ibask_option = 2;
pub const ibask_option_IbaTMO: ibask_option = 3;
pub const ibask_option_IbaEOT: ibask_option = 4;
pub const ibask_option_IbaPPC: ibask_option = 5;
pub const ibask_option_IbaREADDR: ibask_option = 6;
pub const ibask_option_IbaAUTOPOLL: ibask_option = 7;
pub const ibask_option_IbaCICPROT: ibask_option = 8;
pub const ibask_option_IbaIRQ: ibask_option = 9;
pub const ibask_option_IbaSC: ibask_option = 10;
pub const ibask_option_IbaSRE: ibask_option = 11;
pub const ibask_option_IbaEOSrd: ibask_option = 12;
pub const ibask_option_IbaEOSwrt: ibask_option = 13;
pub const ibask_option_IbaEOScmp: ibask_option = 14;
pub const ibask_option_IbaEOSchar: ibask_option = 15;
pub const ibask_option_IbaPP2: ibask_option = 16;
pub const ibask_option_IbaTIMING: ibask_option = 17;
pub const ibask_option_IbaDMA: ibask_option = 18;
pub const ibask_option_IbaReadAdjust: ibask_option = 19;
pub const ibask_option_IbaWriteAdjust: ibask_option = 20;
pub const ibask_option_IbaEventQueue: ibask_option = 21;
pub const ibask_option_IbaSPollBit: ibask_option = 22;
pub const ibask_option_IbaSpollBit: ibask_option = 22;
pub const ibask_option_IbaSendLLO: ibask_option = 23;
pub const ibask_option_IbaSPollTime: ibask_option = 24;
pub const ibask_option_IbaPPollTime: ibask_option = 25;
pub const ibask_option_IbaEndBitIsNormal: ibask_option = 26;
pub const ibask_option_IbaUnAddr: ibask_option = 27;
pub const ibask_option_IbaHSCableLength: ibask_option = 31;
pub const ibask_option_IbaIst: ibask_option = 32;
pub const ibask_option_IbaRsv: ibask_option = 33;
pub const ibask_option_IbaBNA: ibask_option = 512;
pub const ibask_option_Iba7BitEOS: ibask_option = 4096;
pub type ibask_option = __enum_ty;

pub const ibconfig_option_IbcPAD: ibconfig_option = 1;
pub const ibconfig_option_IbcSAD: ibconfig_option = 2;
pub const ibconfig_option_IbcTMO: ibconfig_option = 3;
pub const ibconfig_option_IbcEOT: ibconfig_option = 4;
pub const ibconfig_option_IbcPPC: ibconfig_option = 5;
pub const ibconfig_option_IbcREADDR: ibconfig_option = 6;
pub const ibconfig_option_IbcAUTOPOLL: ibconfig_option = 7;
pub const ibconfig_option_IbcCICPROT: ibconfig_option = 8;
pub const ibconfig_option_IbcIRQ: ibconfig_option = 9;
pub const ibconfig_option_IbcSC: ibconfig_option = 10;
pub const ibconfig_option_IbcSRE: ibconfig_option = 11;
pub const ibconfig_option_IbcEOSrd: ibconfig_option = 12;
pub const ibconfig_option_IbcEOSwrt: ibconfig_option = 13;
pub const ibconfig_option_IbcEOScmp: ibconfig_option = 14;
pub const ibconfig_option_IbcEOSchar: ibconfig_option = 15;
pub const ibconfig_option_IbcPP2: ibconfig_option = 16;
pub const ibconfig_option_IbcTIMING: ibconfig_option = 17;
pub const ibconfig_option_IbcDMA: ibconfig_option = 18;
pub const ibconfig_option_IbcReadAdjust: ibconfig_option = 19;
pub const ibconfig_option_IbcWriteAdjust: ibconfig_option = 20;
pub const ibconfig_option_IbcEventQueue: ibconfig_option = 21;
pub const ibconfig_option_IbcSPollBit: ibconfig_option = 22;
pub const ibconfig_option_IbcSpollBit: ibconfig_option = 22;
pub const ibconfig_option_IbcSendLLO: ibconfig_option = 23;
pub const ibconfig_option_IbcSPollTime: ibconfig_option = 24;
pub const ibconfig_option_IbcPPollTime: ibconfig_option = 25;
pub const ibconfig_option_IbcEndBitIsNormal: ibconfig_option = 26;
pub const ibconfig_option_IbcUnAddr: ibconfig_option = 27;
pub const ibconfig_option_IbcHSCableLength: ibconfig_option = 31;
pub const ibconfig_option_IbcIst: ibconfig_option = 32;
pub const ibconfig_option_IbcRsv: ibconfig_option = 33;
pub const ibconfig_option_IbcBNA: ibconfig_option = 512;
pub type ibconfig_option = __enum_ty;

pub const t1_delays_T1_DELAY_2000ns: t1_delays = 1;
pub const t1_delays_T1_DELAY_500ns: t1_delays = 2;
pub const t1_delays_T1_DELAY_350ns: t1_delays = 3;
pub type t1_delays = __enum_ty;

pub const request_service_bit: ::std::os::raw::c_int = 64;
pub const gpib_events_EventNone: gpib_events = 0;
pub const gpib_events_EventDevTrg: gpib_events = 1;
pub const gpib_events_EventDevClr: gpib_events = 2;
pub const gpib_events_EventIFC: gpib_events = 3;
pub type gpib_events = __enum_ty;

pub const gpib_stb_IbStbRQS: gpib_stb = 64;
pub const gpib_stb_IbStbESB: gpib_stb = 32;
pub const gpib_stb_IbStbMAV: gpib_stb = 16;
pub type gpib_stb = __enum_ty;

pub type Addr4882_t = u16;
pub const NOADDR: Addr4882_t = 65535;
pub const STOPend: ::std::os::raw::c_int = 256;

pub const sad_special_address_NO_SAD: sad_special_address = 0;
pub const sad_special_address_ALL_SAD: sad_special_address = -1;
pub type sad_special_address = __enum_ty;

pub const send_eotmode_NULLend: send_eotmode = 0;
pub const send_eotmode_DABend: send_eotmode = 1;
pub const send_eotmode_NLend: send_eotmode = 2;
pub type send_eotmode = __enum_ty;

extern "C" {
    #[link_name = "\u{1}ibsta"]
    pub static mut ibsta: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}ibcnt"]
    pub static mut ibcnt: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}iberr"]
    pub static mut iberr: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}ibcntl"]
    pub static mut ibcntl: ::std::os::raw::c_long;
}
extern "C" {
    pub fn AllSPoll(
        board_desc: ::std::os::raw::c_int,
        addressList: *const Addr4882_t,
        resultList: *mut ::std::os::raw::c_short,
    );
}
extern "C" {
    pub fn AllSpoll(
        board_desc: ::std::os::raw::c_int,
        addressList: *const Addr4882_t,
        resultList: *mut ::std::os::raw::c_short,
    );
}
extern "C" {
    pub fn DevClear(board_desc: ::std::os::raw::c_int, address: Addr4882_t);
}
extern "C" {
    pub fn DevClearList(board_desc: ::std::os::raw::c_int, addressList: *const Addr4882_t);
}
extern "C" {
    pub fn EnableLocal(board_desc: ::std::os::raw::c_int, addressList: *const Addr4882_t);
}
extern "C" {
    pub fn EnableRemote(board_desc: ::std::os::raw::c_int, addressList: *const Addr4882_t);
}
extern "C" {
    pub fn FindLstn(
        board_desc: ::std::os::raw::c_int,
        padList: *const Addr4882_t,
        resultList: *mut Addr4882_t,
        maxNumResults: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn FindRQS(
        board_desc: ::std::os::raw::c_int,
        addressList: *const Addr4882_t,
        result: *mut ::std::os::raw::c_short,
    );
}
extern "C" {
    pub fn PassControl(board_desc: ::std::os::raw::c_int, address: Addr4882_t);
}
extern "C" {
    pub fn PPoll(board_desc: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_short);
}
extern "C" {
    pub fn PPollConfig(
        board_desc: ::std::os::raw::c_int,
        address: Addr4882_t,
        dataLine: ::std::os::raw::c_int,
        lineSense: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn PPollUnconfig(board_desc: ::std::os::raw::c_int, addressList: *const Addr4882_t);
}
extern "C" {
    pub fn RcvRespMsg(
        board_desc: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
        termination: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ReadStatusByte(
        board_desc: ::std::os::raw::c_int,
        address: Addr4882_t,
        result: *mut ::std::os::raw::c_short,
    );
}
extern "C" {
    pub fn Receive(
        board_desc: ::std::os::raw::c_int,
        address: Addr4882_t,
        buffer: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
        termination: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ReceiveSetup(board_desc: ::std::os::raw::c_int, address: Addr4882_t);
}
extern "C" {
    pub fn ResetSys(board_desc: ::std::os::raw::c_int, addressList: *const Addr4882_t);
}
extern "C" {
    pub fn Send(
        board_desc: ::std::os::raw::c_int,
        address: Addr4882_t,
        buffer: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
        eot_mode: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SendCmds(
        board_desc: ::std::os::raw::c_int,
        cmds: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
    );
}
extern "C" {
    pub fn SendDataBytes(
        board_desc: ::std::os::raw::c_int,
        buffer: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
        eotmode: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SendIFC(board_desc: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SendLLO(board_desc: ::std::os::raw::c_int);
}
extern "C" {
    pub fn SendList(
        board_desc: ::std::os::raw::c_int,
        addressList: *const Addr4882_t,
        buffer: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
        eotmode: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn SendSetup(board_desc: ::std::os::raw::c_int, addressList: *const Addr4882_t);
}
extern "C" {
    pub fn SetRWLS(board_desc: ::std::os::raw::c_int, addressList: *const Addr4882_t);
}
extern "C" {
    pub fn TestSRQ(board_desc: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_short);
}
extern "C" {
    pub fn TestSys(
        board_desc: ::std::os::raw::c_int,
        addressList: *const Addr4882_t,
        resultList: *mut ::std::os::raw::c_short,
    );
}
extern "C" {
    pub fn ThreadIbsta() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ThreadIberr() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ThreadIbcnt() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ThreadIbcntl() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn AsyncIbsta() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AsyncIberr() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AsyncIbcnt() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AsyncIbcntl() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn Trigger(board_desc: ::std::os::raw::c_int, address: Addr4882_t);
}
extern "C" {
    pub fn TriggerList(board_desc: ::std::os::raw::c_int, addressList: *const Addr4882_t);
}
extern "C" {
    pub fn WaitSRQ(board_desc: ::std::os::raw::c_int, result: *mut ::std::os::raw::c_short);
}
extern "C" {
    pub fn ibask(
        ud: ::std::os::raw::c_int,
        option: ::std::os::raw::c_int,
        value: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibbna(
        ud: ::std::os::raw::c_int,
        board_name: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibcac(
        ud: ::std::os::raw::c_int,
        synchronous: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibclr(ud: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibcmd(
        ud: ::std::os::raw::c_int,
        cmd: *const ::std::os::raw::c_void,
        cnt: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibcmda(
        ud: ::std::os::raw::c_int,
        cmd: *const ::std::os::raw::c_void,
        cnt: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibconfig(
        ud: ::std::os::raw::c_int,
        option: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibdev(
        board_index: ::std::os::raw::c_int,
        pad: ::std::os::raw::c_int,
        sad: ::std::os::raw::c_int,
        timo: ::std::os::raw::c_int,
        send_eoi: ::std::os::raw::c_int,
        eosmode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibdma(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibeot(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibeos(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibevent(
        ud: ::std::os::raw::c_int,
        event: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibfind(dev: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibgts(
        ud: ::std::os::raw::c_int,
        shadow_handshake: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibist(ud: ::std::os::raw::c_int, ist: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn iblines(
        ud: ::std::os::raw::c_int,
        line_status: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibln(
        ud: ::std::os::raw::c_int,
        pad: ::std::os::raw::c_int,
        sad: ::std::os::raw::c_int,
        found_listener: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibloc(ud: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibonl(ud: ::std::os::raw::c_int, onl: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibpad(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibpct(ud: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibppc(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibrd(
        ud: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibrda(
        ud: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibrdf(
        ud: ::std::os::raw::c_int,
        file_path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibrpp(
        ud: ::std::os::raw::c_int,
        ppr: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibrsc(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibrsp(
        ud: ::std::os::raw::c_int,
        spr: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibrsv(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibsad(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibsic(ud: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibspb(
        ud: ::std::os::raw::c_int,
        sp_bytes: *mut ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibsre(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibstop(ud: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibtmo(ud: ::std::os::raw::c_int, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibtrg(ud: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibvers(version: *mut *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn ibwait(ud: ::std::os::raw::c_int, mask: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibwrt(
        ud: ::std::os::raw::c_int,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibwrta(
        ud: ::std::os::raw::c_int,
        buf: *const ::std::os::raw::c_void,
        count: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ibwrtf(
        ud: ::std::os::raw::c_int,
        file_path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gpib_error_string(iberr: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}

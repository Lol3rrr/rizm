use crate::{traits::Debugger, Display, Input, Memory};

mod app;
mod disp;
mod fs;
mod rtc;
mod serial;
mod sys;
mod timer;

const GETKEY: u32 = 0xeab;
const GETKEYWAIT_OS: u32 = 0x12bf;
const PRGM_GETKEY_OS: u32 = 0xd39;
const ITOA: u32 = 0x1170;

// https://prizm.cemetech.net/index.php?title=Category:Syscalls
/// Executes a single System-Call
pub async fn syscall<I, D>(
    id: u32,
    memory: &mut Memory,
    input: &mut I,
    display: &mut D,
    debug: &dyn Debugger,
) where
    I: Input,
    D: Display,
{
    let param_1 = memory.read_register(4);
    let param_2 = memory.read_register(5);
    let param_3 = memory.read_register(6);
    let param_4 = memory.read_register(7);

    match id {
        // https://prizm.cemetech.net/index.php?title=Alpha_GetData
        0x0034 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Alpha_GetData");
        }
        // https://prizm.cemetech.net/index.php?title=Alpha_SetData
        0x0035 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Alpha_SetData");
        }
        // https://prizm.cemetech.net/index.php?title=AUX_DisplayErrorMessage
        0x0C01 => {
            debug.print("https://prizm.cemetech.net/index.php?title=AUX_DisplayErrorMessage");
        }
        // https://prizm.cemetech.net/index.php?title=BatteryIcon
        0x1D89 => {
            debug.print("https://prizm.cemetech.net/index.php?title=BatteryIcon");
        }
        // https://prizm.cemetech.net/index.php?title=BCDtoInternal
        0x0160 => {
            debug.print("https://prizm.cemetech.net/index.php?title=BCDtoInternal");
        }
        // https://prizm.cemetech.net/index.php?title=Bkey_ClrAllFlags
        0x0111 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Bkey_ClrAllFlags");
        }
        // https://prizm.cemetech.net/index.php?title=Bkey_GetAllFlags
        0x0EA0 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Bkey_GetAllFlags");
        }
        // https://prizm.cemetech.net/index.php?title=Bkey_SetAllFlags
        0x0EA1 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Bkey_SetAllFlags");
        }
        // https://prizm.cemetech.net/index.php?title=Bkey_SetFlag
        0x0112 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Bkey_SetFlag");
        }
        // https://prizm.cemetech.net/index.php?title=Box
        0x092B => {
            debug.print("https://prizm.cemetech.net/index.php?title=Box");
        }
        // https://prizm.cemetech.net/index.php?title=Box2
        0x17FC => {
            debug.print("https://prizm.cemetech.net/index.php?title=Box2");
        }
        // https://prizm.cemetech.net/index.php?title=BoxInnerClear
        0x17FD => {
            debug.print("https://prizm.cemetech.net/index.php?title=BoxInnerClear");
        }
        // https://prizm.cemetech.net/index.php?title=BoxYLimits
        0x17FB => {
            debug.print("https://prizm.cemetech.net/index.php?title=BoxYLimits");
        }
        // https://prizm.cemetech.net/index.php?title=ByteToHex
        0x1347 => {
            debug.print("https://prizm.cemetech.net/index.php?title=ByteToHex");
        }
        // https://prizm.cemetech.net/index.php?title=CharacterSelectDialog
        0x07A2 => {
            debug.print("https://prizm.cemetech.net/index.php?title=CharacterSelectDialog");
        }
        // https://prizm.cemetech.net/index.php?title=CLIP_Store
        0x07E5 => {
            debug.print("https://prizm.cemetech.net/index.php?title=CLIP_Store");
        }
        // https://prizm.cemetech.net/index.php?title=CMT_Delay_100micros
        0x11D7 => {
            debug.print("https://prizm.cemetech.net/index.php?title=CMT_Delay_100micros");
        }
        // https://prizm.cemetech.net/index.php?title=CMT_Delay_micros
        0x11D6 => {
            debug.print("https://prizm.cemetech.net/index.php?title=CMT_Delay_micros");
        }
        // https://prizm.cemetech.net/index.php?title=ColorIndexDialog1
        0x1815 => {
            debug.print("https://prizm.cemetech.net/index.php?title=ColorIndexDialog1");
        }
        // https://prizm.cemetech.net/index.php?title=Comm_Close
        0x1354 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Comm_Close");
        }
        // https://prizm.cemetech.net/index.php?title=Comm_Open
        0x1353 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Comm_Open");
        }
        // https://prizm.cemetech.net/index.php?title=Comm_Terminate
        0x13F1 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Comm_Terminate");
        }
        // https://prizm.cemetech.net/index.php?title=Comm_TryCheckPacket
        0x1396 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Comm_TryCheckPacket");
        }
        // https://prizm.cemetech.net/index.php?title=ConfirmFileOverwriteDialog
        0x1802 => {
            debug.print("https://prizm.cemetech.net/index.php?title=ConfirmFileOverwriteDialog");
        }
        // https://prizm.cemetech.net/index.php?title=Cursor_GetSettings
        0x01F5 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Cursor_GetSettings");
        }
        // https://prizm.cemetech.net/index.php?title=Cursor_SetFlashOff
        0x08C8 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Cursor_SetFlashOff");
        }
        // https://prizm.cemetech.net/index.php?title=Cursor_SetFlashOn
        0x08C7 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Cursor_SetFlashOn");
        }
        // https://prizm.cemetech.net/index.php?title=Cursor_SetPosition
        0x01F1 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Cursor_SetPosition");
        }
        // https://prizm.cemetech.net/index.php?title=DefineStatusAreaFlags
        0x02B8 => {
            debug.print("https://prizm.cemetech.net/index.php?title=DefineStatusAreaFlags");
        }
        // https://prizm.cemetech.net/index.php?title=DefineStatusGlyph
        0x1D7B => {
            debug.print("https://prizm.cemetech.net/index.php?title=DefineStatusGlyph");
        }
        // https://prizm.cemetech.net/index.php?title=DefineStatusMessage
        0x1D77 => {
            debug.print("https://prizm.cemetech.net/index.php?title=DefineStatusMessage");
        }
        // https://prizm.cemetech.net/index.php?title=DirectDrawRectangle
        0x02AA => {
            debug.print("https://prizm.cemetech.net/index.php?title=DirectDrawRectangle");
        }
        // https://prizm.cemetech.net/index.php?title=DispInt
        0x0259 => {
            debug.print("https://prizm.cemetech.net/index.php?title=DispInt");
        }
        // https://prizm.cemetech.net/index.php?title=DisplayMainMenu
        0x1E6A => {
            debug.print("https://prizm.cemetech.net/index.php?title=DisplayMainMenu");
        }
        // https://prizm.cemetech.net/index.php?title=DisplayMBString
        0x121D => {
            debug.print("https://prizm.cemetech.net/index.php?title=DisplayMBString");
        }
        // https://prizm.cemetech.net/index.php?title=DisplayMBString2
        0x1218 => {
            debug.print("https://prizm.cemetech.net/index.php?title=DisplayMBString2");
        }
        // https://prizm.cemetech.net/index.php?title=DisplayMessageBox
        0x1E4B => {
            debug.print("https://prizm.cemetech.net/index.php?title=DisplayMessageBox");
        }
        // https://prizm.cemetech.net/index.php?title=DisplayStatusArea
        0x1D81 => {
            debug.print("https://prizm.cemetech.net/index.php?title=DisplayStatusArea");
        }
        // https://prizm.cemetech.net/index.php?title=DrawFrame
        0x02A8 => {
            debug.print("https://prizm.cemetech.net/index.php?title=DrawFrame");
        }
        // https://prizm.cemetech.net/index.php?title=DrawFrameWorkbench
        0x0923 => {
            debug.print("https://prizm.cemetech.net/index.php?title=DrawFrameWorkbench");
        }
        // https://prizm.cemetech.net/index.php?title=DrawHeaderLine
        0x02BB => {
            debug.print("https://prizm.cemetech.net/index.php?title=DrawHeaderLine");
        }
        // https://prizm.cemetech.net/index.php?title=D_c_Icon
        0x1D8E => {
            debug.print("https://prizm.cemetech.net/index.php?title=D_c_Icon");
        }
        // https://prizm.cemetech.net/index.php?title=EditMBStringChar
        0x1224 => {
            debug.print("https://prizm.cemetech.net/index.php?title=EditMBStringChar");
        }
        // https://prizm.cemetech.net/index.php?title=EditMBStringCtrl
        0x120E => {
            debug.print("https://prizm.cemetech.net/index.php?title=EditMBStringCtrl");
        }
        // https://prizm.cemetech.net/index.php?title=EditMBStringCtrl2
        0x120A => {
            debug.print("https://prizm.cemetech.net/index.php?title=EditMBStringCtrl2");
        }
        // https://prizm.cemetech.net/index.php?title=EditMBStringCtrl3
        0x120B => {
            debug.print("https://prizm.cemetech.net/index.php?title=EditMBStringCtrl3");
        }
        // https://prizm.cemetech.net/index.php?title=EditMBStringCtrl4
        0x120C => {
            debug.print("https://prizm.cemetech.net/index.php?title=EditMBStringCtrl4");
        }
        // https://prizm.cemetech.net/index.php?title=EnableDisplayHeader
        0x1D7F => {
            debug.print("https://prizm.cemetech.net/index.php?title=EnableDisplayHeader");
        }
        // https://prizm.cemetech.net/index.php?title=EnableStatusArea
        0x02B7 => {
            debug.print("https://prizm.cemetech.net/index.php?title=EnableStatusArea");
        }
        // https://prizm.cemetech.net/index.php?title=FKey_Display
        0x0131 => {
            debug.print("https://prizm.cemetech.net/index.php?title=FKey_Display");
        }
        // https://prizm.cemetech.net/index.php?title=FrameColor
        0x02A3 => {
            debug.print("https://prizm.cemetech.net/index.php?title=FrameColor");
        }
        // https://prizm.cemetech.net/index.php?title=GetAppName
        0x1E9F => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetAppName");
        }
        // https://prizm.cemetech.net/index.php?title=GetAutoPowerOffTime
        0x1E91 => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetAutoPowerOffTime");
        }
        // https://prizm.cemetech.net/index.php?title=GetBacklightDuration
        0x12D9 => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetBacklightDuration");
        }
        // https://prizm.cemetech.net/index.php?title=GetBatteryType
        0x12D5 => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetBatteryType");
        }
        // https://prizm.cemetech.net/index.php?title=GetFKeyPtr
        0x12F3 => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetFKeyPtr");
        }
        // https://prizm.cemetech.net/index.php?title=GetGetkeyToMainFunctionReturnFlag
        0x1E99 => {
            debug.print(
                "https://prizm.cemetech.net/index.php?title=GetGetkeyToMainFunctionReturnFlag",
            );
        }
        // https://prizm.cemetech.net/index.php?title=GetMainBatteryVoltage
        0x1186 => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetMainBatteryVoltage");
        }
        // https://prizm.cemetech.net/index.php?title=GetMiniGlyphPtr
        0x01E9 => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetMiniGlyphPtr");
        }
        // https://prizm.cemetech.net/index.php?title=GetSecondaryVramAddress
        0x1E50 => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetSecondaryVramAddress");
        }
        // https://prizm.cemetech.net/index.php?title=GetSetupSetting
        0x0031 => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetSetupSetting");
        }
        // https://prizm.cemetech.net/index.php?title=GetStackPtr
        0x1A2C => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetStackPtr");
        }
        // https://prizm.cemetech.net/index.php?title=GetVRAMAddress
        0x01E6 => {
            debug.print("https://prizm.cemetech.net/index.php?title=GetVRAMAddress");
        }
        // https://prizm.cemetech.net/index.php?title=GlibGetOSVersionInfo
        0x002B => {
            debug.print("https://prizm.cemetech.net/index.php?title=GlibGetOSVersionInfo");
        }
        // https://prizm.cemetech.net/index.php?title=HexToByte
        0x1344 => {
            debug.print("https://prizm.cemetech.net/index.php?title=HexToByte");
        }
        // https://prizm.cemetech.net/index.php?title=HexToNibble
        0x1343 => {
            debug.print("https://prizm.cemetech.net/index.php?title=HexToNibble");
        }
        // https://prizm.cemetech.net/index.php?title=HexToWord
        0x1345 => {
            debug.print("https://prizm.cemetech.net/index.php?title=HexToWord");
        }
        // https://prizm.cemetech.net/index.php?title=HourGlass
        0x02C7 => {
            debug.print("https://prizm.cemetech.net/index.php?title=HourGlass");
        }
        // https://prizm.cemetech.net/index.php?title=ItoA_10digit
        0x1633 => {
            debug.print("https://prizm.cemetech.net/index.php?title=ItoA_10digit");
        }
        // https://prizm.cemetech.net/index.php?title=KeyboardIcon
        0x1D8A => {
            debug.print("https://prizm.cemetech.net/index.php?title=KeyboardIcon");
        }
        // https://prizm.cemetech.net/index.php?title=Keyboard_CursorFlash
        0x08CA => {
            debug.print("https://prizm.cemetech.net/index.php?title=Keyboard_CursorFlash");
        }
        // https://prizm.cemetech.net/index.php?title=Keyboard_PutKeycode
        0x12C6 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Keyboard_PutKeycode");
        }
        // https://prizm.cemetech.net/index.php?title=Keyboard_SpyMatrixCode
        0x12C9 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Keyboard_SpyMatrixCode");
        }
        // https://prizm.cemetech.net/index.php?title=LineIcon
        0x1D8F => {
            debug.print("https://prizm.cemetech.net/index.php?title=LineIcon");
        }
        // https://prizm.cemetech.net/index.php?title=LoadVRAM_1
        0x1E63 => {
            debug.print("https://prizm.cemetech.net/index.php?title=LoadVRAM_1");
        }
        // https://prizm.cemetech.net/index.php?title=LocalizeMessage1
        0x12FC => {
            debug.print("https://prizm.cemetech.net/index.php?title=LocalizeMessage1");
        }
        // https://prizm.cemetech.net/index.php?title=Locate_OS
        0x1863 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Locate_OS");
        }
        // https://prizm.cemetech.net/index.php?title=LongToAscHex
        0x1841 => {
            debug.print("https://prizm.cemetech.net/index.php?title=LongToAscHex");
        }
        // https://prizm.cemetech.net/index.php?title=MB_ElementCount
        0x1163 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MB_ElementCount");
        }
        // https://prizm.cemetech.net/index.php?title=MCSDelVar2
        0x1558 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCSDelVar2");
        }
        // https://prizm.cemetech.net/index.php?title=MCSGetData1
        0x1563 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCSGetData1");
        }
        // https://prizm.cemetech.net/index.php?title=MCSGetDlen2
        0x1562 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCSGetDlen2");
        }
        // https://prizm.cemetech.net/index.php?title=MCSGetOpenItem
        0x1560 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCSGetOpenItem");
        }
        // https://prizm.cemetech.net/index.php?title=MCSOvwDat2
        0x1552 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCSOvwDat2");
        }
        // https://prizm.cemetech.net/index.php?title=MCSPutVar2
        0x154D => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCSPutVar2");
        }
        // https://prizm.cemetech.net/index.php?title=MCS_CreateDirectory
        0x154B => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCS_CreateDirectory");
        }
        // https://prizm.cemetech.net/index.php?title=MCS_DeleteDirectory
        0x1516 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCS_DeleteDirectory");
        }
        // https://prizm.cemetech.net/index.php?title=MCS_GetCapa
        0x1532 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCS_GetCapa");
        }
        // https://prizm.cemetech.net/index.php?title=MCS_GetMainMemoryStart
        0x1543 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCS_GetMainMemoryStart");
        }
        // https://prizm.cemetech.net/index.php?title=MCS_GetState
        0x1529 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCS_GetState");
        }
        // https://prizm.cemetech.net/index.php?title=MCS_WriteItem
        0x151A => {
            debug.print("https://prizm.cemetech.net/index.php?title=MCS_WriteItem");
        }
        // https://prizm.cemetech.net/index.php?title=MsgBoxMoveWB
        0x0938 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MsgBoxMoveWB");
        }
        // https://prizm.cemetech.net/index.php?title=MsgBoxPop
        0x17F9 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MsgBoxPop");
        }
        // https://prizm.cemetech.net/index.php?title=MsgBoxPush
        0x17F7 => {
            debug.print("https://prizm.cemetech.net/index.php?title=MsgBoxPush");
        }
        // https://prizm.cemetech.net/index.php?title=NibbleToHex
        0x1346 => {
            debug.print("https://prizm.cemetech.net/index.php?title=NibbleToHex");
        }
        // https://prizm.cemetech.net/index.php?title=NormIcon
        0x1D8D => {
            debug.print("https://prizm.cemetech.net/index.php?title=NormIcon");
        }
        // https://prizm.cemetech.net/index.php?title=OpenFileDialog
        0x17E9 => {
            debug.print("https://prizm.cemetech.net/index.php?title=OpenFileDialog");
        }
        // https://prizm.cemetech.net/index.php?title=OS_InnerWait_ms
        0x1BB4 => {
            debug.print("https://prizm.cemetech.net/index.php?title=OS_InnerWait_ms");
        }
        // https://prizm.cemetech.net/index.php?title=OverwriteConfirmation
        0x0D91 => {
            debug.print("https://prizm.cemetech.net/index.php?title=OverwriteConfirmation");
        }
        // https://prizm.cemetech.net/index.php?title=PowerOff
        0x1839 => {
            debug.print("https://prizm.cemetech.net/index.php?title=PowerOff");
        }
        // https://prizm.cemetech.net/index.php?title=PrintCXY
        0x0239 => {
            debug.print("https://prizm.cemetech.net/index.php?title=PrintCXY");
        }
        // https://prizm.cemetech.net/index.php?title=PrintGlyph
        0x0238 => {
            debug.print("https://prizm.cemetech.net/index.php?title=PrintGlyph");
        }
        // https://prizm.cemetech.net/index.php?title=PrintLine
        0x1883 => {
            debug.print("https://prizm.cemetech.net/index.php?title=PrintLine");
        }
        // https://prizm.cemetech.net/index.php?title=PrintLine2
        0x1865 => {
            debug.print("https://prizm.cemetech.net/index.php?title=PrintLine2");
        }
        // https://prizm.cemetech.net/index.php?title=PrintMini
        0x023C => {
            debug.print("https://prizm.cemetech.net/index.php?title=PrintMini");
        }
        // https://prizm.cemetech.net/index.php?title=PrintMiniGlyph
        0x023B => {
            debug.print("https://prizm.cemetech.net/index.php?title=PrintMiniGlyph");
        }
        // https://prizm.cemetech.net/index.php?title=PrintMiniMini
        0x021B => {
            debug.print("https://prizm.cemetech.net/index.php?title=PrintMiniMini");
        }
        // https://prizm.cemetech.net/index.php?title=PrintXY
        0x18F9 => {
            debug.print("https://prizm.cemetech.net/index.php?title=PrintXY");
        }
        // https://prizm.cemetech.net/index.php?title=PrintXY_2
        0x18EC => {
            debug.print("https://prizm.cemetech.net/index.php?title=PrintXY_2");
        }
        // https://prizm.cemetech.net/index.php?title=Print_OS
        0x01F9 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Print_OS");
        }
        // https://prizm.cemetech.net/index.php?title=ProgressBar
        0x180E => {
            debug.print("https://prizm.cemetech.net/index.php?title=ProgressBar");
        }
        // https://prizm.cemetech.net/index.php?title=ProgressBar0
        0x180B => {
            debug.print("https://prizm.cemetech.net/index.php?title=ProgressBar0");
        }
        // https://prizm.cemetech.net/index.php?title=ProgressBar2
        0x1809 => {
            debug.print("https://prizm.cemetech.net/index.php?title=ProgressBar2");
        }
        // https://prizm.cemetech.net/index.php?title=RadIcon
        0x1D8B => {
            debug.print("https://prizm.cemetech.net/index.php?title=RadIcon");
        }
        // https://prizm.cemetech.net/index.php?title=RealIcon
        0x1D8C => {
            debug.print("https://prizm.cemetech.net/index.php?title=RealIcon");
        }
        // https://prizm.cemetech.net/index.php?title=ResetAllDialog
        0x1E23 => {
            debug.print("https://prizm.cemetech.net/index.php?title=ResetAllDialog");
        }
        // https://prizm.cemetech.net/index.php?title=Restart
        0x1187 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Restart");
        }
        // https://prizm.cemetech.net/index.php?title=SaveFileDialog
        0x0C66 => {
            debug.print("https://prizm.cemetech.net/index.php?title=SaveFileDialog");
        }
        // https://prizm.cemetech.net/index.php?title=SaveVRAM_1
        0x1E62 => {
            debug.print("https://prizm.cemetech.net/index.php?title=SaveVRAM_1");
        }
        // https://prizm.cemetech.net/index.php?title=Scrollbar
        0x1A0A => {
            debug.print("https://prizm.cemetech.net/index.php?title=Scrollbar");
        }
        // https://prizm.cemetech.net/index.php?title=SetAutoPowerOffTime
        0x1E90 => {
            debug.print("https://prizm.cemetech.net/index.php?title=SetAutoPowerOffTime");
        }
        // https://prizm.cemetech.net/index.php?title=SetBackGround
        0x1EF8 => {
            debug.print("https://prizm.cemetech.net/index.php?title=SetBackGround");
        }
        // https://prizm.cemetech.net/index.php?title=SetBacklightDuration
        0x12D8 => {
            debug.print("https://prizm.cemetech.net/index.php?title=SetBacklightDuration");
        }
        // https://prizm.cemetech.net/index.php?title=SetBatteryType
        0x12D4 => {
            debug.print("https://prizm.cemetech.net/index.php?title=SetBatteryType");
        }
        // https://prizm.cemetech.net/index.php?title=SetCursorFlashToggle
        0x08D2 => {
            debug.print("https://prizm.cemetech.net/index.php?title=SetCursorFlashToggle");
        }
        // https://prizm.cemetech.net/index.php?title=SetQuitHandler
        0x1E6E => {
            debug.print("https://prizm.cemetech.net/index.php?title=SetQuitHandler");
        }
        // https://prizm.cemetech.net/index.php?title=SetSetupSetting
        0x0032 => {
            debug.print("https://prizm.cemetech.net/index.php?title=SetSetupSetting");
        }
        // https://prizm.cemetech.net/index.php?title=SetupMode_StatusIcon
        0x0A8B => {
            debug.print("https://prizm.cemetech.net/index.php?title=SetupMode_StatusIcon");
        }
        // https://prizm.cemetech.net/index.php?title=Set_FKeys1
        0x012B => {
            debug.print("https://prizm.cemetech.net/index.php?title=Set_FKeys1");
        }
        // https://prizm.cemetech.net/index.php?title=Set_FKeys2
        0x0129 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Set_FKeys2");
        }
        // https://prizm.cemetech.net/index.php?title=SMEM_FindFirst
        0x0DAC => {
            debug.print("https://prizm.cemetech.net/index.php?title=SMEM_FindFirst");
        }
        // https://prizm.cemetech.net/index.php?title=SMEM_MapIconToExt
        0x0C2C => {
            debug.print("https://prizm.cemetech.net/index.php?title=SMEM_MapIconToExt");
        }
        // https://prizm.cemetech.net/index.php?title=SpecialMatrixcodeProcessing
        0x1E60 => {
            debug.print("https://prizm.cemetech.net/index.php?title=SpecialMatrixcodeProcessing");
        }
        // https://prizm.cemetech.net/index.php?title=StandardScrollbar
        0x0C5A => {
            debug.print("https://prizm.cemetech.net/index.php?title=StandardScrollbar");
        }
        // https://prizm.cemetech.net/index.php?title=TakeScreenshot
        0x17E6 => {
            debug.print("https://prizm.cemetech.net/index.php?title=TakeScreenshot");
        }
        // https://prizm.cemetech.net/index.php?title=TakeScreenshot2
        0x17E7 => {
            debug.print("https://prizm.cemetech.net/index.php?title=TakeScreenshot2");
        }
        // https://prizm.cemetech.net/index.php?title=TestMode
        0x0EA7 => {
            debug.print("https://prizm.cemetech.net/index.php?title=TestMode");
        }
        // https://prizm.cemetech.net/index.php?title=WordToHex
        0x1348 => {
            debug.print("https://prizm.cemetech.net/index.php?title=WordToHex");
        }
        // https://prizm.cemetech.net/index.php?title=WriteBackground
        0x1EF7 => {
            debug.print("https://prizm.cemetech.net/index.php?title=WriteBackground");
        }

        GETKEY => {
            let (key, modifier) = input.get_key().await;
            let key_code = key.serialize(&modifier);

            memory.write_long(param_1, key_code as u32);
        }
        GETKEYWAIT_OS => {
            debug.print("GetKeyWait_OS");
        }
        PRGM_GETKEY_OS => {
            debug.print("PRGM_GetKey_OS");
        }
        ITOA => {
            let result_str = param_1.to_string();
            let result_bytes = result_str.as_bytes();

            debug.print("Itoa Syscall");
            for (index, tmp_byte) in result_bytes.iter().enumerate() {
                let addr = param_2 + (index as u32);
                memory.write_byte(addr, *tmp_byte);
            }
        }
        _ if disp::is_syscall(id) => {
            disp::handle_syscall(
                id, param_1, param_2, param_3, param_4, display, memory, debug,
            );
        }
        _ if fs::is_syscall(id) => {
            fs::handle_syscall(id, param_1, param_2, param_3, param_4, memory);
        }
        _ if app::is_syscall(id) => {
            app::handle_syscall(id, debug);
        }
        _ if timer::is_syscall(id) => {
            timer::handle_syscall(id, debug);
        }
        _ if sys::is_syscall(id) => {
            sys::handle_syscall(id, debug);
        }
        _ if serial::is_syscall(id) => {
            serial::handle_syscall(id, debug);
        }
        _ if rtc::is_syscall(id) => {
            rtc::handle_syscall(id, debug);
        }
        _ => {
            debug.print(&format!("Unknown Syscall: x{:04X}", id));
        }
    };
}

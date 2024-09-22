#[doc = "DFE packet EasyDMA channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfepacket {
    ptr: *mut u8,
}
unsafe impl Send for Dfepacket {}
unsafe impl Sync for Dfepacket {}
impl Dfepacket {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data pointer"]
    #[inline(always)]
    pub const fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Maximum number of buffer words to transfer"]
    #[inline(always)]
    pub const fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Number of samples transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(self) -> crate::common::Reg<regs::Amount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel {
    ptr: *mut u8,
}
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description collection: Pin select for DFE pin n"]
    #[inline(always)]
    pub const fn dfegpio(self, n: usize) -> crate::common::Reg<regs::Dfegpio, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "2.4 GHz radio"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RadioNs {
    ptr: *mut u8,
}
unsafe impl Send for RadioNs {}
unsafe impl Sync for RadioNs {}
impl RadioNs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Enable RADIO in TX mode"]
    #[inline(always)]
    pub const fn tasks_txen(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Enable RADIO in RX mode"]
    #[inline(always)]
    pub const fn tasks_rxen(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Start RADIO"]
    #[inline(always)]
    pub const fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Stop RADIO"]
    #[inline(always)]
    pub const fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Disable RADIO"]
    #[inline(always)]
    pub const fn tasks_disable(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Start the RSSI and take one single sample of the receive signal strength"]
    #[inline(always)]
    pub const fn tasks_rssistart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Stop the RSSI measurement"]
    #[inline(always)]
    pub const fn tasks_rssistop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Start the bit counter"]
    #[inline(always)]
    pub const fn tasks_bcstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Stop the bit counter"]
    #[inline(always)]
    pub const fn tasks_bcstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub const fn tasks_edstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Stop the energy detect measurement"]
    #[inline(always)]
    pub const fn tasks_edstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub const fn tasks_ccastart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Stop the clear channel assessment"]
    #[inline(always)]
    pub const fn tasks_ccastop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Subscribe configuration for task TXEN"]
    #[inline(always)]
    pub const fn subscribe_txen(
        self,
    ) -> crate::common::Reg<regs::SubscribeTxen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Subscribe configuration for task RXEN"]
    #[inline(always)]
    pub const fn subscribe_rxen(
        self,
    ) -> crate::common::Reg<regs::SubscribeRxen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn subscribe_start(
        self,
    ) -> crate::common::Reg<regs::SubscribeStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(
        self,
    ) -> crate::common::Reg<regs::SubscribeStop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Subscribe configuration for task DISABLE"]
    #[inline(always)]
    pub const fn subscribe_disable(
        self,
    ) -> crate::common::Reg<regs::SubscribeDisable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Subscribe configuration for task RSSISTART"]
    #[inline(always)]
    pub const fn subscribe_rssistart(
        self,
    ) -> crate::common::Reg<regs::SubscribeRssistart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Subscribe configuration for task RSSISTOP"]
    #[inline(always)]
    pub const fn subscribe_rssistop(
        self,
    ) -> crate::common::Reg<regs::SubscribeRssistop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Subscribe configuration for task BCSTART"]
    #[inline(always)]
    pub const fn subscribe_bcstart(
        self,
    ) -> crate::common::Reg<regs::SubscribeBcstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Subscribe configuration for task BCSTOP"]
    #[inline(always)]
    pub const fn subscribe_bcstop(
        self,
    ) -> crate::common::Reg<regs::SubscribeBcstop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Subscribe configuration for task EDSTART"]
    #[inline(always)]
    pub const fn subscribe_edstart(
        self,
    ) -> crate::common::Reg<regs::SubscribeEdstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Subscribe configuration for task EDSTOP"]
    #[inline(always)]
    pub const fn subscribe_edstop(
        self,
    ) -> crate::common::Reg<regs::SubscribeEdstop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "Subscribe configuration for task CCASTART"]
    #[inline(always)]
    pub const fn subscribe_ccastart(
        self,
    ) -> crate::common::Reg<regs::SubscribeCcastart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Subscribe configuration for task CCASTOP"]
    #[inline(always)]
    pub const fn subscribe_ccastop(
        self,
    ) -> crate::common::Reg<regs::SubscribeCcastop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "RADIO has ramped up and is ready to be started"]
    #[inline(always)]
    pub const fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Address sent or received"]
    #[inline(always)]
    pub const fn events_address(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Packet payload sent or received"]
    #[inline(always)]
    pub const fn events_payload(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Packet sent or received"]
    #[inline(always)]
    pub const fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "RADIO has been disabled"]
    #[inline(always)]
    pub const fn events_disabled(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "A device address match occurred on the last received packet"]
    #[inline(always)]
    pub const fn events_devmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "No device address match occurred on the last received packet"]
    #[inline(always)]
    pub const fn events_devmiss(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "Sampling of receive signal strength complete"]
    #[inline(always)]
    pub const fn events_rssiend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "Bit counter reached bit count value"]
    #[inline(always)]
    pub const fn events_bcmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "Packet received with CRC ok"]
    #[inline(always)]
    pub const fn events_crcok(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "Packet received with CRC error"]
    #[inline(always)]
    pub const fn events_crcerror(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "IEEE 802.15.4 length field received"]
    #[inline(always)]
    pub const fn events_framestart(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
    #[inline(always)]
    pub const fn events_edend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "The sampling of energy detection has stopped"]
    #[inline(always)]
    pub const fn events_edstopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "Wireless medium in idle - clear to send"]
    #[inline(always)]
    pub const fn events_ccaidle(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "Wireless medium busy - do not send"]
    #[inline(always)]
    pub const fn events_ccabusy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "The CCA has stopped"]
    #[inline(always)]
    pub const fn events_ccastopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
    #[inline(always)]
    pub const fn events_rateboost(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "RADIO has ramped up and is ready to be started TX path"]
    #[inline(always)]
    pub const fn events_txready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "RADIO has ramped up and is ready to be started RX path"]
    #[inline(always)]
    pub const fn events_rxready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "MAC header match found"]
    #[inline(always)]
    pub const fn events_mhrmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "Preamble indicator"]
    #[inline(always)]
    pub const fn events_sync(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "Generated when last bit is sent on air, or received from air"]
    #[inline(always)]
    pub const fn events_phyend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "CTE is present (early warning right after receiving CTEInfo byte)"]
    #[inline(always)]
    pub const fn events_ctepresent(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "Publish configuration for event READY"]
    #[inline(always)]
    pub const fn publish_ready(self) -> crate::common::Reg<regs::PublishReady, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Publish configuration for event ADDRESS"]
    #[inline(always)]
    pub const fn publish_address(
        self,
    ) -> crate::common::Reg<regs::PublishAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Publish configuration for event PAYLOAD"]
    #[inline(always)]
    pub const fn publish_payload(
        self,
    ) -> crate::common::Reg<regs::PublishPayload, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Publish configuration for event END"]
    #[inline(always)]
    pub const fn publish_end(self) -> crate::common::Reg<regs::PublishEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "Publish configuration for event DISABLED"]
    #[inline(always)]
    pub const fn publish_disabled(
        self,
    ) -> crate::common::Reg<regs::PublishDisabled, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "Publish configuration for event DEVMATCH"]
    #[inline(always)]
    pub const fn publish_devmatch(
        self,
    ) -> crate::common::Reg<regs::PublishDevmatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "Publish configuration for event DEVMISS"]
    #[inline(always)]
    pub const fn publish_devmiss(
        self,
    ) -> crate::common::Reg<regs::PublishDevmiss, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[doc = "Publish configuration for event RSSIEND"]
    #[inline(always)]
    pub const fn publish_rssiend(
        self,
    ) -> crate::common::Reg<regs::PublishRssiend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
    }
    #[doc = "Publish configuration for event BCMATCH"]
    #[inline(always)]
    pub const fn publish_bcmatch(
        self,
    ) -> crate::common::Reg<regs::PublishBcmatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[doc = "Publish configuration for event CRCOK"]
    #[inline(always)]
    pub const fn publish_crcok(self) -> crate::common::Reg<regs::PublishCrcok, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[doc = "Publish configuration for event CRCERROR"]
    #[inline(always)]
    pub const fn publish_crcerror(
        self,
    ) -> crate::common::Reg<regs::PublishCrcerror, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[doc = "Publish configuration for event FRAMESTART"]
    #[inline(always)]
    pub const fn publish_framestart(
        self,
    ) -> crate::common::Reg<regs::PublishFramestart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[doc = "Publish configuration for event EDEND"]
    #[inline(always)]
    pub const fn publish_edend(self) -> crate::common::Reg<regs::PublishEdend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[doc = "Publish configuration for event EDSTOPPED"]
    #[inline(always)]
    pub const fn publish_edstopped(
        self,
    ) -> crate::common::Reg<regs::PublishEdstopped, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "Publish configuration for event CCAIDLE"]
    #[inline(always)]
    pub const fn publish_ccaidle(
        self,
    ) -> crate::common::Reg<regs::PublishCcaidle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[doc = "Publish configuration for event CCABUSY"]
    #[inline(always)]
    pub const fn publish_ccabusy(
        self,
    ) -> crate::common::Reg<regs::PublishCcabusy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[doc = "Publish configuration for event CCASTOPPED"]
    #[inline(always)]
    pub const fn publish_ccastopped(
        self,
    ) -> crate::common::Reg<regs::PublishCcastopped, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[doc = "Publish configuration for event RATEBOOST"]
    #[inline(always)]
    pub const fn publish_rateboost(
        self,
    ) -> crate::common::Reg<regs::PublishRateboost, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[doc = "Publish configuration for event TXREADY"]
    #[inline(always)]
    pub const fn publish_txready(
        self,
    ) -> crate::common::Reg<regs::PublishTxready, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d4usize) as _) }
    }
    #[doc = "Publish configuration for event RXREADY"]
    #[inline(always)]
    pub const fn publish_rxready(
        self,
    ) -> crate::common::Reg<regs::PublishRxready, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d8usize) as _) }
    }
    #[doc = "Publish configuration for event MHRMATCH"]
    #[inline(always)]
    pub const fn publish_mhrmatch(
        self,
    ) -> crate::common::Reg<regs::PublishMhrmatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01dcusize) as _) }
    }
    #[doc = "Publish configuration for event SYNC"]
    #[inline(always)]
    pub const fn publish_sync(self) -> crate::common::Reg<regs::PublishSync, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "Publish configuration for event PHYEND"]
    #[inline(always)]
    pub const fn publish_phyend(
        self,
    ) -> crate::common::Reg<regs::PublishPhyend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "Publish configuration for event CTEPRESENT"]
    #[inline(always)]
    pub const fn publish_ctepresent(
        self,
    ) -> crate::common::Reg<regs::PublishCtepresent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[doc = "Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "CRC status"]
    #[inline(always)]
    pub const fn crcstatus(self) -> crate::common::Reg<regs::Crcstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Received address"]
    #[inline(always)]
    pub const fn rxmatch(self) -> crate::common::Reg<regs::Rxmatch, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "CRC field of previously received packet"]
    #[inline(always)]
    pub const fn rxcrc(self) -> crate::common::Reg<regs::Rxcrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "Device address match index"]
    #[inline(always)]
    pub const fn dai(self) -> crate::common::Reg<regs::Dai, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "Payload status"]
    #[inline(always)]
    pub const fn pdustat(self) -> crate::common::Reg<regs::Pdustat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "CTEInfo parsed from received packet"]
    #[inline(always)]
    pub const fn ctestatus(self) -> crate::common::Reg<regs::Ctestatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x044cusize) as _) }
    }
    #[doc = "DFE status information"]
    #[inline(always)]
    pub const fn dfestatus(self) -> crate::common::Reg<regs::Dfestatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0458usize) as _) }
    }
    #[doc = "Packet pointer"]
    #[inline(always)]
    pub const fn packetptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "Frequency"]
    #[inline(always)]
    pub const fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "Output power"]
    #[inline(always)]
    pub const fn txpower(self) -> crate::common::Reg<regs::Txpower, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "Data rate and modulation"]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Packet configuration register 0"]
    #[inline(always)]
    pub const fn pcnf0(self) -> crate::common::Reg<regs::Pcnf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "Packet configuration register 1"]
    #[inline(always)]
    pub const fn pcnf1(self) -> crate::common::Reg<regs::Pcnf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "Base address 0"]
    #[inline(always)]
    pub const fn base0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "Base address 1"]
    #[inline(always)]
    pub const fn base1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "Prefixes bytes for logical addresses 0-3"]
    #[inline(always)]
    pub const fn prefix0(self) -> crate::common::Reg<regs::Prefix0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[doc = "Prefixes bytes for logical addresses 4-7"]
    #[inline(always)]
    pub const fn prefix1(self) -> crate::common::Reg<regs::Prefix1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[doc = "Transmit address select"]
    #[inline(always)]
    pub const fn txaddress(self) -> crate::common::Reg<regs::Txaddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[doc = "Receive address select"]
    #[inline(always)]
    pub const fn rxaddresses(self) -> crate::common::Reg<regs::Rxaddresses, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[doc = "CRC configuration"]
    #[inline(always)]
    pub const fn crccnf(self) -> crate::common::Reg<regs::Crccnf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
    }
    #[doc = "CRC polynomial"]
    #[inline(always)]
    pub const fn crcpoly(self) -> crate::common::Reg<regs::Crcpoly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
    }
    #[doc = "CRC initial value"]
    #[inline(always)]
    pub const fn crcinit(self) -> crate::common::Reg<regs::Crcinit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x053cusize) as _) }
    }
    #[doc = "Interframe spacing in us"]
    #[inline(always)]
    pub const fn tifs(self) -> crate::common::Reg<regs::Tifs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
    }
    #[doc = "RSSI sample"]
    #[inline(always)]
    pub const fn rssisample(self) -> crate::common::Reg<regs::Rssisample, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[doc = "Current radio state"]
    #[inline(always)]
    pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[doc = "Data whitening initial value"]
    #[inline(always)]
    pub const fn datawhiteiv(self) -> crate::common::Reg<regs::Datawhiteiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
    }
    #[doc = "Bit counter compare"]
    #[inline(always)]
    pub const fn bcc(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[doc = "Description collection: Device address base segment n"]
    #[inline(always)]
    pub const fn dab(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
    }
    #[doc = "Description collection: Device address prefix n"]
    #[inline(always)]
    pub const fn dap(self, n: usize) -> crate::common::Reg<regs::Dap, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize + n * 4usize) as _) }
    }
    #[doc = "Device address match configuration"]
    #[inline(always)]
    pub const fn dacnf(self) -> crate::common::Reg<regs::Dacnf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[doc = "Search pattern configuration"]
    #[inline(always)]
    pub const fn mhrmatchconf(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0644usize) as _) }
    }
    #[doc = "Pattern mask"]
    #[inline(always)]
    pub const fn mhrmatchmas(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0648usize) as _) }
    }
    #[doc = "Radio mode configuration register 0"]
    #[inline(always)]
    pub const fn modecnf0(self) -> crate::common::Reg<regs::Modecnf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0650usize) as _) }
    }
    #[doc = "IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub const fn sfd(self) -> crate::common::Reg<regs::Sfd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0660usize) as _) }
    }
    #[doc = "IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub const fn edcnt(self) -> crate::common::Reg<regs::Edcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0664usize) as _) }
    }
    #[doc = "IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub const fn edsample(self) -> crate::common::Reg<regs::Edsample, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0668usize) as _) }
    }
    #[doc = "IEEE 802.15.4 clear channel assessment control"]
    #[inline(always)]
    pub const fn ccactrl(self) -> crate::common::Reg<regs::Ccactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x066cusize) as _) }
    }
    #[doc = "Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
    #[inline(always)]
    pub const fn dfemode(self) -> crate::common::Reg<regs::Dfemode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
    #[doc = "Configuration for CTE inline mode"]
    #[inline(always)]
    pub const fn cteinlineconf(self) -> crate::common::Reg<regs::Cteinlineconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0904usize) as _) }
    }
    #[doc = "Various configuration for Direction finding"]
    #[inline(always)]
    pub const fn dfectrl1(self) -> crate::common::Reg<regs::Dfectrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0910usize) as _) }
    }
    #[doc = "Start offset for Direction finding"]
    #[inline(always)]
    pub const fn dfectrl2(self) -> crate::common::Reg<regs::Dfectrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0914usize) as _) }
    }
    #[doc = "GPIO patterns to be used for each antenna"]
    #[inline(always)]
    pub const fn switchpattern(self) -> crate::common::Reg<regs::Switchpattern, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0928usize) as _) }
    }
    #[doc = "Clear the GPIO pattern array for antenna control"]
    #[inline(always)]
    pub const fn clearpattern(self) -> crate::common::Reg<regs::Clearpattern, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x092cusize) as _) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub const fn psel(self) -> Psel {
        unsafe { Psel::from_ptr(self.ptr.add(0x0930usize) as _) }
    }
    #[doc = "DFE packet EasyDMA channel"]
    #[inline(always)]
    pub const fn dfepacket(self) -> Dfepacket {
        unsafe { Dfepacket::from_ptr(self.ptr.add(0x0950usize) as _) }
    }
    #[doc = "Peripheral power control"]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;

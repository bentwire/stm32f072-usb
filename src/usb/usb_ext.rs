use hal::stm32::usb::{EP0R, EP1R};

pub trait UsbEpExt {
    fn toggle_tx_out(&self);
    fn toggle_tx_stall(&self);
    fn toggle_out(&self);
    fn toggle_0(&self);
    fn toggle_rx(&self);
    fn toggle_tx(&self);
    fn toggle(&self, mask: u32, val: u32, flags: u32);
    fn clear_ctr_rx(&self);
    fn clear_ctr_tx(&self);
}

const EP_MASK: u32 = 0x0F0F;
const EP_TX_MASK: u32 = 0x0030;
const EP_RX_MASK: u32 = 0x3000;
const EP_TX_RX_MASK: u32 = (EP_TX_MASK | EP_RX_MASK);

const EP_TX_VALID: u32 = 0x0030;
const EP_RX_VALID: u32 = 0x3000;
const EP_TX_RX_VALID: u32 = (EP_TX_VALID | EP_RX_VALID);

const EP_TX_STALL: u32 = 0x0010;
const EP_STATUS_OUT: u32 = 0x0100;

const EP_CTR_RX: u32 = 0x8000;
const EP_CTR_TX: u32 = 0x0080;

const EP_DTOG_RX: u32 = 0x4000;
const EP_DTOG_TX: u32 = 0x0040;

impl UsbEpExt for EP0R {
    fn toggle_tx_stall(&self) {
        self.toggle(EP_TX_RX_MASK, EP_RX_VALID | EP_TX_STALL, 0)
    }

    fn toggle_tx_out(&self) {
        self.toggle(EP_TX_MASK, EP_TX_VALID, EP_STATUS_OUT)
    }

    fn toggle_out(&self) {
        self.toggle(EP_TX_RX_MASK, EP_TX_RX_VALID, EP_STATUS_OUT)
    }

    fn toggle_0(&self) {
        self.toggle(EP_TX_RX_MASK, EP_TX_RX_VALID, 0)
    }

    fn toggle_rx(&self) {
        self.toggle(EP_DTOG_RX, EP_DTOG_RX, 0)
    }

    fn toggle_tx(&self) {
        self.toggle(EP_DTOG_TX, EP_DTOG_TX, 0)
    }

    fn toggle(&self, mask: u32, val: u32, flags: u32) {
        self.modify(|r, w| unsafe { w.bits(((r.bits() & (EP_MASK | mask)) ^ val) | flags) })
    }

    fn clear_ctr_rx(&self) {
        self.modify(|r, w| unsafe { w.bits((r.bits() & EP_MASK) | EP_CTR_TX) });
    }

    fn clear_ctr_tx(&self) {
        self.modify(|r, w| unsafe { w.bits((r.bits() & EP_MASK) | EP_CTR_RX) });
    }
}

impl UsbEpExt for EP1R {
    fn toggle_tx_stall(&self) {
        self.toggle(EP_TX_RX_MASK, EP_RX_VALID | EP_TX_STALL, 0)
    }

    fn toggle_tx_out(&self) {
        self.toggle(EP_TX_MASK, EP_TX_VALID, 0)
    }

    fn toggle_out(&self) {
        self.toggle(EP_TX_RX_MASK, EP_TX_RX_VALID, EP_STATUS_OUT)
    }

    fn toggle_0(&self) {
        self.toggle(EP_TX_RX_MASK, EP_TX_RX_VALID, 0)
    }

    fn toggle_rx(&self) {
        self.toggle(EP_DTOG_RX, EP_DTOG_RX, 0)
    }

    fn toggle_tx(&self) {
        self.toggle(EP_DTOG_TX, EP_DTOG_TX, 0)
    }

    fn toggle(&self, mask: u32, val: u32, flags: u32) {
        self.modify(|r, w| unsafe { w.bits(((r.bits() & (EP_MASK | mask)) ^ val) | flags) })
    }

    fn clear_ctr_rx(&self) {
        self.modify(|r, w| unsafe { w.bits((r.bits() & EP_MASK) | EP_CTR_TX) });
    }

    fn clear_ctr_tx(&self) {
        self.modify(|r, w| unsafe { w.bits((r.bits() & EP_MASK) | EP_CTR_RX) });
    }
}

use crate::{
    device::{adb_message_device::ADBMessageDevice, MessageCommand},
    ADBMessageTransport, RebootType, Result,
};

impl<T: ADBMessageTransport> ADBMessageDevice<T> {
    pub(crate) fn reboot(&mut self, reboot_type: RebootType) -> Result<()> {
        self.open_session(format!("reboot:{}\0", reboot_type).as_bytes())?;

        self.get_transport_mut()
            .read_message()
            .and_then(|message| message.assert_command(MessageCommand::Okay))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Connector {
    handle: *const crate::ffi::DrmModeConnector,
    connector_id: libc::c_uint,
    encoder_id: libc::c_uint,
    connector_type: crate::ffi::enums::ConnectorType,
    connector_type_id: libc::c_uint,
    connection_status: crate::ffi::enums::ConnectionStatus,

    mm_width: libc::c_uint,
    mm_height: libc::c_uint,
    
    subpixel: crate::ffi::enums::SubPixel,

    pub(crate) modes: Vec<crate::core::ModeInfo>,
}

impl Connector {
    pub fn new(c: &crate::ffi::DrmModeConnector) -> Self {
        Self {
            handle: c as *const crate::ffi::DrmModeConnector,
            connector_id : c.connector_id,
            encoder_id : c.encoder_id,
            connector_type : c.connector_type,
            connector_type_id : c.connector_type_id,
            connection_status : c.connection,
            mm_width : c.mm_width,
            mm_height : c.mm_height,
            subpixel : c.subpixel,
            
            modes : get_modes(c)
        }
    }

    pub fn get_id(&self) -> libc::c_uint {
        self.connector_id
    }

    pub fn get_connection_status(&self) -> crate::ffi::enums::ConnectionStatus {
        self.connection_status
    }

    pub fn get_encoder_id(&self) -> libc::c_uint {
        self.encoder_id
    }

}

impl Drop for Connector {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::drmModeFreeConnector(self.handle);
            colored_rs::print_debug!("Connector: {:?} droped", self.handle);
        }
    }
}

fn get_modes(c: &crate::ffi::DrmModeConnector) -> Vec<crate::core::ModeInfo> {
    unsafe {std::slice::from_raw_parts(c.modes, c.count_modes as usize)}.iter().map(|x| {
        crate::core::ModeInfo::new(x)
    }).collect::<Vec<_>>()
}